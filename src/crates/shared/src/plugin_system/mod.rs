use libloading::Library;
use plugin_interface::{EventState, PluginInformation};
use serde::Serialize;
use std::{
    ffi::{c_void, CString},
    ptr, thread,
    time::Duration,
};
use tokio::sync::{mpsc::Receiver, Mutex};

#[derive(Debug, Serialize)]
pub enum Loaded {
    NeBilo { pochemu: String },
}

struct PluginRuntimeInfo {
    plugin_information: Box<PluginInformation>,
    #[allow(dead_code)]
    library: Library, // опасная темка
    state: *const c_void,
}

pub fn load_plugins(receiver: Mutex<Receiver<String>>) {
    unsafe {
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let libs = vec!["./plugins/libtest_dynamic_lib.so"];

                let mut infos = load_infos(libs);

                run_inits(&mut infos); // ! сегфолтит  сука !

                loop {
                    for info in &mut infos {
                        let event_callback = info.plugin_information.event_callback;
                        thread::sleep(Duration::from_micros(1_000));
                        let mut recv = receiver.lock().await;
                        let res = recv.recv().await;
                        let ptr = CString::new(res.unwrap()).unwrap();
                        let state = event_callback(EventState {
                            state: info.state,
                            event: ptr.as_ptr(),
                        });
                        info.state = state.state;
                    }
                }
            })
        })
    };
}

unsafe fn load_infos(libs: Vec<&str>) -> Vec<PluginRuntimeInfo> {
    let mut infos = vec![];
    for lib in libs {
        let library = Library::new(lib).unwrap();
        let plugin_information = library
            .get::<*mut plugin_interface::PluginInfo>(b"plugin_info")
            .expect("lib is not loaded")
            .read();

        let plugin_information = Box::from_raw(plugin_information().cast_mut());

        infos.push(PluginRuntimeInfo {
            library,
            state: ptr::null(),
            plugin_information,
        });
    }
    infos
}

#[allow(clippy::vec_box)] // рассмотрим это позже
fn run_inits(infos: &mut Vec<PluginRuntimeInfo>) {
    unsafe {
        for info in infos {
            let a = (info.plugin_information.init_callback)(); // state is not safe yet
            info.state = a.state;
        }
    }
}
