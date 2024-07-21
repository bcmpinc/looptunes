use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig, Wireframe2dPlugin};
use bevy::transform::components::Transform;
use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::window::{CursorIcon, Window};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, 
            Wireframe2dPlugin,
            Material2dPlugin::<FancyCircleMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_circles)
        .add_systems(Update, toggle_wireframe)
        .add_systems(Update, toggle_circles)
        .run();
}

fn setup(mut commands: Commands, mut windows: Query<&mut Window>) {
    commands.spawn(Camera2dBundle::default());
    
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Pointer;
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct FancyCircleMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    width: f32,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for FancyCircleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circle.wgsl".into()
    }
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

fn spawn_circles(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<FancyCircleMaterial>>) {
    // Example circle data
    let nodes = vec![
        Node::new(0.0, 0.0, 50.0),
        Node::new(100.0, 100.0, 75.0),
        Node::new(-150.0, -150.0, 30.0),
    ];

    for node in nodes {
        let mesh = Rectangle::default();
        
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(mesh)),
            transform: Transform::from_translation(Vec3::new(node.x, node.y, 0.0)).with_scale(Vec3::splat(node.radius)),
            material: materials.add(FancyCircleMaterial {
                color: LinearRgba::rgb(0.0, 1.0, 1.0),
                width: 0.1,
            }),
            ..Default::default()
        });
    }
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
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
