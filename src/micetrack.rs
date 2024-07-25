
use bevy::app::Plugin;
use bevy::prelude::*;

// With inspiration from: https://crates.io/crates/bevy_mouse_tracking_plugin.
// and: https://bevyengine.org/examples/2d-rendering/2d-viewport-to-world/

pub struct MiceTrack;

impl Plugin for MiceTrack {
    fn build(&self, app: &mut App) {
        app
            .add_systems(First, tracking)
            .insert_resource(MousePos::default());
    }
}

#[derive(Resource)]
pub struct MousePos {
    pub position: Vec2,
    pub on_screen: bool,
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
    q_mouse: Query<(&Camera, &GlobalTransform)>,
    q_window: Query<&Window>,
    mut pos: ResMut<MousePos>,
) {
    let (camera, camera_transform) = q_mouse.single();
    let window = q_window.single();
    if let Some(viewport_position) = window.cursor_position() {
        pos.position = camera.viewport_to_world_2d(camera_transform, viewport_position).unwrap();
        pos.on_screen = true;
    } else {
        pos.on_screen = false;
        return;
    }
}
