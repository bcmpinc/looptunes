use std::f32::consts::{PI, TAU};

use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

use crate::{CommandsExt, Cycle, MousePos};

pub struct ConnectorPlugin;

impl Plugin for ConnectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(SpawnScene, (create_segment_mesh, create_bow_sprite, create_arrow_sprite));
        app.add_systems(Update, (bow_tracks_segment, connector_arrow_tracks_cursor));
        app.add_systems(PostUpdate, (
            arrow_copy_phase,
            (bow_with_segment,arrow_with_segment),
            position_segment_mesh,
        ).chain());
        app.add_systems(Last, (
            (clear_orphaned_bows, clear_orphaned_arrows),
            clear_orphaned_segments,
            (clear_unconnected_bows, clear_unconnected_arrows),
        ).chain());
        app.insert_resource(Connector(None));
    }
}

#[derive(Component)]
pub struct Segment {
    pub source: Vec2,
    pub target: Vec2,
    pub source_size: f32,
    pub target_size: f32,
    pub child_cycle: Entity,
    pub bow: Entity,
    pub arrow: Entity,
}

impl Segment {
    /** Spawns a new segment originating from child_cycle. */
    pub fn spawn(commands: &mut Commands, child_cycle: Entity) -> Entity {
        //println!("Creating new connector for {:?}", child_cycle);
        let segment = commands.spawn_empty().id();
        let bow = commands.spawn(
            Bow(segment)
        ).set_parent(child_cycle).id();
        let arrow = commands.spawn(Arrow{
            segment,
            child_cycle,
        }).id();
        commands.entity(segment).insert(Segment{
            source: default(), 
            target: default(), 
            source_size: 1.0, 
            target_size: 1.0, 
            child_cycle,
            bow,
            arrow,
        }).id()
    }
}

#[derive(Component)] pub struct Bow(pub Entity);
#[derive(Component)] pub struct Arrow{
    pub segment: Entity,
    pub child_cycle: Entity,
}

#[derive(Resource)] pub struct Connector(pub Option<Entity>);

fn create_segment_mesh(
    mut commands: Commands,
    q: Query<Entity,(With<Segment>,Without<Mesh2dHandle>)>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for seg in q.iter() {
        let mesh = Rectangle::default();
        commands.entity(seg).insert(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(mesh)),
            material: materials.add(ColorMaterial::from_color(Color::WHITE)),
            ..default()
        });
    }
}

fn position_segment_mesh(
    mut q: Query<(Ref<Segment>,&mut Transform)>,
) {
    for (seg, mut transform) in q.iter_mut() {
        if seg.is_changed() {
            let length = Vec2::distance(seg.target, seg.source);
            let width = f32::min(seg.source_size, seg.target_size) * 10.0;
            transform.scale = Vec3::new(length, width, 1.0);
            transform.translation = (0.5 * (seg.source + seg.target)).extend(2.0);
            transform.rotation = Quat::from_rotation_z(Vec2::to_angle(seg.target - seg.source));
        }
    }
}

fn clear_orphaned_segments(
    mut commands: Commands,
    q: Query<(Entity, &Segment)>,
) {
    for (entity, seg) in q.iter() {
        if commands.get_entity(seg.bow).is_none() {
            commands.try_despawn(entity);
        }
    }
}

const BOW_POSITION: Vec3 = Vec3::new(0.0,-0.53,0.0);
fn create_bow_sprite(
    mut commands: Commands,
    q: Query<Entity,(With<Bow>,Without<Sprite>)>,
    asset_server: Res<AssetServer>, 
) {
    for bow in q.iter() {
        commands.entity(bow).insert(SpriteBundle {
            texture: asset_server.load("images/bow.png"),
            transform: Transform::from_translation(BOW_POSITION).with_scale(Vec3::splat(0.0025)),
            ..default()
        });
    }
}

fn bow_with_segment(
    mut q: Query<(&Bow,&GlobalTransform)>,
    mut segments: Query<&mut Segment>,
) {
    for (bow, transform) in q.iter_mut() {
        if let Ok(mut seg) = segments.get_mut(bow.0) {
            seg.source = transform.transform_point(Vec3::new(0.0,-10.0, 0.0)).truncate();
            seg.source_size = transform.affine().x_axis.length();
        }
    }
}

