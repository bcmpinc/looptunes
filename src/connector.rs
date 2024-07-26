use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

pub struct ConnectorPlugin;

impl Plugin for ConnectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(SpawnScene, create_segment_mesh);
        app.add_systems(PostUpdate, position_segment_mesh);
    }
}

#[derive(Component)]
pub struct Segment {
    pub source: Vec2,
    pub target: Vec2,
    pub source_size: f32,
    pub target_size: f32,
}

#[derive(Component)] struct Bow(Entity);
#[derive(Component)] struct Arrow(Entity);

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
            ..Default::default()
        });
    }
}

fn position_segment_mesh(
    mut q: Query<(Ref<Segment>,&mut Transform)>,
) {
    for (seg, mut transform) in q.iter_mut() {
        if seg.is_changed() {
            let length = Vec2::distance(seg.target, seg.source);
            let width = f32::min(seg.source_size, seg.target_size);
            transform.scale = Vec3::new(length, width, 1.0);
            transform.translation = (0.5 * (seg.source + seg.target)).extend(0.0);
            transform.rotation = Quat::from_rotation_z(Vec2::to_angle(seg.target - seg.source));
        }
    }
}
