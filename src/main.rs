use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::system::Commands;
use bevy::prelude::Query;
use bevy::window::{CursorIcon, Window};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut windows: Query<&mut Window>) {
    commands.spawn(Camera2dBundle::default());
    
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Pointer;
}
