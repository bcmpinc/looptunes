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
use smallvec::SmallVec;

// Modules
mod connector; use connector::*;
mod cyclewave; use cyclewave::*;
mod looptunes; use looptunes::*; 
mod micetrack; use micetrack::*;
mod pancamera; use pancamera::*;
mod utilities; use utilities::*;

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
                    prevent_default_event_handling: true,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            PanCamera(MouseButton::Right),
            CycleWavePlugin,
            MiceTrack,
            LoopTunes,
            ConnectorPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_cyclewaves)
        .add_systems(PreUpdate, child_cycles)
        .add_systems(Update, (
            hover_cycle, 
            connect_create,
            (delete_circle, clone_circle, drag_cycle, draw_cycle, connect_cycle, scroll_cycle.run_if(is_shift)),
            connect_drop
        ).chain())
        .add_systems(Update, (colorize, add_circle))
        .configure_sets(Update, (ZoomSystem).run_if(is_not_shift))
        .add_systems(PostUpdate, play_everything)
        .add_systems(SpawnScene, track_hover)
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle{
        transform: Transform::from_scale(Vec3::new(0.02,0.02,1.0)),
        ..default()
    });
    
    let highlight = commands.spawn((
        ColorMesh2dBundle{
            mesh: Mesh2dHandle(meshes.add(Annulus::new(1.08, 1.10).mesh().resolution(16))),
            material: materials.add(ColorMaterial::from_color(Color::WHITE)),
            visibility: Visibility::Hidden,
            ..default()
        },
        Highlight,
    )).id();

    commands.insert_resource(Hover{
        entity: None,
        position: default(), 
        old_position: default(), 
        pressed: default(),
        highlight,
    });
}

#[derive(Resource)]
pub struct Hover {
    pub entity: Option<Entity>,
    pub position: Vec2,
    pub old_position: Vec2,
    pub pressed: bool,
    pub highlight: Entity,
}

/** Moves the Higlight entity to whatever the Hover resource points to. */
fn track_hover(
    mut commands: Commands, 
    mut entity: Query<(&mut Visibility, &mut Transform, Option<&Parent>), With<Highlight>>,
    cycles: Query<&Cycle>,
    hover: Res<Hover>,
){
    use Visibility::*;
    let (mut visible, mut transform, parent) = entity.get_mut(hover.highlight).unwrap();
    match hover.entity {
        Some(hover_entity) => {
            let scale = cycles.get(hover_entity).unwrap().scale();
            transform.scale = Vec3::new(scale, scale, 1.0);
            if parent == None || parent.unwrap().get() != hover_entity {
                assert!(hover.highlight != hover_entity);
                commands.entity(hover.highlight).set_parent(hover_entity);
            }
            if *visible != Visible {
                *visible = Visible;
            }
        },
        None if *visible != Hidden => {
            commands.entity(hover.highlight).remove_parent();
            *visible = Hidden;
        },
        _ => {}
    }
}

fn nearest_circle(
    cycles: &Query<(Entity, &mut Cycle, &GlobalTransform)>,
    mouse: Res<MousePos>,
) -> Option<(Entity, Vec2, bool)> {
    let mut res: Option<(Entity, Vec2, bool)> = None;
    let mut nearest = f32::INFINITY;

    // Find "nearest" circle.
    for (entity, cycle, transform) in cycles.iter() {
        let translation = transform.translation();
        let scale = cycle.scale();
        let pos = (mouse.position - translation.xy()) / scale;
        let dist = pos.length();
        let score = (dist + 1.0) * scale;
        if dist < 1.05 && nearest > score {
            nearest = score;
            res = Some((entity, pos, dist > 0.45 && scale / mouse.zoom > 200.0));
        }
    }
    res
}