fn bow_tracks_segment(
    mut q: Query<(&Bow, &mut Transform, &Parent),Without<Segment>>,
    q_parent: Query<&GlobalTransform>,
    segments: Query<&Segment>,
) {
    for (bow, mut transform, parent) in q.iter_mut() {
        let Ok(seg) = segments.get(bow.0) else {continue};
        let Ok(parent_pos) = q_parent.get(parent.get()) else {continue};
        transform.rotation = Quat::from_rotation_z(Vec2::to_angle(seg.target - parent_pos.translation().truncate())) * Quat::from_rotation_z(PI/2.0);
        transform.translation = transform.rotation * BOW_POSITION;
    }
}

fn clear_orphaned_bows(
    mut commands: Commands,
    q: Query<Entity, (With<Bow>, Without<Parent>)>,
) {
    for entity in q.iter() {
        commands.try_despawn(entity);
    }
}

fn clear_unconnected_bows(
    mut commands: Commands,
    q: Query<(Entity, &Bow)>,
) {
    for (id, bow) in q.iter() {
        if commands.get_entity(bow.0).is_none() {
            commands.try_despawn(id);
        }
    }
}

const ARROW_POSITION : Vec3 = Vec3::new(0.0,0.8,0.0);
fn create_arrow_sprite(
    mut commands: Commands,
    q: Query<Entity,(With<Arrow>,Without<Sprite>)>,
    asset_server: Res<AssetServer>, 
) {
    for arrow in q.iter() {
        commands.entity(arrow).insert(SpriteBundle {
            texture: asset_server.load("images/arrow.png"),
            transform: Transform::from_translation(ARROW_POSITION).with_scale(Vec3::splat(0.0025)),
            ..default()
        });
    }
}

fn arrow_copy_phase(
    mut q: Query<(&Arrow,&mut Transform), With<Parent>>,
    cycles: Query<&Cycle>,
) {
    for (arrow, mut transform) in q.iter_mut() {
        if let Ok(cycle) = cycles.get(arrow.child_cycle) {
            transform.rotation = Quat::from_rotation_z(-cycle.phase_in_parent() * TAU);
            transform.translation = transform.rotation * ARROW_POSITION;
        }
    }
}

fn arrow_with_segment(
    q: Query<(&Arrow,&GlobalTransform)>,
    mut segments: Query<&mut Segment>,
) {
    for (arrow, transform) in q.iter() {
        if let Ok(mut seg) = segments.get_mut(arrow.segment) {
            seg.target = transform.transform_point(Vec3::new(0.0,100.0, 0.0)).truncate();
            seg.target_size = transform.affine().x_axis.length();
        }
    }
}

fn connector_arrow_tracks_cursor(
    mut q: Query<&mut Transform, (With<Arrow>, Without<Parent>)>,
    connector: Res<Connector>,
    segments: Query<&Segment>,
    mouse: Res<MousePos>,
) {
    let Some(segment) = connector_segment(&connector, &segments) else {return};
    let Ok(mut arrow) = q.get_mut(segment.arrow) else {return};
    
    // Make arrow follow the cursor
    let delta = arrow.translation.truncate() - mouse.position;
    let delta_clamped = delta.clamp_length(0.3, 0.3);
    arrow.translation = (mouse.position + delta_clamped).extend(0.0);
    arrow.rotation = Quat::from_rotation_z(delta.to_angle() - PI/2.0);
}

fn clear_orphaned_arrows(
    mut commands: Commands,
    q: Query<Entity, (With<Arrow>, Without<Parent>)>,
    connector: Res<Connector>,
    segments: Query<&Segment>,
) {
    let Some(segment) = connector_segment(&connector, &segments) else {return};
    for entity in q.iter() {
        if segment.arrow != entity {
            commands.try_despawn(entity);
        }
    }
}

fn clear_unconnected_arrows(
    mut commands: Commands,
    q: Query<(Entity, &Arrow)>,
) {
    for (id, arrow) in q.iter() {
        if commands.get_entity(arrow.segment).is_none() {
            commands.try_despawn(id);
        }
    }
}

pub fn connector_segment<'a> (
    connector: &Connector,
    segments: &'a Query<&Segment>,
) -> Option<&'a Segment> {
    return segments.get(connector.0?).ok()
}
