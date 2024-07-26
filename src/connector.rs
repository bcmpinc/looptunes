use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

pub struct ConnectorPlugin;

impl Plugin for ConnectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(SpawnScene, (create_segment_mesh, create_bow_sprite, create_arrow_sprite));
        app.add_systems(Update, bow_tracks_segment);
        app.add_systems(PostUpdate, (position_segment_mesh,bow_with_segment,arrow_with_segment));
    }
}

#[derive(Component)]
pub struct Segment {
    pub source: Vec2,
    pub target: Vec2,
    pub source_size: f32,
    pub target_size: f32,
}

#[derive(Component)] pub struct Bow(pub Entity);
#[derive(Component)] pub struct Arrow(pub Entity);

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
    mut q: Query<(&Bow, &mut Transform),Without<Segment>>,
    mut segments: Query<&Transform,With<Segment>>,
) {
    for (bow, mut transform) in q.iter_mut() {
        if let Ok(seg) = segments.get_mut(bow.0) {
            transform.rotation = seg.rotation * Quat::from_rotation_z(PI/2.0);
            transform.translation = transform.rotation * BOW_POSITION;
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

fn arrow_with_segment(
    mut q: Query<(&Arrow,&GlobalTransform)>,
    mut segments: Query<&mut Segment>,
) {
    for (arrow, transform) in q.iter_mut() {
        if let Ok(mut seg) = segments.get_mut(arrow.0) {
            seg.target = transform.transform_point(Vec3::new(0.0,100.0, 0.0)).truncate();
            seg.target_size = transform.affine().x_axis.length();
        }
    }
}