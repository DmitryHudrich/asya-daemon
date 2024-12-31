use libloading::Library;
use plugin_interface::{EventState, PluginInformation};
use std::{
    ffi::{c_void, CString},
    ptr, thread,
    time::Duration,
};
use tokio::sync::{mpsc::Receiver, Mutex};

// todo: редизайн типов чтобы такой хуеты как с Library не было
// !! порядок полей менять НЕЛЬЗЯ тоже может быть сегфолт
struct PluginRuntimeInfo {
    plugin_information: Box<PluginInformation>,
    _library: Library, // это поле вообще никгде не юзается, но без него сегфолт.
    state: *const c_void,
}

pub fn load_plugins(receiver: Mutex<Receiver<String>>) {
    unsafe {
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let libs = vec!["./plugins/libtest_dynamic_lib.so"];

                let mut plugins_data = load_plugin_data(libs);

                run_inits(&mut plugins_data);
                poll_execute(plugins_data, receiver).await
            })
        })
    };
}

async unsafe fn poll_execute(
    mut plugins_data: Vec<PluginRuntimeInfo>,
    receiver: Mutex<Receiver<String>>,
) {
    loop {
        for info in &mut plugins_data {
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
}

unsafe fn load_plugin_data(libs: Vec<&str>) -> Vec<PluginRuntimeInfo> {
    const FN_PLUGIN_INFO: &[u8; 11] = b"plugin_info";
    let mut infos = vec![];
    for lib in libs {
        let library = Library::new(lib).unwrap();
        let plugin_information = library
            .get::<*mut plugin_interface::PluginInfo>(FN_PLUGIN_INFO)
            .expect("lib is not loaded")
            .read();

        let plugin_information = Box::from_raw(plugin_information().cast_mut());

        infos.push(PluginRuntimeInfo {
            _library: library,
            state: ptr::null(),
            plugin_information,
        });
    }
    infos
}

unsafe fn run_inits(infos: &mut Vec<PluginRuntimeInfo>) {
    for info in infos {
        let new_state = (info.plugin_information.init_callback)();
        info.state = new_state.state;
    }
}
