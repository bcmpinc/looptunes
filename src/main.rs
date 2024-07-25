use core::f32;

use bevy::input::mouse::MouseWheel;
use bevy::math::Vec3;
use bevy::sprite::Mesh2dHandle;
use bevy::transform::components::Transform;
use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::window::{CursorIcon, Window};

mod pancamera; use pancamera::*;
mod cyclewave; use cyclewave::*;
mod wireframe; use wireframe::*;
mod micetrack; use micetrack::*;

fn is_shift(keyboard: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard.pressed(KeyCode::ShiftLeft)  || keyboard.pressed(KeyCode::ShiftRight)
} 
fn is_not_shift(keyboard: Res<ButtonInput<KeyCode>>) -> bool {
    !is_shift(keyboard)
}

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
        .add_systems(Update, (hover_cycle, drag_cycle, scroll_cycle.run_if(is_shift)).chain())
        .insert_resource(Hover::default())
        .configure_sets(Update, (ZoomSystem).run_if(is_not_shift))
        .run();
}

fn setup(
    mut commands: Commands, 
    mut windows: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Pointer;

    commands.spawn((
        ColorMesh2dBundle{
            mesh: Mesh2dHandle(meshes.add(Annulus::new(0.53, 0.55).mesh().resolution(16))),
            material: materials.add(ColorMaterial::from_color(Color::WHITE)),
            visibility: Visibility::Hidden,
            ..default()
        },
        Highlight,
    ));
}

#[derive(Resource)]
pub struct Hover {
    pub entity: Option<Entity>,
    pub pressed: bool,
}

impl Default for Hover {
    fn default() -> Self {
        Self { 
            entity: Default::default(), 
            pressed: Default::default(),
        }
    }
}

fn hover_cycle(
    mut commands: Commands, 
    cycles: Query<(Entity, &GlobalTransform), (With<Cycle>, Without<Highlight>)>,
    mut hover: Query<(Entity, &mut Visibility), (With<Highlight>, Without<Cycle>)>,
    mouse: Res<MousePos>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut hover_entity: ResMut<Hover>,
) {
    // Don't update higlight while button is pressed.
    hover_entity.pressed = buttons.pressed(MouseButton::Left);
    if hover_entity.pressed {return;}

    let (h_entity, mut h_vis) = hover.single_mut();
    let mut nearest = f32::INFINITY;

    // Reset hover entity
    *h_vis = Visibility::Hidden;
    hover_entity.entity = None;
    if !mouse.on_screen {return;}

    // Find "nearest" circle and move hover entity towards it.
    for (entity, cycle_pos) in cycles.iter() {
        let translation = cycle_pos.translation();
        let scale = cycle_pos.affine().x_axis[0] / 2.0;
        let dist = (mouse.position - translation.xy()).length();
        if dist < scale && nearest > dist + scale {
            *h_vis = Visibility::Visible;
            commands.entity(h_entity).set_parent(entity);
            nearest = dist + scale;
            hover_entity.entity = Some(entity);
        }
    }
}

fn drag_cycle(
    mut q_cycles: Query<&mut Transform, (With<Cycle>,Without<Camera2d>)>,
    q_camera: Query<&Transform, (With<Camera2d>, Without<Cycle>)>,
    mut motion: EventReader<CursorMoved>,
    hover_entity: Res<Hover>,
) {
    if !hover_entity.pressed {return}
    let Some(cycle_id) = hover_entity.entity else {return};
    let Ok(mut cycle) = q_cycles.get_mut(cycle_id) else {return};
    let scale = q_camera.single().scale.x;
    for event in motion.read() {
        if let Some(offset) = event.delta {
            cycle.translation += Vec3::new(offset.x * scale, offset.y * -scale, 0.0);
        }
    }
}

fn scroll_cycle(
    mut q_cycles: Query<(&mut Cycle)>,
    hover_entity: Res<Hover>,
    mut scroll: EventReader<MouseWheel>,

) {
    let Some(entity) = hover_entity.entity else {return};
    let Ok(mut cycle) = q_cycles.get_mut(entity) else {return};
    for event in scroll.read() {
        cycle.change_frequency(-event.y as i32);
    }
}

#[derive(Component)]
struct Highlight;

struct Node {
    x: f32,
    y: f32,
    color: LinearRgba,
    f: fn(f32) -> f32,
    freq: u32,
}

impl Node  {
    fn new(x: f32, y: f32, color: LinearRgba, f: fn(f32) -> f32, freq: u32) -> Self {
        Self { x, y, color, f, freq }
    }
}

fn spawn_cyclewaves(
    mut commands: Commands,
) {
    // Example circle data
    let nodes = vec![
        Node::new(0.0, 0.0, LinearRgba::rgb(0.0, 1.0, 1.0), Wave::TRIANGLE, 3),
        Node::new(30.0, 30.0, LinearRgba::rgb(1.0, 0.0, 1.0), Wave::SAWTOOTH, 4),
        Node::new(-20.0, 20.0, LinearRgba::rgb(1.0, 1.0, 0.0), Wave::NOISE, 5),
        Node::new(30.0, 30.0, LinearRgba::rgb(0.2, 1.0, 0.2), Wave::SQUARE, 6),
        Node::new(60.0, 10.0, LinearRgba::rgb(1.0, 0.5, 0.1), Wave::SINE, 20),
    ];

    for node in nodes {
        commands.spawn(CycleWaveBundle{
            cycle: Cycle{
                color: node.color,
                frequency: node.freq,
                ..Default::default()
            },
            wave: Wave::new(node.f),
            transform: Transform::from_translation(Vec3::new(node.x, node.y, 0.0)),
            ..default()
        });
    }
}

