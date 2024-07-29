use bevy::prelude::*;

#[cfg(not(target_family="wasm"))] pub use self::native::*;
#[cfg(target_family="wasm")] pub use self::wasm::*;

pub trait ClipboardResource : Resource {
    fn try_paste(&mut self) -> Option<String>;
    fn try_copy(&mut self, callback: impl FnOnce() -> String) -> Option<()>;
}

mod wasm {
    use bevy::prelude::*;
    use crossbeam_channel::{bounded, Receiver};
    use web_sys::ClipboardEvent;
    use web_sys::wasm_bindgen::JsCast;
    use web_sys::wasm_bindgen::prelude::Closure;

    use super::ClipboardResource;

    #[derive(Resource)] pub struct Clipboard {
        copy_queue: Receiver<ClipboardEvent>,
        paste_queue: Receiver<ClipboardEvent>,
    }

    unsafe impl Send for Clipboard {} // WASM is single threaded
    unsafe impl Sync for Clipboard {} // WASM is single threaded

    impl ClipboardResource for Clipboard {
        fn try_paste(&mut self) -> Option<String> {
            let event = self.paste_queue.try_recv().ok()?;
            let clipboard_data = event.clipboard_data()?;
            let pasted_text = clipboard_data.get_data("text/plain").ok()?;
            event.prevent_default();
            Some(pasted_text)
        }

        fn try_copy(&mut self, callback: impl FnOnce() -> String) -> Option<()> {
            let event = self.copy_queue.try_recv().ok()?;
            let clipboard_data = event.clipboard_data()?;
            let copied_text = callback();
            clipboard_data.set_data("text/plain", &copied_text).ok()?;
            event.prevent_default();
            Some(())
        }
    }

    pub struct ClipboardPlugin;
    impl Plugin for ClipboardPlugin {
        fn build(&self, app: &mut App) {

            // Generate message passing channels.            
            let (copy_tx, copy_queue ) = bounded::<ClipboardEvent>(1);
            let (paste_tx,paste_queue) = bounded::<ClipboardEvent>(1);
            
            let window = web_sys::window().unwrap();

            // Set up a copy event listener.
            let copy_closure = Closure::<dyn FnMut(_)>::new(move |event: ClipboardEvent| {
                _ = copy_tx.try_send(event);
            });
            _ = window.add_event_listener_with_callback("copy", copy_closure.as_ref().unchecked_ref());
            copy_closure.forget();

            // Set up a paste event listener.
            let paste_closure = Closure::<dyn FnMut(_)>::new(move |event: ClipboardEvent| {
                _ = paste_tx.try_send(event);
            });
            _ = window.add_event_listener_with_callback("paste", paste_closure.as_ref().unchecked_ref());
            paste_closure.forget();
             
            // Insert the Clipboard resource
            app.insert_resource(Clipboard{
                copy_queue,
                paste_queue,
            });
        }
    }
}

mod native {
    use std::sync::atomic::{AtomicBool, Ordering};

    use bevy::prelude::*;
    use copypasta::*;

    use super::ClipboardResource;

    fn is_ctrl(keyboard: &ButtonInput<KeyCode>) -> bool {
        keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight)
    }
    
    #[derive(Resource)] pub struct Clipboard {
        ctx: ClipboardContext,
        request_copy: AtomicBool,
        request_paste: AtomicBool,
    }

    impl ClipboardResource for Clipboard {
        fn try_paste(&mut self) -> Option<String> {
            if self.request_paste.swap(false, Ordering::Relaxed) {
                self.ctx.get_contents().ok()
            } else {
                None
            }
        }

        fn try_copy(&mut self, callback: impl FnOnce() -> String) -> Option<()>{
            if self.request_copy.swap(false, Ordering::Relaxed) {
                self.ctx.set_contents(callback()).ok()
            } else {
                None
            }
        }
    }

    pub struct ClipboardPlugin;
    impl Plugin for ClipboardPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(Clipboard {
                ctx: ClipboardContext::new().unwrap(),
                request_copy: AtomicBool::new(false),
                request_paste: AtomicBool::new(false),
            });
            app.add_systems(First, |
                keyboard: Res<ButtonInput<KeyCode>>,
                clipboard: ResMut<Clipboard>,
            | {
                if is_ctrl(&keyboard) {
                    if keyboard.just_pressed(KeyCode::KeyC) {
                        clipboard.request_copy.store(true, Ordering::Relaxed);
                    }
                    if keyboard.just_pressed(KeyCode::KeyV) {
                        clipboard.request_paste.store(true, Ordering::Relaxed);
                    }
                }
            });
        }
    }
}
