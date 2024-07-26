use std::f32::consts::{PI, TAU};
use std::mem::swap;

use bevy::input::mouse::MouseWheel;
use bevy::math::Vec3;
use bevy::sprite::Mesh2dHandle;
use bevy::transform::components::Transform;
use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::window::{CursorIcon, PrimaryWindow, Window};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

mod connector; use connector::*;
mod cyclewave; use cyclewave::*;
mod looptunes; use looptunes::*; 
mod micetrack; use micetrack::*;
mod pancamera; use pancamera::*;
mod wireframe; use wireframe::*;

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
            EmbeddedAssetPlugin{mode: PluginMode::ReplaceDefault},
            DefaultPlugins.set(
                AssetPlugin{meta_check: bevy::asset::AssetMetaCheck::Never, ..default()},
            ),
            Wireframe(KeyCode::Space),
            PanCamera(MouseButton::Right),
            CycleWavePlugin,
            MiceTrack,
            LoopTunes,
            ConnectorPlugin,
        ))
        .add_systems(Startup, (setup, set_window_title))
        .add_systems(Startup, spawn_cyclewaves)
        .add_systems(Update, (hover_cycle, drag_cycle, draw_cycle, scroll_cycle.run_if(is_shift)).chain())
        .insert_resource(Hover::default())
        .configure_sets(Update, (ZoomSystem).run_if(is_not_shift))
        .add_systems(PostUpdate, play_anything.run_if(backend_has_capacity))
        .insert_resource(PlayPosition(0))
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle{
        transform: Transform::from_scale(Vec3::new(0.01,0.01,1.0)),
        ..default()
    });
    
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

fn set_window_title(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.title = "Loop Tunes".to_string();
    } 
}

#[derive(Resource)]
pub struct Hover {
    pub entity: Option<Entity>,
    pub position: Vec2,
    pub old_position: Vec2,
    pub pressed: bool,
}

