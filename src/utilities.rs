use bevy::input::ButtonInput;
use bevy::prelude::*;

pub trait CommandsExt<T> {
    fn try_despawn(&mut self, id: T);
}

impl CommandsExt<Entity> for Commands<'_,'_> {
    fn try_despawn(&mut self, id: Entity) {
        if let Some(mut c) = self.get_entity(id) {
            c.remove_parent().clear_children().despawn();
        }
    }
}

impl CommandsExt<Option<Entity>> for Commands<'_,'_> {
    fn try_despawn(&mut self, id: Option<Entity>) {
        if let Some(entity) = id {
            self.try_despawn(entity);
        };
    }
}

impl<Err> CommandsExt<Result<Entity,Err>> for Commands<'_,'_> {
    fn try_despawn(&mut self, id: Result<Entity, Err>) {
        if let Ok(entity) = id {
            self.try_despawn(entity);
        };
    }
}

pub fn is_ctrl(keyboard: &ButtonInput<KeyCode>) -> bool {
    keyboard.pressed(KeyCode::ControlLeft)  || keyboard.pressed(KeyCode::ControlRight)
}

pub fn is_shift(keyboard: &ButtonInput<KeyCode>) -> bool {
    keyboard.pressed(KeyCode::ShiftLeft)  || keyboard.pressed(KeyCode::ShiftRight)
}

#[cfg(target_family="wasm")]
#[macro_export]
macro_rules! println {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}
#[cfg(not(target_arch = "wasm32"))]
pub use std::println;

pub fn soft_signum(val: f32) -> f32 {
    if val < -0.1 {return -1.0;}
    if val >  0.1 {return  1.0;}
    return 0.0;
}