use bevy::window::Cursor;
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

fn tracking (
    mut commands: Commands,
    q: Query<(Entity, &Camera2d, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    for mut camera in q.iter() {


        let position = MousePos{
            position: Vec3::ZERO, 
            on_screen: false,
        };
        commands.entity(camera.0).insert(position);
    }
}
