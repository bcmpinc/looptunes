use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::sprite::{ColorMesh2dBundle, Mesh2dHandle};
use bevy::transform::components::Transform;
use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::window::{CursorIcon, Window};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_circles)
        .run();
}

fn setup(mut commands: Commands, mut windows: Query<&mut Window>) {
    commands.spawn(Camera2dBundle::default());
    
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Pointer;
}

struct Node {
    x: f32,
    y: f32,
    radius: f32,
}

impl Node  {
    fn new(x: f32, y: f32, radius: f32) -> Self {
        Self { x, y, radius }
    }
}

fn spawn_circles(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Example circle data
    let nodes = vec![
        Node::new(0.0, 0.0, 50.0),
        Node::new(100.0, 100.0, 75.0),
        Node::new(-150.0, -150.0, 30.0),
    ];

    for node in nodes {
        let mesh = Circle::new(node.radius);

        commands.spawn(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(mesh)),
            transform: Transform::from_translation(Vec3::new(node.x, node.y, 0.0)),
            material: materials.add(Color::linear_rgb(0.0, 1.0, 1.0)),
            ..Default::default()
        });
    }
}