fn hover_cycle(
    cycles: Query<(Entity, &mut Cycle, &GlobalTransform)>, // Does not need to be mut, but function parameter type checking...
    mouse: Res<MousePos>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut hover: ResMut<Hover>,
    mut windows: Query<&mut Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Don't update higlight while button is pressed.
    hover.pressed = buttons.pressed(MouseButton::Left);
    if hover.pressed {return}

    // Reset hover entity.
    hover.entity = None;
    
    // Reset mouse cursor
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Default;
    if !mouse.on_screen {return}
    
    // Find "nearest" circle and move hover entity towards it.
    let Some((entity, position, draw)) = nearest_circle(&cycles, mouse) else {return};
    hover.entity = Some(entity);
    hover.position = position;
    if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
        window.cursor.icon = CursorIcon::Copy;
    } else if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
        window.cursor.icon = CursorIcon::Pointer;
    } else if draw {
        window.cursor.icon = CursorIcon::Crosshair;
    } else {
        window.cursor.icon = CursorIcon::Grab;
    }
}

fn drag_cycle(
    mut q_cycles: Query<&mut Transform, (With<Cycle>,Without<Camera2d>)>,
    q_camera: Query<&Transform, (With<Camera2d>, Without<Cycle>)>,
    mut motion: EventReader<CursorMoved>,
    hover: Res<Hover>,
    mut windows: Query<&mut Window>,
) {
    if !hover.pressed {return}

    let mut window = windows.single_mut();
    if window.cursor.icon == CursorIcon::Grab {
        window.cursor.icon = CursorIcon::Grabbing; 
    }
    if window.cursor.icon != CursorIcon::Grabbing {return}

    let Some(cycle_id) = hover.entity else {return};
    let Ok(mut cycle) = q_cycles.get_mut(cycle_id) else {return};
    let scale = q_camera.single().scale.x;
    for event in motion.read() {
        if let Some(offset) = event.delta {
            cycle.translation += Vec3::new(offset.x * scale, offset.y * -scale, 0.0);
        }
    }
}

/** Checks whether child is an (grand-)*child of parent. */
fn has_parent(parents: &Query<&Parent>, mut child: Entity, parent: Entity) -> bool {
    loop {
        if child == parent {return true}
        match parents.get(child) {
            Ok(e) => {child = e.get();},
            Err(_) => return false,
        }
    }
}

fn connect_create(
    mut commands: Commands,
    windows: Query<&Window>,
    hover: Res<Hover>,
    mut connector: ResMut<Connector>,
    segments: Query<(Entity,&Segment)>,
) {
    if !hover.pressed {return}
    if windows.single().cursor.icon != CursorIcon::Pointer {return}

    if connector.0.is_some() {return}
    let Some(child_cycle) = hover.entity else {return};
    for (ent, seg) in segments.iter() {
        if seg.child_cycle == child_cycle {
            // Repurpose existing connector.
            connector.0 = Some(ent);
            // Remove self from parent
            commands.entity(child_cycle).remove_parent_in_place().remove::<Playing>();
            return
        }
    }
    
    // Create a new connector.
    connector.0 = Some(Segment::spawn(&mut commands, child_cycle));
}

fn connect_cycle(
    mut windows: Query<&mut Window>,
    mut cycles: Query<(Entity, &mut Cycle, &GlobalTransform)>,
    mut hover: ResMut<Hover>,
    connector: Res<Connector>,
    mouse: Res<MousePos>,
    keyboard: Res<ButtonInput<KeyCode>>,
    parents: Query<&Parent>,
    mut segments: Query<&mut Segment>,
) {
    if !hover.pressed {return}
    let Some(mut segment) = connector_segment_mut(&connector, &mut segments) else {return};
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Pointer;

    // Attach arrow to hovered cycle.
    if let Some((entity, position, _draw)) = nearest_circle(&cycles, mouse) {
        let cc_id = segment.child_cycle;

        // Don't connect to *cc_id* or any of its children.
        if has_parent(&parents, entity, cc_id) {
            window.cursor.icon = CursorIcon::NotAllowed;
        } else {
            hover.entity = Some(entity);
            hover.position = position;
            segment.parent_cycle = Some(entity);
            let (_, mut cc, _) = cycles.get_mut(cc_id).unwrap();
            let mut phase = 1.25 - position.to_angle() / TAU;
            if is_shift(keyboard) {
                phase = (16.0 * phase).round() / 16.0;
            }
            cc.phase = phase % 1.0;
            return
        }
    }
    if hover.entity != None {
        hover.entity = None;
        segment.parent_cycle = None;
    }
}

