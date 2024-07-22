use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};


#[derive(Resource, Clone, Copy)]
pub struct Wireframe(pub KeyCode);

impl Plugin for Wireframe {
    fn build(&self, app: &mut App) {
        app.add_plugins(Wireframe2dPlugin);
        app.insert_resource(*self);
        app.add_systems(Update, toggle_wireframe);
    }
}

fn toggle_wireframe(
    wireframe: Res<Wireframe>,
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(wireframe.0) {
        wireframe_config.global = !wireframe_config.global;
    }
}
