use bevy::{ecs::system::SystemId, prelude::*};

#[cfg(not(target_family="wasm"))] pub use self::native::*;
#[cfg(target_family="wasm")] pub use self::wasm::*;

#[derive(Resource)]
pub struct Clipboard {
    pub copy: SystemId<(),String>,
    pub paste: SystemId<String>,
}


mod wasm {
    use bevy::ecs::system::SystemId;
    use bevy::prelude::*;
    use crossbeam_channel::{bounded, Receiver};
    use web_sys::ClipboardEvent;
    use web_sys::wasm_bindgen::JsCast;
    use web_sys::wasm_bindgen::prelude::Closure;

    use super::ClipboardResource;
    use crate::println;

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
            Some(pasted_text)
        }

        fn try_copy(&mut self, callback: impl FnOnce() -> String) -> Option<()> {
            let event = self.copy_queue.try_recv().ok()?;
            let clipboard_data = event.clipboard_data()?;
            let copied_text = callback();
            clipboard_data.set_data("text/plain", &copied_text).ok()?;
            Some(())
        }
    }

    fn test(clipboard: Res<'_, Clipboard>) -> String {"test".into()}
    fn test2(pasted: In<String>, clipboard: Res<'_, Clipboard>) {}

    pub struct ClipboardPlugin;
    impl Plugin for ClipboardPlugin {
        fn build(&self, app: &mut App) {
            let x = app.register_system(test);
            let x = app.register_system(test2);
            let world = unsafe{app.world_mut().as_unsafe_world_cell().world_mut()};
            //app.world_mut();
            let test: String = "test".into();
            world.run_system_with_input(x, test);

            // Generate message passing channels.            
            let (copy_tx, copy_queue ) = bounded::<ClipboardEvent>(1);
            let (paste_tx,paste_queue) = bounded::<ClipboardEvent>(1);
            
            let window = web_sys::window().unwrap();

            // Set up a copy event listener.
            let copy_closure = Closure::<dyn FnMut(_)>::new(move |event: ClipboardEvent| {
                println!("Copying event");
                event.prevent_default();
                _ = copy_tx.try_send(event);
            });
            _ = window.add_event_listener_with_callback("copy", copy_closure.as_ref().unchecked_ref());
            copy_closure.forget();

            // Set up a paste event listener.
            let paste_closure = Closure::<dyn FnMut(_)>::new(move |event: ClipboardEvent| {
                println!("Pasting event");
                event.prevent_default();
                _ = paste_tx.try_send(event);
            });
            _ = window.add_event_listener_with_callback("paste", paste_closure.as_ref().unchecked_ref());
            paste_closure.forget();
             
            // Insert the Clipboard resource
            app.insert_resource(Clipboard{
                copy_queue,
                paste_queue,
            });

            app.add_systems(First, |
                keyboard: Res<ButtonInput<KeyCode>>,
                clipboard: ResMut<Clipboard>,
            | {
                if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
                    if keyboard.just_pressed(KeyCode::KeyC) {
                        println!("Copying shortcut");
                    }
                    if keyboard.just_pressed(KeyCode::KeyV) {
                        println!("Pasting shortcut");
                    }
                }
            });
        }
    }
}

mod native {
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
                .add_systems(First, key_handler);
        }
    }

    fn key_handler(
        mut commands: Commands,
        keyboard: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
            if keyboard.just_pressed(KeyCode::KeyC) {
                commands.add(copy_command);
            }
            if keyboard.just_pressed(KeyCode::KeyV) {
                commands.add(paste_command);
            }
        }
    }

    fn copy_command(world: &mut World) {
        let Some(clipboard) = world.get_resource::<Clipboard>() else {return};
        let mut internal = world.get_resource_mut::<ClipboardInternal>().unwrap();
        let Ok(copy_text) = world.run_system(clipboard.copy) else {return};
        internal.ctx.set_contents(copy_text);
    }

    fn paste_command(world: &mut World) {
        let Some(clipboard) = world.get_resource::<Clipboard>() else {return};
        let mut internal = world.get_resource_mut::<ClipboardInternal>().unwrap();
        let paste_text = internal.ctx.get_contents() else {return};
        world.run_system_with_input(clipboard.paste, paste_text);
    }
}
