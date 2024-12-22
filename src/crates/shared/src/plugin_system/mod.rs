use std::ffi::CString;

use libloading::Library;
use plugin_interface::*;

pub async fn load_plugins() -> anyhow::Result<()> {
    unsafe {
        let library = Library::new("./plugins/libtest_dynamic_lib.so")?;
        let plugin_info = library
            .get::<*mut plugin_interface::PluginInfo>(b"plugin_info")?
            .read();

        let plugin_information = Box::from_raw(plugin_info().cast_mut());
        let plugin_name = CString::from_raw(plugin_information.name.cast_mut()).into_string()?;
        println!("Loaded plugin {plugin_name}!");

        let callbacks = Vec::from_raw_parts(
            plugin_information.callback.cast_mut(),
            plugin_information.total_callbacks as usize,
            plugin_information.total_callbacks as usize,
        );
        callbacks.iter().for_each(|handler| {
            (handler)(&Event {
                name: CString::new("eblan event").unwrap(),
                invariant: Invariant {
                    name: CString::new("Test invariant").unwrap(),
                    fields: std::ptr::null(),
                    fields_len: 0,
                },
                available_invariants: vec![InvariantDeclaration {
                    name: CString::new("Test invariant").unwrap(),
                    fields: vec![CString::new("Test field").unwrap()].as_ptr(),
                    fields_len: 1,
                }]
                .as_ptr(),
                available_invariants_len: 1,
            })
        });
    };
    Ok(())
}
