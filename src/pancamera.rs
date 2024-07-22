use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::transform::components::Transform;
use bevy::prelude::*;
use bevy::app::{App, Plugin, Update};

pub struct PanCamera;

const DRAG_BUTTON : MouseButton = MouseButton::Left;

impl Plugin for PanCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_movement);
        app.add_systems(Update, cursor_grab);
    }
}

fn camera_movement(
    mut query: Query<&mut Transform, With<Camera2d>>,
    mut motion: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.pressed(DRAG_BUTTON) {
        let mut transform = query.single_mut();
        for event in motion.read() {
            transform.translation.x -= event.delta.x;
            transform.translation.y += event.delta.y;
        }
    }
}

fn cursor_grab(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let mut primary_window = window.single_mut();

    if buttons.just_pressed(DRAG_BUTTON) {
        primary_window.cursor.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor.visible = false;
    }

    if buttons.just_released(DRAG_BUTTON) {
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    }
}