impl Default for Hover {
    fn default() -> Self {
        Self { 
            entity: default(), 
            position: default(), 
            old_position: default(), 
            pressed: default(), 
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
    mut windows: Query<&mut Window>,
) {
    // Don't update higlight while button is pressed.
    hover_entity.pressed = buttons.pressed(MouseButton::Left);
    if hover_entity.pressed {return}
    
    
    let (h_entity, mut h_vis) = hover.single_mut();
    let mut nearest = f32::INFINITY;
    
    // Reset hover entity
    *h_vis = Visibility::Hidden;
    hover_entity.entity = None;

    // Reset mouse cursor
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Default;
    if !mouse.on_screen {return}

    // Find "nearest" circle and move hover entity towards it.
    for (entity, cycle_pos) in cycles.iter() {
        let translation = cycle_pos.translation();
        let scale = cycle_pos.affine().x_axis[0] / 2.0;
        let pos = (mouse.position - translation.xy()) / scale;
        let dist = pos.length();
        let score = (dist + 1.0) * scale;
        if dist < 1.05 && nearest > score {
            *h_vis = Visibility::Visible;
            commands.entity(h_entity).set_parent(entity);
            nearest = score;
            hover_entity.entity = Some(entity);
            hover_entity.position = pos;
            if dist < 0.45 {
                window.cursor.icon = CursorIcon::Grab;
            } else {
                window.cursor.icon = CursorIcon::Crosshair;
            }
        }
    }
}

fn drag_cycle(
    mut q_cycles: Query<&mut Transform, (With<Cycle>,Without<Camera2d>)>,
    q_camera: Query<&Transform, (With<Camera2d>, Without<Cycle>)>,
    mut motion: EventReader<CursorMoved>,
    hover_entity: Res<Hover>,
    mut windows: Query<&mut Window>,
) {
    if !hover_entity.pressed {return}

    let mut window = windows.single_mut();
    if window.cursor.icon == CursorIcon::Grab {
        window.cursor.icon = CursorIcon::Grabbing; 
    }
    if window.cursor.icon != CursorIcon::Grabbing {return}

    let Some(cycle_id) = hover_entity.entity else {return};
    let Ok(mut cycle) = q_cycles.get_mut(cycle_id) else {return};
    let scale = q_camera.single().scale.x;
    for event in motion.read() {
        if let Some(offset) = event.delta {
            cycle.translation += Vec3::new(offset.x * scale, offset.y * -scale, 0.0);
        }
    }
}

fn get_index(pos: Vec2) -> usize {
    (1024.0 * (PI + f32::atan2(pos.x, -pos.y)) / TAU) as usize
}

fn draw_cycle(
    mut q_cycles: Query<(&Transform, &mut Wave), (With<Cycle>,Without<Camera2d>)>,
    mut hover_entity: ResMut<Hover>,
    windows: Query<&mut Window>,
    mouse: Res<MousePos>,
) {
    if !hover_entity.pressed {return}
    let window = windows.single();
    if window.cursor.icon != CursorIcon::Crosshair {return}

    let Some(cycle_id) = hover_entity.entity else {return};
    let Ok((cycle, mut wave)) = q_cycles.get_mut(cycle_id) else {return};
    
    let translation = cycle.translation;
    let scale = cycle.scale.x / 2.0;
    let pos = (mouse.position - translation.xy()) / scale;
    let a = hover_entity.position;
    let b = pos;

    // Line drawing algorithm
    // println!("Draw {:?} to {:?}", a, b);
    let mut ia = get_index(a);
    let mut ib = get_index(b);
    let mut va = a.length() * 2.0 - 1.0;
    let mut vb = b.length() * 2.0 - 1.0;
    if ia > ib + 512 {
        ib += 1024;
    }
    if ib > ia + 512 {
        ia += 1024;
    }
    if ib < ia {
        swap(&mut ia, &mut ib);
        swap(&mut va, &mut vb);
    }
    if ia == ib {
        wave.pattern[ia] = vb;
    } else {
        assert!(ia < ib);
        for i in ia..=ib {
            let value = va + (vb-va) * (i-ia) as f32 / (ib-ia) as f32;
            wave.pattern[i % 1024] = value.clamp(0.0,1.0);
        }
    }

    hover_entity.old_position = a;
    hover_entity.position = b;
}

fn scroll_cycle(
    mut q_cycles: Query<&mut Cycle>,
    hover_entity: Res<Hover>,
    mut scroll: EventReader<MouseWheel>,
) {
    let Some(entity) = hover_entity.entity else {return};
    let Ok(mut cycle) = q_cycles.get_mut(entity) else {return};
    for event in scroll.read() {
        cycle.change_frequency(-event.y.signum() as i32);
    }
}

#[derive(Component)]
struct Highlight;

#[derive(Resource)]
struct PlayPosition(u32);

fn play_anything(
    q_cycles: Query<(&Cycle,&Wave)>,
    hover_entity: Res<Hover>,
    backend: Res<LoopTunesBackend>,
    mut pos: ResMut<PlayPosition>,
) {
    let Some(entity) = hover_entity.entity else {return};
    let Ok((cycle, wave)) = q_cycles.get(entity) else {return};
    for i in 0..PLAY_CHUNK as u32 {
        let t = (pos.0 + i) as f64 / 48000.0;
        let wave_pos = t * cycle.frequency();
        let index = (wave_pos.fract() * 1024.0) as usize;
        let sample = wave.pattern[index] - wave.average;
        _ = backend.producer.send(sample * 0.2);
    }
    pos.0 += PLAY_CHUNK as u32;
    pos.0 %= 48000 * 256;
}


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
        Node::new(-2.0, 0.0, LinearRgba::rgb(0.0, 1.0, 1.0), Wave::TRIANGLE, 63),
        Node::new(-1.0, 0.0, LinearRgba::rgb(1.0, 0.0, 1.0), Wave::SAWTOOTH, 63),
        Node::new( 0.0, 0.0, LinearRgba::rgb(1.0, 1.0, 0.0), Wave::NOISE, 63),
        Node::new( 1.0, 0.0, LinearRgba::rgb(0.2, 1.0, 0.2), Wave::SQUARE, 63),
        Node::new( 2.0, 0.0, LinearRgba::rgb(1.0, 0.5, 0.1), Wave::SINE, 63),
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

    commands.spawn(Segment {
        source: Vec2::new(-2.0,0.0),
        target: Vec2::new(0.0,2.0),
        source_size: 0.1,
        target_size: 0.1,
    });

    commands.spawn(Segment {
        source: Vec2::new(2.0,0.0),
        target: Vec2::new(0.0,2.0),
        source_size: 0.1,
        target_size: 0.1,
    });

}