/** Fires when the player releases the mouse button while dragging a connector. */
fn connect_drop(
    mut commands: Commands,
    hover: Res<Hover>,
    mut connector: ResMut<Connector>,
    segments: Query<&Segment>,
    parents: Query<&Parent>,
    playing: Query<(), With<Playing>>,
) {
    if hover.pressed {return}
    let Some(segment) = connector_segment(&connector, &segments) else {return};
    //print!("Dropping connector: ");
    
    let cc_id = segment.child_cycle;
    if let Some(parent) = hover.entity {
        if !has_parent(&parents, parent, cc_id) {
            //println!("attached to {:?}", parent);
            assert!(cc_id != parent);
            assert!(segment.parent_cycle != None);
            let mut ec = commands.entity(cc_id);
            ec.set_parent_in_place(parent);
            if playing.get(parent).is_ok() {
                ec.insert(Playing);
            }
            connector.0 = None;
            return;
        }
    }

    //println!("removing");
    // No new parent, delete the connector
    despawn_segment(&mut commands, connector.0.unwrap(), segment);
    connector.0 = None;
}

fn clone_circle(
    mut commands: Commands,
    q_cycles: Query<(&Cycle, &Wave)>,
    mut hover: ResMut<Hover>,
    mouse: Res<MousePos>,
    mut windows: Query<&mut Window>,
) {
    if !hover.pressed {return}
    let Some(ent) = hover.entity else {return};
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

    hover.entity = Some(entity.id());
}

fn clear_children_in_place(
    commands: &mut Commands,
    hover: &Res<Hover>,
    q_children: &Query<&Children>,
    parent: Entity,
) {
    if let Ok(children) = q_children.get(parent) {
        for child in children {
            if let Some(mut c) = commands.get_entity(*child) {
                if *child == hover.highlight {
                    c.remove_parent();
                } else {
                    c.remove_parent_in_place();
                    c.remove::<Playing>();
                }
            }
        }
    }
}

fn delete_circle(
    mut commands: Commands,
    mut hover: ResMut<Hover>,
    connector: Res<Connector>,
    q_children: Query<&Children>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::Delete) {return}
    if connector.0 != None {return}
    let Some(entity) = hover.entity else {return};
    hover.entity = None;
    clear_children_in_place(&mut commands, &hover.into(), &q_children, entity);
    commands.try_despawn(entity);
}

fn get_index(pos: Vec2) -> usize {
    (1024.0 * (PI + f32::atan2(pos.x, -pos.y)) / TAU) as usize
}

fn draw_cycle(
    mut q_cycles: Query<(&Cycle, &mut Wave, &GlobalTransform)>,
    mut hover: ResMut<Hover>,
    windows: Query<&mut Window>,
    mouse: Res<MousePos>,
) {
    if !hover.pressed {return}
    let window = windows.single();
    if window.cursor.icon != CursorIcon::Crosshair {return}

    let Some(cycle_id) = hover.entity else {return};
    let Ok((cycle, mut wave, transform)) = q_cycles.get_mut(cycle_id) else {return};
    
    let translation = transform.affine().translation;
    let scale = cycle.scale();
    let pos = (mouse.position - translation.xy()) / scale;
    let a = hover.position;
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

    hover.old_position = a;
    hover.position = b;
}

fn scroll_cycle(
    mut q_cycles: Query<&mut Cycle>,
    hover: Res<Hover>,
    mut scroll: EventReader<MouseWheel>,
) {
    let Some(entity) = hover.entity else {return};
    let Ok(mut cycle) = q_cycles.get_mut(entity) else {return};
    for event in scroll.read() {
        cycle.change_frequency(-event.y.signum() as i32);
    }
}

#[derive(Component)]
struct Highlight;

#[derive(Component)]
struct ChildCycles(SmallVec<[Entity; 8]>);

fn child_cycles(
    mut commands: Commands,
    q: Query<(Entity, Option<Ref<Children>>), With<Cycle>>,
    mut q_cc: Query<&mut ChildCycles, With<Cycle>>,
) {
    for (entity, children) in q.iter() {
        let childcycles = q_cc.get_mut(entity).ok();
        if let Some(childs) = children {
            if !childs.is_changed() {continue}

            let mut res = SmallVec::<[Entity; 8]>::new();
            for &e in childs.iter() {
                if q.get(e).is_ok() {
                    res.push(e);
                }
            }
            if !res.is_empty() {
                let component = ChildCycles(res);
                if let Some(mut cc) = childcycles {
                    *cc = component;
                } else {
                    commands.entity(entity).insert(component);
                }
                continue;
            }
        };

        if childcycles.is_some() {
            commands.entity(entity).remove::<ChildCycles>();
        }
    }
}

