use bevy::math::Vec3;
use bevy::transform::components::Transform;
use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::window::{CursorIcon, Window};

mod pancamera; use pancamera::*;
mod cyclewave;  use cyclewave::*;
mod wireframe; use wireframe::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins((
            DefaultPlugins,
            Wireframe(KeyCode::Space),
            PanCamera(MouseButton::Right),
            CycleWavePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_cyclewaves)
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
    f: fn(f32) -> f32,
}

impl Node  {
    fn new(x: f32, y: f32, radius: f32, color: LinearRgba, f: fn(f32) -> f32) -> Self {
        Self { x, y, radius, color, f }
    }
}

fn spawn_cyclewaves(
    mut commands: Commands,
) {
    // Example circle data
    let nodes = vec![
        Node::new(0.0, 0.0, 50.0, LinearRgba::rgb(0.0, 1.0, 1.0), |x| (1.0-2.0*x).abs()),
        Node::new(30.0, 30.0, 75.0, LinearRgba::rgb(1.0, 0.0, 1.0), |x| (x*3.0).fract()),
        Node::new(-20.0, 20.0, 60.0, LinearRgba::rgb(1.0, 1.0, 0.0), |x| ((x*x).fract()*12345.0).fract()),
    ];

    for node in nodes {
        commands.spawn(CycleWaveBundle{
            cycle: Cycle{
                color: node.color,
                ..Default::default()
            },
            wave: Wave::new(node.f),
            transform: Transform::from_translation(Vec3::new(node.x, node.y, 0.0)).with_scale(Vec3::splat(node.radius)),
            ..default()
        });
    }
}

