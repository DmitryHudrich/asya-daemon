use std::{
    ffi::{c_char, CString},
    thread,
    time::Duration,
};

use lazy_static::lazy_static;
use libloading::Library;
use log::info;
use serde::Serialize;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

type CallbackVec = Vec<unsafe extern "C" fn(*const c_char)>;

#[derive(Debug, Serialize)]
pub enum Loaded {
    NeBilo { pochemu: String },
}

// lazy_static! {
//     pub static ref INSTANCE: PluginManager = {
//         let (sender, receiver) = mpsc::channel(32);
//         PluginManager { sender, receiver }
//     };
// }
//
// pub struct PluginManager {
//     pub sender: Sender<String>,
//     pub receiver: Receiver<String>,
// }

pub fn load_plugins(receiver: Mutex<Receiver<String>>) {
    unsafe {
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let library = Library::new("./plugins/libtest_dynamic_lib.so").unwrap();
                let plugin_info = library
                    .get::<*mut plugin_interface::PluginInfo>(b"plugin_info")
                    .unwrap()
                    .read();

                let plugin_information = Box::from_raw(plugin_info().cast_mut());
                let plugin_name = CString::from_raw(plugin_information.name.cast_mut())
                    .into_string()
                    .unwrap();

                info!("Loaded plugin {plugin_name}!");

                let callbacks = Vec::from_raw_parts(
                    plugin_information.callback.cast_mut(),
                    plugin_information.total_callbacks as usize,
                    plugin_information.total_callbacks as usize,
                );

                loop {
                    for callback in callbacks.clone() {
                        for _ in 0..1000 {
                            // let input = if i == 999 { "я eblan" } else { "секс" };
                            // let cstr = CString::new(input).unwrap();
                            thread::sleep(Duration::from_micros(1_000));
                            let mut recv = receiver.lock().await;
                            let res = recv.recv().await;
                            let ptr = CString::new(res.unwrap()).unwrap();
                            callback(ptr.as_ptr());
                        }
                    }
                }
            })
        })
    };
}
