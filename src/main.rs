use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig, Wireframe2dPlugin};
use bevy::transform::components::Transform;
use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::window::{CursorIcon, Window};

mod pancamera; use pancamera::*;
mod material;  use material::*;
mod wireframe; use wireframe::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Wireframe::new(KeyCode::Space),
            Wireframe2dPlugin,
            Materials,
            PanCamera,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_circles)
        .add_systems(Update, toggle_circles)
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
    color: LinearRgba,
}

impl Node  {
    fn new(x: f32, y: f32, radius: f32, color: LinearRgba) -> Self {
        Self { x, y, radius, color }
    }
}

fn spawn_circles(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<FancyCircleMaterial>>) {
    // Example circle data
    let nodes = vec![
        Node::new(0.0, 0.0, 50.0, LinearRgba::rgb(0.0, 1.0, 1.0)),
        Node::new(100.0, 100.0, 75.0, LinearRgba::rgb(1.0, 0.0, 1.0)),
        Node::new(-150.0, -150.0, 30.0, LinearRgba::rgb(1.0, 1.0, 0.0)),
    ];

    for node in nodes {
        let mesh = Rectangle::default();
        
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(mesh)),
            transform: Transform::from_translation(Vec3::new(node.x, node.y, 0.0)).with_scale(Vec3::splat(node.radius)),
            material: materials.add(FancyCircleMaterial::new(node.color, 0.2)),
            ..Default::default()
        });
    }
}


fn toggle_circles(
    mut query : Query<&mut Mesh2dHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for mut mesh in query.iter_mut() {
        if keyboard.just_pressed(KeyCode::KeyR) {
            *mesh = Mesh2dHandle(meshes.add(Rectangle::default()));
        }
        if keyboard.just_pressed(KeyCode::KeyC) {
            *mesh = Mesh2dHandle(meshes.add(Circle::default()));
        }
    }
}

