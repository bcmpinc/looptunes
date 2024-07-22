use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::transform::components::Transform;
use bevy::prelude::*;
use bevy::app::{App, Plugin, Update};

pub struct PanCamera;

const DRAG_BUTTON : MouseButton = MouseButton::Left;
const ZOOM_SENSITIVITY: f32 = -0.2;

impl Plugin for PanCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_movement);
        app.add_systems(Update, cursor_grab);
        app.add_systems(Update, camera_zoom);
    }
}

fn camera_movement(
    mut query: Query<&mut Transform, With<Camera2d>>,
    mut motion: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.pressed(DRAG_BUTTON) {
        let mut transform = query.single_mut();
        let scale = transform.scale.x;
        for event in motion.read() {
            transform.translation.x -= event.delta.x * scale;
            transform.translation.y += event.delta.y * scale;
        }
    }
}

fn camera_zoom(
    mut query: Query<&mut Transform, With<Camera2d>>,
    mut scroll: EventReader<MouseWheel>,
) {
    let mut transform = query.single_mut();
    for event in scroll.read() {
        let scale_change = f32::powf(2.0, event.y * ZOOM_SENSITIVITY);
        transform.scale *= Vec3::new(scale_change, scale_change, 1.0);
        transform.scale = Vec3::max(transform.scale, Vec3::new(0.1, 0.1, 1.0));
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
