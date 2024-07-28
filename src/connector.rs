use std::f32::consts::{PI, TAU};

use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

use crate::{CommandsExt, Cycle, MousePos};

pub struct ConnectorPlugin;

impl Plugin for ConnectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, arrow_sync_parent);
        app.add_systems(Update, connector_arrow_tracks_cursor);
        app.add_systems(SpawnScene, (create_segment_mesh, create_bow_sprite, create_arrow_sprite));
        app.add_systems(PostUpdate, position_segment_mesh);
        app.add_systems(Last, clear_orphaned_segments);
        app.insert_resource(Connector(None));
    }
}

#[derive(Component)]
pub struct Segment {
    pub child_cycle: Entity,
    pub parent_cycle: Option<Entity>,
    bow: Entity,
    arrow: Entity,
}

impl Segment {
    /** Spawns a new segment originating from child_cycle. */
    pub fn spawn(commands: &mut Commands, child_cycle: Entity) -> Entity {
        //println!("Creating new connector for {:?}", child_cycle);
        let bow = commands.spawn(Bow).set_parent(child_cycle).id();
        let arrow = commands.spawn(Arrow).id();
        commands.spawn(Segment{
            parent_cycle: None,
            child_cycle,
            bow,
            arrow,
        }).id()
    }
}

#[derive(Component)] struct Bow;
#[derive(Component)] struct Arrow;

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

pub fn despawn_segment(commands: &mut Commands, entity: Entity, segment: &Segment) {
    commands.try_despawn(segment.bow);
    commands.try_despawn(segment.arrow);
    commands.try_despawn(entity);
}

fn clear_orphaned_segments(
    mut commands: Commands,
    q: Query<(Entity, &Segment)>,
) {
    for (entity, segment) in q.iter() {
        if commands.get_entity(segment.child_cycle).is_none() {
            despawn_segment(&mut commands, entity, segment);
        } else if let Some(parent) = segment.parent_cycle {
            if commands.get_entity(parent).is_none() {
                despawn_segment(&mut commands, entity, segment);
            }
        }
    }
}

const BOW_POSITION: Vec3 = Vec3::new(0.0,-1.1,0.0);
const ARROW_POSITION: Vec3 = Vec3::new(0.0,1.6,0.0);
const SPRITE_SCALE: f32 = 0.005;
const LINE_WIDTH: f32 = SPRITE_SCALE * 10.0;

fn position_segment_mesh(
    q_segment: Query<(Entity, &Segment)>,
    mut q_transform: Query<&mut Transform>,
    q_global_transform: Query<&GlobalTransform>,
    q_cycle: Query<&Cycle>,
) {
    for (segment_entity, segment) in q_segment.iter() {
        let Ok(child_cycle) = q_cycle.get(segment.child_cycle) else {continue};
        let parent_cycle = segment.parent_cycle.and_then(|id| q_cycle.get(id).ok());
        let child_pos = q_global_transform.get(segment.child_cycle).unwrap();

        let arrow_global = q_global_transform.get(segment.arrow).unwrap();
        let target = arrow_global.transform_point(Vec3::new(0.0,100.0, 0.0)).truncate();
        let target_size = parent_cycle.map_or(1.0, |c| c.scale());
        if parent_cycle.is_some() {
            let target_rotation = Quat::from_rotation_z(-child_cycle.phase_in_parent() * TAU);
            *q_transform.get_mut(segment.arrow).unwrap() = Transform{
                scale: Vec3::new(target_size * SPRITE_SCALE, target_size * SPRITE_SCALE, 1.0),
                rotation: target_rotation,
                translation: target_size * (target_rotation * ARROW_POSITION),
            };
        }

        let bow_global = q_global_transform.get(segment.bow).unwrap();
        let source = bow_global.transform_point(Vec3::new(0.0,-10.0, 0.0)).truncate();
        let source_size = child_cycle.scale();
        let source_rotation = Quat::from_rotation_z(Vec2::to_angle(target - child_pos.translation().truncate())) * Quat::from_rotation_z(PI/2.0);
        *q_transform.get_mut(segment.bow).unwrap() = Transform{
            scale: Vec3::new(source_size * SPRITE_SCALE, source_size * SPRITE_SCALE, 1.0),
            rotation: source_rotation,
            translation: source_size * (source_rotation * BOW_POSITION),
        };        

        let length = Vec2::distance(target, source);
        let width = f32::min(source_size, target_size) * LINE_WIDTH;
        *q_transform.get_mut(segment_entity).unwrap() = Transform{
            scale: Vec3::new(length, width, 1.0),
            rotation: Quat::from_rotation_z(Vec2::to_angle(target - source)),
            translation: (0.5 * (source + target)).extend(2.0),
        };
    }
}

fn create_bow_sprite(
    mut commands: Commands,
    q: Query<Entity,(With<Bow>,Without<Sprite>)>,
    asset_server: Res<AssetServer>, 
) {
    for bow in q.iter() {
        commands.entity(bow).insert(SpriteBundle {
            texture: asset_server.load("images/bow.png"),
            ..default()
        });
    }
}

fn create_arrow_sprite(
    mut commands: Commands,
    q: Query<Entity,(With<Arrow>,Without<Sprite>)>,
    asset_server: Res<AssetServer>, 
) {
    for arrow in q.iter() {
        commands.entity(arrow).insert(SpriteBundle {
            texture: asset_server.load("images/arrow.png"),
            ..default()
        });
    }
}

fn arrow_sync_parent(
    mut commands: Commands,
    mut q_segments: Query<&Segment>,
    mut q_arrows: Query<Option<&Parent>>,
) {
    for segment in q_segments.iter_mut() {
        let Ok(arrow_parent) = q_arrows.get_mut(segment.arrow) else {continue};
        let Some(mut arrow_entity) = commands.get_entity(segment.arrow) else {continue};
        match (segment.parent_cycle, arrow_parent) {
            (Some(x), Some(y)) if x != y.get() => {
                arrow_entity.set_parent(x);
            },
            (Some(x), None) => {
                arrow_entity.set_parent(x);
            },
            (None, Some(_)) => {
                arrow_entity.remove_parent_in_place();
                return;
            },
            (None, None) => return,
            _ => ()
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
    *arrow = Transform{
        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
        rotation: Quat::from_rotation_z(delta.to_angle() - PI/2.0),
        translation: (mouse.position + delta_clamped).extend(0.0),
    }
}

pub fn connector_segment<'a> (connector: &Connector,segments: &'a Query<&Segment>) -> Option<&'a Segment> {
    return segments.get(connector.0?).ok()
}

pub fn connector_segment_mut<'a> (connector: &Connector,segments: &'a mut Query<&mut Segment>) -> Option<Mut<'a, Segment>> {
    return segments.get_mut(connector.0?).ok()
}