#[inline]
fn synthesize<'a>(cycle: &'a Cycle, wave: &'a Wave, time: impl Iterator<Item = &'a f64> + 'a, phase: f64) -> impl Iterator<Item = f32> + 'a {
    time.map(move |&t| {
        let wave_pos = t * cycle.frequency() - phase;
        let index = (wave_pos.fract() * 1024.0) as usize;
        wave.pattern[index]
    })
}

fn play_everything(
    q_cycles: Query<(&Cycle,&Wave,Option<&ChildCycles>), With<Playing>>,
    q_roots: Query<Entity, (Without<Parent>, With<Playing>)>,
    mut backend: ResMut<LoopTunesBackend>,
) {
    // Only produce when there is space in the buffer.
    if !backend.has_free_space() {return}
    
    // If nothing is playing reset playback.
    if q_roots.is_empty() {
        backend.reset();
        return
    }

    // Prepare a stack of nodes.
    struct Node {
        entity: Entity,
        volume: Vec<f32>,
    }
    let mut stack: Vec<Node> = Vec::with_capacity(32);
    for entity in q_roots.iter() {
        let volume = [0.2;1024].into();
        stack.push(Node{entity, volume});
    }

    // Collect the samples from each node
    let time: Vec<f64> = backend.time_chunk();
    let mut result: Vec<f32> = [0.0;1024].into();
    while let Some(node) = stack.pop() {
        let Ok((cycle, wave, option_children)) = q_cycles.get(node.entity) else {continue};
        if let Some(children) = option_children {
            // Recurse into child nodes
            for &child in children.0.iter() {
                // Get the child cycle, assuming it is being played.
                let Ok((child_cycle, _, _)) = q_cycles.get(child) else {continue};

                // Mix this node!
                let child_volume 
                    = synthesize(&cycle, &wave, time.iter(), child_cycle.phase_in_parent() as f64)
                    .zip(node.volume.iter())
                    .map(|(s,v)| s*v);

                stack.push(Node { 
                    entity: child,
                    volume: child_volume.collect(),
                });
            }
        } else {
            // Play this node!
            let samples 
                = synthesize(&cycle, &wave, time.iter(), 0.0)
                .zip(node.volume.iter())
                .map(|(s,v)| (s - wave.average)*v);

            result
                .iter_mut()
                .zip(samples)
                .for_each(|(r, s)| *r += s);
        }
    }

    backend.send_buffer(&result);
}

fn spawn_cyclewaves(
    mut commands: Commands,
) {
    // Example circle data
    let nodes = vec![
        (-6.0, 0.0, LinearRgba::rgb(0.0, 1.0, 1.0), Wave::TRIANGLE),
        (-3.0, 0.0, LinearRgba::rgb(1.0, 0.0, 1.0), Wave::SAWTOOTH),
        ( 0.0, 0.0, LinearRgba::rgb(1.0, 1.0, 0.0), Wave::NOISE),
        ( 3.0, 0.0, LinearRgba::rgb(0.2, 1.0, 0.2), Wave::SQUARE),
        ( 6.0, 0.0, LinearRgba::rgb(1.0, 0.5, 0.1), Wave::SINE),
    ];

    for (x,y,color,f) in nodes {
        commands.spawn(CycleWaveBundle{
            cycle: Cycle{
                color: color,
                frequency: Cycle::NOTE_A4,
                ..Default::default()
            },
            wave: Wave::new(f),
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            ..default()
        });
    }
}

fn colorize(
    hover: Res<Hover>,
    mut q_cycles: Query<&mut Cycle>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.pressed(KeyCode::KeyC) {return}
    let Some(ent) = hover.entity else {return};
    let Ok(mut cycle) = q_cycles.get_mut(ent) else {return};
    let hue = (hover.position.to_angle() + PI) / TAU;
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
