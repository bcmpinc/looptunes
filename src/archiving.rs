use bevy::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider};
use web_sys::{wasm_bindgen::{prelude::Closure, JsCast}, EventTarget};

pub struct ArchivingPlugin;

impl Plugin for ArchivingPlugin {
    fn build(&self, app: &mut App) {
        let mut ctx = ClipboardContext::new().unwrap();
        println!("Test: {:?}", ctx.get_contents());
        let x = ctx.set_contents("This is a test".into());
        println!("Res: {:?}", x);

        let window = web_sys::window().unwrap();

        web_sys::console::log_1(&format!("Initializing clipboard!").into());

        // Set up a paste event listener.
        let paste_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::ClipboardEvent| {
            web_sys::console::log_1(&format!("Pasting").into());
            if let Some(clipboard_data) = event.clipboard_data() {
                if let Ok(pasted_text) = clipboard_data.get_data("text/plain") {
                    web_sys::console::log_1(&format!("Pasted text: {}", pasted_text).into());
                }
            }
        });

        _ = window.add_event_listener_with_callback("paste", paste_closure.as_ref().unchecked_ref());

        // Prevent the closure from being dropped.
        paste_closure.forget();

        // Set up a copy event listener.
        let copy_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::ClipboardEvent| {
            web_sys::console::log_1(&format!("Copying").into());
            if let Some(clipboard_data) = event.clipboard_data() {
                _ = clipboard_data.set_data("text/plain", "Web clipboard content from rust").unwrap();
                web_sys::console::log_1(&"Copy event handled!".into());
            }
            event.prevent_default();
        });

        _ = window.add_event_listener_with_callback("copy", copy_closure.as_ref().unchecked_ref());

        // Prevent the closure from being dropped.
        copy_closure.forget();

    }
}
