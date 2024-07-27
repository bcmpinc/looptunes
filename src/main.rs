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
use bevy::window::{CursorIcon, PresentMode, Window, WindowTheme};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

use rand::{thread_rng, Rng};

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
            DefaultPlugins.set(AssetPlugin{
                meta_check: bevy::asset::AssetMetaCheck::Never, ..default()
            }).set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Loop Tunes!".into(),
                    present_mode: PresentMode::AutoNoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            Wireframe(KeyCode::Space),
            PanCamera(MouseButton::Right),
            CycleWavePlugin,
            MiceTrack,
            LoopTunes,
            ConnectorPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_cyclewaves)
        .add_systems(Update, (hover_cycle, delete_circle, clone_circle, drag_cycle, draw_cycle, scroll_cycle.run_if(is_shift)).chain())
        .add_systems(Update, (colorize, add_circle))
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
            mesh: Mesh2dHandle(meshes.add(Annulus::new(0.54, 0.55).mesh().resolution(16))),
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
    keyboard: Res<ButtonInput<KeyCode>>,
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
            if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
                window.cursor.icon = CursorIcon::Copy;
            } else if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
                window.cursor.icon = CursorIcon::Pointer;
            } else if dist < 0.45 || scale / mouse.zoom < 200.0 {
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

fn clone_circle(
    mut commands: Commands,
    q_cycles: Query<(&Cycle, &Wave)>,
    mut hover_entity: ResMut<Hover>,
    mouse: Res<MousePos>,
    mut windows: Query<&mut Window>,
) {
    if !hover_entity.pressed {return}
    let Some(ent) = hover_entity.entity else {return};
    let Ok((cycle, wave)) = q_cycles.get(ent) else {return};

    let mut window = windows.single_mut();
    if window.cursor.icon != CursorIcon::Copy {return}
    window.cursor.icon = CursorIcon::Grabbing; 

    let entity = commands.spawn(CycleWaveBundle{
        cycle: cycle.clone(),
        wave: Wave{
            pattern: wave.pattern.clone(),
            ..default()
        },
        transform: Transform::from_translation(mouse.position.extend(0.0)),
        ..default()
    });

    hover_entity.entity = Some(entity.id());
}

fn delete_circle(
    mut commands: Commands,
    mut hover_res: ResMut<Hover>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hover: Query<(Entity, &mut Visibility), With<Highlight>>,
) {
    if !keyboard.just_pressed(KeyCode::Delete) {return}
    let Some(entity) = hover_res.entity else {return};
    // println!("Deleting {:?}", entity);
    let (hover_entity, mut hover_visible) = hover.single_mut();
    commands.entity(hover_entity).remove_parent();
    *hover_visible = Visibility::Hidden;
    commands.entity(entity).despawn_recursive();
    hover_res.entity = None;
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
    pos.0 %= 48000 * 256 * 3;
}

fn spawn_cyclewaves(
    mut commands: Commands,
) {
    // Example circle data
    let nodes = vec![
        (-3.0, 0.0, LinearRgba::rgb(0.0, 1.0, 1.0), Wave::TRIANGLE),
        (-1.5, 0.0, LinearRgba::rgb(1.0, 0.0, 1.0), Wave::SAWTOOTH),
        ( 0.0, 0.0, LinearRgba::rgb(1.0, 1.0, 0.0), Wave::NOISE),
        ( 1.5, 0.0, LinearRgba::rgb(0.2, 1.0, 0.2), Wave::SQUARE),
        ( 3.0, 0.0, LinearRgba::rgb(1.0, 0.5, 0.1), Wave::SINE),
    ];

    let mut segment = commands.spawn(Segment {
        source: Vec2::new(0.0,-2.0),
        target: Vec2::new(0.0,2.0),
        ..default()
    }).id();

    for (x,y,color,f) in nodes {
        let new_segment = commands.spawn(Segment {
            source: Vec2::new(0.0,-2.0),
            target: Vec2::new(0.0,2.0),
            ..default()
        }).id();

        commands.spawn(CycleWaveBundle{
            cycle: Cycle{
                color: color,
                frequency: Cycle::NOTE_A4,
                ..Default::default()
            },
            wave: Wave::new(f),
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            ..default()
        }).with_children(|parent|{
            parent.spawn(Bow(segment));
            parent.spawn(Arrow(new_segment));
        });
        segment = new_segment;
    }
}

fn colorize(
    hover_entity: Res<Hover>,
    mut q_cycles: Query<&mut Cycle>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.pressed(KeyCode::KeyC) {return}
    let Some(ent) = hover_entity.entity else {return};
    let Ok(mut cycle) = q_cycles.get_mut(ent) else {return};
    let hue = (hover_entity.position.to_angle() + PI) / TAU;
    cycle.color = Color::hsv(360.0 * hue, 1.0, 1.0).into();
}

fn add_circle(
    mut commands: Commands,
    mouse: Res<MousePos>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let function = match () {
        _ if keyboard.just_pressed(KeyCode::Digit1) => Wave::SINE,
        _ if keyboard.just_pressed(KeyCode::Digit2) => Wave::TRIANGLE,
        _ if keyboard.just_pressed(KeyCode::Digit3) => Wave::SAWTOOTH,
        _ if keyboard.just_pressed(KeyCode::Digit4) => Wave::SQUARE,
        _ if keyboard.just_pressed(KeyCode::Digit5) => Wave::NOISE,
        _ if keyboard.just_pressed(KeyCode::Digit6) => |v: f32| if v < 0.25 {1.0} else {0.0},
        _ if keyboard.just_pressed(KeyCode::Digit7) => |v: f32| if v < 0.125 {1.0} else {0.0},
        _ if keyboard.just_pressed(KeyCode::Digit8) => |v: f32| f32::exp(-4.0 * v),
        _ if keyboard.just_pressed(KeyCode::Digit9) => |v: f32| f32::clamp(1.0 - f32::abs(1.0 - 4.0*v), 0.0, 1.0),
        _ if keyboard.just_pressed(KeyCode::Digit0) => |v: f32| f32::clamp(2.0 - f32::abs(2.0 - 8.0*v), 0.0, 1.0),
        _ => return
    };

    let frequency = if is_shift(keyboard) {
        Cycle::DEFAULT_FREQUENCY
    } else {
        Cycle::NOTE_A4
    };

    commands.spawn(CycleWaveBundle{
        cycle: Cycle {
            color: Color::hsv(thread_rng().gen_range(0.0..360.0), 1.0, 1.0).into(),
            frequency: frequency,
            ..Default::default()
        },
        wave: Wave::new(function),
        transform: Transform::from_translation(mouse.position.extend(0.0)),
        ..default()
    });
}
