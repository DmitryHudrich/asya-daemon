use libloading::Library;
use log::*;
use plugin_interface::{EventState, PluginInformation, State};
use serde::Serialize;
use std::{
    ffi::{CStr, CString},
    fs, io,
    path::Path,
    ptr::{self},
    thread,
    time::Duration,
};
use tokio::sync::{mpsc::Receiver, Mutex};

use crate::{configuration::CONFIG, event_system};

mod api_callbacks;

// todo: редизайн типов чтобы такой хуеты как с Library не было
// !! порядок полей менять НЕЛЬЗЯ тоже может быть сегфолт
struct PluginRuntimeInfo {
    plugin_information: Box<PluginInformation>,
    _library: Library, // это поле вообще никгде не юзается, но без него сегфолт.
    state: *mut State,
}

/// Event publishing from plugins.
#[derive(Debug, Serialize)]
pub struct PluginEvent {
    sender: String,
    data: String,
}

/// Loads plugins from path from config.
pub fn load_plugins(receiver: Mutex<Receiver<String>>) {
    unsafe {
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let libraries_path = find_plugins();

                let mut plugins_data = load_plugin_data(libraries_path);

                run_inits(&mut plugins_data);
                do_loop(&mut plugins_data, receiver).await
            })
        })
    };
}

/// Finds plugins for user's OS and returs their pathes.
fn find_plugins() -> Vec<String> {
    let plugins_folder = &CONFIG.plugins.plugins_folder;
    let extension = if cfg!(target_family = "unix") {
        "so" // sal?
    } else {
        "dll"
    };

    find_files_with_extension(Path::new(plugins_folder), extension).unwrap_or_default()
}

fn find_files_with_extension(dir: &Path, extension: &str) -> io::Result<Vec<String>> {
    let mut files_with_extension = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            if let Some(ext) = path.extension() {
                if ext == extension {
                    if let Some(path_str) = path.to_str() {
                        files_with_extension.push(path_str.to_string());
                    }
                }
            }
        }
    }

    Ok(files_with_extension)
}

async unsafe fn do_loop(plugins_data: &mut [PluginRuntimeInfo], receiver: Mutex<Receiver<String>>) {
    let mut recv = receiver.lock().await;
    loop {
        for info in &mut *plugins_data {
            if !recv.is_empty() {
                check_event_for_send(info, &mut recv).await;
            } else {
                (info.plugin_information.execute_callback)(info.state, api_callbacks::get_api());
            }
            check_event_for_publish(info).await;
        }
        tokio::time::sleep(Duration::from_micros(100)).await;
    }
}

async unsafe fn check_event_for_send<'a>(
    info: &mut PluginRuntimeInfo,
    event_recv: &mut tokio::sync::MutexGuard<'a, Receiver<String>>,
) {
    let event_callback = info.plugin_information.event_callback;
    let recieved_event = event_recv.recv().await;

    // Maybe we should pass some set of events instead one?
    let ptr = extract_ptr(recieved_event);

    let event_state = Box::into_raw(Box::new(EventState {
        state: info.state,
        event: ptr,
    }));
    (event_callback)(event_state, api_callbacks::get_api());
}

fn extract_ptr(res: Option<String>) -> *const std::ffi::c_char {
    CString::new(res.expect("mpsc for events was closed. this is a bug."))
        // release ownership here because memory frees in free_event_memory()
        .map(|cstring| cstring.into_raw().cast_const())
        .unwrap_or_else(|_| {
            warn!("Event string representation contains zero byte, which is not allowed.");
            ptr::null()
        })
}

async unsafe fn check_event_for_publish(info: &mut PluginRuntimeInfo) {
    if let Some(plugin_state) = ptr::NonNull::new(info.state) {
        check_event(plugin_state, info).await;
        check_request(plugin_state).await;

        free_memory(info);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadableRequest(pub String);

async unsafe fn check_request(plugin_state: ptr::NonNull<State>) {
    if let Some(request_ptr) = ptr::NonNull::new(plugin_state.read().human_request) {
        let request_data = CStr::from_ptr(request_ptr.as_ptr()).to_str();
        if let Ok(str_data) = request_data {
            event_system::publish(ReadableRequest(str_data.to_string())).await;
        }
    }
}

async unsafe fn check_event(plugin_state: ptr::NonNull<State>, info: &mut PluginRuntimeInfo) {
    if let Some(published_event) = ptr::NonNull::new(plugin_state.read().published_event) {
        let published_event_data = CStr::from_ptr(published_event.as_ptr()).to_str();
        match published_event_data {
            Ok(str_data) => {
                let general_event = PluginEvent {
                    sender: CStr::from_ptr(info.plugin_information.name)
                        .to_str()
                        .unwrap() /* i think, plugin name will be not changed due asya
                        lifetime, so that we don't have to check this every time. */
                        .to_string(),
                    data: str_data.to_string(),
                };
                event_system::publish(general_event).await;
            }
            Err(_) => {
                warn!("Plugin sent an event, that cannot be represent as a valid Utf8 string. Event wasn'n published.")
            }
        }
    }
}

unsafe fn free_memory(info: &mut PluginRuntimeInfo) {
    let event_raw = (*info.state).published_event;
    if !event_raw.is_null() {
        drop(Box::from_raw(event_raw)); // panics if plugins frees memory independently, e.g. if after
                                        // casting event to CString
        (*info.state).published_event = ptr::null_mut()
    }

    let message_raw = (*info.state).readable_message;
    if !message_raw.is_null() {
        drop(Box::from_raw(message_raw)); // same as above
        (*info.state).readable_message = ptr::null_mut()
    }

    let request_raw = (*info.state).human_request;
    if !message_raw.is_null() {
        drop(Box::from_raw(request_raw)); // same as above
        (*info.state).human_request = ptr::null_mut()
    }
}

unsafe fn load_plugin_data(libs: Vec<String>) -> Vec<PluginRuntimeInfo> {
    const FN_PLUGIN_INFO: &[u8; 11] = b"plugin_info";
    let mut infos = vec![];
    for lib in libs {
        let library = match Library::new(&lib) {
            Ok(lib) => lib,
            Err(err) => {
                warn!("Library {} wasn't loaded due error: {}", lib, err);
                continue;
            }
        };
        let plugin_information_callback = match library
            .get::<*mut plugin_interface::PluginInfoCallback>(FN_PLUGIN_INFO)
        {
            Ok(callback) => callback.read(),
            Err(err) => {
                warn!(
                    "Library {} wasn't loaded 
                    because lib doesn't containt valid FN_PLUGIN_INFO function or / and it's signature is incorrect. | {}",
                    lib,
                    err
                );
                continue;
            }
        };

        let boxed_plugin_information_callback =
            Box::from_raw(plugin_information_callback().cast_mut());

        infos.push(PluginRuntimeInfo {
            _library: library,
            state: ptr::null_mut(),
            plugin_information: boxed_plugin_information_callback,
        });
    }
    infos
}

unsafe fn run_inits(infos: &mut Vec<PluginRuntimeInfo>) {
    for info in infos {
        info.state = (info.plugin_information.init_callback)(api_callbacks::get_api());
    }
}
