use bevy::render::camera::RenderTarget;
use bevy::window::{Cursor, WindowRef};
use bevy::{app::Plugin, math::Vec3};
use bevy::prelude::*;

// With inspiration from: https://crates.io/crates/bevy_mouse_tracking_plugin.

pub struct MiceTrack;

impl Plugin for MiceTrack {
    fn build(&self, app: &mut App) {
        app.add_systems(First, tracking);
    }
}

#[derive(Component)]
pub struct MousePos {
    position: Vec3,
    on_screen: bool,
}

impl Default for MousePos {
    fn default() -> Self {
        Self {
            position: default(),
            on_screen: false,
        }
    }
}

fn tracking (
    q: Query<(&mut MousePos, &Camera)>,
    windows: Query<&Window>,
) {
    for (mut pos, camera) in q.iter() {
        if let RenderTarget::Window(WindowRef::Entity(window_entity)) = camera.target {
            let window = windows.get(window_entity).unwrap();
            println!("Yay camera time!")
        }
        if let RenderTarget::Window(WindowRef::Primary) = camera.target {
            println!("Sadness :(")
            
        }
    }
}
