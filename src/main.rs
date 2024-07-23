use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
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
            CycleWavePlugin,
            PanCamera(MouseButton::Left),
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
    f: fn(f32) -> f32,
}

impl Node  {
    fn new(x: f32, y: f32, radius: f32, color: LinearRgba, f: fn(f32) -> f32) -> Self {
        Self { x, y, radius, color, f }
    }
}

fn spawn_circles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<WaveMaterial>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // Example circle data
    let nodes = vec![
        Node::new(0.0, 0.0, 50.0, LinearRgba::rgb(0.0, 1.0, 1.0), |x| (1.0-2.0*x).abs()),
        Node::new(30.0, 30.0, 75.0, LinearRgba::rgb(1.0, 0.0, 1.0), |x| (x*3.0).fract()),
        Node::new(-20.0, 20.0, 60.0, LinearRgba::rgb(1.0, 1.0, 0.0), |x| ((x*x).fract()*12345.0).fract()),
    ];

    for node in nodes {
        let mesh = Rectangle::default();
        let gen = |x: i32| ((node.f)(x as f32 / 1024.0).clamp(0.0,1.0) * 65535.0) as u16;
        let grayscale_data = (0..1024).map(gen).flat_map(u16::to_le_bytes).collect::<Vec<u8>>();
        let image = Image::new_fill(
            Extent3d {
                width: 1024,
                height: 1,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &grayscale_data,
            TextureFormat::R16Unorm,
            RenderAssetUsages::RENDER_WORLD,
        );
        let image_handle = textures.add(image);
    
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(mesh)),
            transform: Transform::from_translation(Vec3::new(node.x, node.y, 0.0)).with_scale(Vec3::splat(node.radius)),
            material: materials.add(WaveMaterial::new(node.color, image_handle)),
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

