use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::sprite::Wireframe2dConfig;


#[derive(Resource, Clone, Copy)]
pub struct Wireframe {
    key: KeyCode
}
impl Wireframe {
    pub fn new(key: KeyCode) -> Wireframe {
        Wireframe{key}
    }
}

impl Plugin for Wireframe {
    fn build(&self, app: &mut App) {
        app.insert_resource(*self);
        app.add_systems(Update, toggle_wireframe);
    }
}

fn toggle_wireframe(
    wireframe: Res<Wireframe>,
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(wireframe.key) {
        wireframe_config.global = !wireframe_config.global;
    }
}
