use bevy::{ecs::system::SystemId, prelude::*};

#[derive(Resource)]
pub struct Clipboard {
    pub copy: SystemId<(),String>,
    pub paste: SystemId<String>,
}


#[cfg(target_family="wasm")] pub use self::wasm::*;
#[cfg(target_family="wasm")] mod wasm {
    use bevy::prelude::*;
    use crossbeam_channel::{bounded, Receiver};
    use web_sys::js_sys::{wasm_bindgen, Promise};
    use web_sys::ClipboardEvent;
    use web_sys::wasm_bindgen::JsCast;
    use web_sys::wasm_bindgen::prelude::Closure;

    use crate::println;

    pub struct ClipboardInternal {
        copy_queue: Receiver<ClipboardEvent>,
        paste_queue: Receiver<ClipboardEvent>,
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
                println!("Copying event");
                _ = copy_tx.try_send(event);
                event.prevent_default();
            });
            _ = window.add_event_listener_with_callback("copy", copy_closure.as_ref().unchecked_ref());
            copy_closure.forget();

            // Set up a paste event listener.
            let paste_closure = Closure::<dyn FnMut(_)>::new(move |event: ClipboardEvent| {
                println!("Pasting event");
                _ = paste_tx.try_send(event);
                event.prevent_default();
            });
            _ = window.add_event_listener_with_callback("paste", paste_closure.as_ref().unchecked_ref());
            paste_closure.forget();
             
            // Insert the Clipboard resource
            app.insert_non_send_resource(ClipboardInternal{
                copy_queue,
                paste_queue,
            });

            app.add_systems(First, event_handler);
        }
    }

    fn event_handler(
        world: ResMut<World>,
        internal: NonSendMut<ClipboardInternal>,
    ) {
        if let Some(event) = self.paste_queue.try_recv().ok() {
            let clipboard_data = event.clipboard_data()?;
            let pasted_text = clipboard_data.get_data("text").ok()?;
            Some(pasted_text)
        }
        
        if let Some(event) = self.copy_queue.try_recv().ok() {
            let clipboard_data = event.clipboard_data()?;
            let copied_text = callback();
            clipboard_data.set_data("text", &copied_text).ok()?;
            Some(())
        }
    }
}

#[cfg(not(target_family="wasm"))] pub use self::native::*;
#[cfg(not(target_family="wasm"))] mod native {
    use bevy::prelude::*;
    use copypasta::*;

    use super::Clipboard;

    #[derive(Resource)] struct ClipboardInternal {
        ctx: ClipboardContext,
    }

    pub struct ClipboardPlugin;
    impl Plugin for ClipboardPlugin {
        fn build(&self, app: &mut App) {
            app
                .insert_resource(ClipboardInternal {
                    ctx: ClipboardContext::new().unwrap()
                })
                .add_systems(First, event_handler);
        }
    }

    fn event_handler(world: &mut World) {
        let Some(keyboard) = world.get_resource::<ButtonInput<KeyCode>>() else {return};
        if !keyboard.pressed(KeyCode::ControlLeft) && !keyboard.pressed(KeyCode::ControlRight) {return};
        let do_copy = keyboard.just_pressed(KeyCode::KeyC);
        let do_paste = keyboard.just_pressed(KeyCode::KeyV);
        let Some(clipboard) = world.get_resource::<Clipboard>() else {return};
        if do_copy {
            let copy = clipboard.copy;
            let Ok(copy_text) = world.run_system(copy) else {return};
            let mut internal = world.get_resource_mut::<ClipboardInternal>().unwrap();
            _ = internal.ctx.set_contents(copy_text);
        }
        if do_paste {
            let Some(clipboard) = world.get_resource::<Clipboard>() else {return};
            let paste = clipboard.paste;
            let mut internal = world.get_resource_mut::<ClipboardInternal>().unwrap();
            let Ok(paste_text) = internal.ctx.get_contents() else {return};
            _ = world.run_system_with_input(paste, paste_text);
        }
    }
}
