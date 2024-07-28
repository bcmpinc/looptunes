use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::transform::components::Transform;
use bevy::prelude::*;
use bevy::app::{App, Plugin, Update};

use crate::MousePos;

const ZOOM_SENSITIVITY: Vec3 = Vec3::new(0.9, 0.9, 1.0);

#[derive(Resource, Clone, Copy)]
pub struct PanCameraPlugin(pub MouseButton);

#[derive(SystemSet,Hash,Debug,PartialEq,Eq,Clone,Copy)] pub struct PanSystem;
#[derive(SystemSet,Hash,Debug,PartialEq,Eq,Clone,Copy)] pub struct ZoomSystem;

impl Plugin for PanCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(*self);
        app.add_systems(Update, (cursor_grab, camera_movement).in_set(PanSystem).chain());
        app.add_systems(Update, camera_zoom.in_set(ZoomSystem));
    }
}

fn camera_movement(
    pan_camera: Res<PanCameraPlugin>,
    mut query: Query<&mut Transform, With<Camera2d>>,
    mut motion: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.pressed(pan_camera.0) {
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
    cursor: Res<MousePos>,
) {
    let mut transform = query.single_mut();
    let mut offset = transform.translation - Vec3::new(cursor.position.x, cursor.position.y, 0.0);
    for event in scroll.read() {
        transform.translation -= offset;
        if event.y < 0.0 {
            transform.scale /= ZOOM_SENSITIVITY;
            offset /= ZOOM_SENSITIVITY;
        } else {
            transform.scale *= ZOOM_SENSITIVITY;
            offset *= ZOOM_SENSITIVITY;
        }
        transform.scale = Vec3::max(transform.scale, Vec3::new(0.0003, 0.0003, 1.0));
        transform.translation += offset;
    }
}

fn cursor_grab(
    pan_camera: Res<PanCameraPlugin>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let mut primary_window = window.single_mut();

    if buttons.just_pressed(pan_camera.0) {
        primary_window.cursor.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor.visible = false;
    }

    if buttons.just_released(pan_camera.0) {
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    }
}
