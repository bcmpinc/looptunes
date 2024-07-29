use bevy::{ecs::system::SystemId, prelude::*};

#[derive(Resource, Clone)]
pub struct Clipboard {
    pub copy: SystemId<(),String>,
    pub paste: SystemId<String>,
}

#[cfg(target_family="wasm")] pub use self::wasm::*;
#[cfg(target_family="wasm")] mod wasm {
    use std::ptr::null_mut;

    use bevy::prelude::*;
    use web_sys::ClipboardEvent;
    use web_sys::wasm_bindgen::JsCast;
    use web_sys::wasm_bindgen::prelude::Closure;

    use crate::println;

    use super::Clipboard;

    // Here be dragons! 
    static mut WORLD_ALIAS: *mut World = null_mut();

    pub struct ClipboardPlugin;
    impl Plugin for ClipboardPlugin {
        fn build(&self, app: &mut App) {
            let window = web_sys::window().unwrap();

            // Set up a copy event listener.
            let copy_closure = Closure::<dyn FnMut(_)>::new(move |event: ClipboardEvent| {
                println!("Copying event");
                let world = unsafe{&mut *WORLD_ALIAS};

                let clipboard = world.get_resource::<Clipboard>().unwrap();
                let copy_text = world.run_system(clipboard.copy).unwrap();
                let clipboard_data = event.clipboard_data().unwrap();
                _ = clipboard_data.set_data("text", &copy_text);

                event.prevent_default();
            });
            _ = window.add_event_listener_with_callback("copy", copy_closure.as_ref().unchecked_ref());
            copy_closure.forget();

            // Set up a paste event listener.
            let paste_closure = Closure::<dyn FnMut(_)>::new(move |event: ClipboardEvent| {
                println!("Pasting event");
                let world = unsafe{&mut *WORLD_ALIAS};

                let clipboard_data = event.clipboard_data().unwrap();
                let paste_text = clipboard_data.get_data("text").unwrap();
                let clipboard = world.get_resource::<Clipboard>().unwrap();
                _ = world.run_system_with_input(clipboard.paste, paste_text);

                event.prevent_default();
            });
            _ = window.add_event_listener_with_callback("paste", paste_closure.as_ref().unchecked_ref());
            paste_closure.forget();

            app.add_systems(Last, |world: &mut World| unsafe{WORLD_ALIAS = world as *mut World});
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
        let clipboard = clipboard.clone();
        if do_copy {
            let Ok(copy_text) = world.run_system(clipboard.copy) else {return};
            let mut internal = world.get_resource_mut::<ClipboardInternal>().unwrap();
            _ = internal.ctx.set_contents(copy_text);
        }
        if do_paste {
            let mut internal = world.get_resource_mut::<ClipboardInternal>().unwrap();
            let Ok(paste_text) = internal.ctx.get_contents() else {return};
            _ = world.run_system_with_input(clipboard.paste, paste_text);
        }
    }
}
