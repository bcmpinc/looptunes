use bevy::math::Vec3;
use bevy::sprite::Mesh2dHandle;
use bevy::transform::components::Transform;
use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::window::{CursorIcon, PrimaryWindow, Window};

mod pancamera; use pancamera::*;
mod cyclewave; use cyclewave::*;
mod wireframe; use wireframe::*;
mod micetrack; use micetrack::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins((
            DefaultPlugins,
            Wireframe(KeyCode::Space),
            PanCamera(MouseButton::Right),
            CycleWavePlugin,
            MiceTrack,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_cyclewaves)
        .add_systems(Update, hover_cycle)
        .run();
}

fn setup(
    mut commands: Commands, 
    mut windows: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        MousePos::default(),
    ));
    
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Pointer;

    commands.spawn((
        ColorMesh2dBundle{
            mesh: Mesh2dHandle(meshes.add(Annulus::new(1.00, 1.01).mesh().resolution(16))),
            material: materials.add(ColorMaterial::from_color(Color::linear_rgb(0.5, 0.5, 0.5))),
            visibility: Visibility::Hidden,
            ..default()
        },
        Highlight,
    ));
}

fn hover_cycle(
    cycles: Query<&Transform, (With<Cycle>, Without<Highlight>)>,
    mut hover: Query<(&mut Transform, &mut Visibility), (With<Highlight>, Without<Cycle>)>,
    camera: Query<&Camera>,
) {
    let cam = camera.single();

}

#[derive(Component)]
struct Highlight;

struct Node {
    x: f32,
    y: f32,
    radius: f32,
    color: LinearRgba,
    f: fn(f32) -> f32,
    freq: u32,
}

impl Node  {
    fn new(x: f32, y: f32, radius: f32, color: LinearRgba, f: fn(f32) -> f32, freq: u32) -> Self {
        Self { x, y, radius, color, f, freq }
    }
}

fn spawn_cyclewaves(
    mut commands: Commands,
) {
    // Example circle data
    let nodes = vec![
        Node::new(0.0, 0.0, 50.0, LinearRgba::rgb(0.0, 1.0, 1.0), Wave::TRIANGLE, 3),
        Node::new(30.0, 30.0, 75.0, LinearRgba::rgb(1.0, 0.0, 1.0), Wave::SAWTOOTH, 4),
        Node::new(-20.0, 20.0, 60.0, LinearRgba::rgb(1.0, 1.0, 0.0), Wave::NOISE, 5),
        Node::new(20.0, -50.0, 30.0, LinearRgba::rgb(0.2, 1.0, 0.2), Wave::SQUARE, 6),
        Node::new(60.0, 10.0, 25.0, LinearRgba::rgb(1.0, 0.5, 0.1), Wave::SINE, 20),
    ];

    for node in nodes {
        commands.spawn(CycleWaveBundle{
            cycle: Cycle{
                color: node.color,
                frequency: node.freq,
                ..Default::default()
            },
            wave: Wave::new(node.f),
            transform: Transform::from_translation(Vec3::new(node.x, node.y, 0.0)).with_scale(Vec3::splat(node.radius)),
            ..default()
        });
    }
}

