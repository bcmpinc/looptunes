use bevy::app::App;
use bevy::asset::Asset;
use bevy::color::LinearRgba;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin};

pub struct NodePlugin;
impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app .add_plugins(Material2dPlugin::<FancyCircleMaterial>::default())
            .add_systems(Update, update_timers);
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FancyCircleMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    time: f32,
    #[texture(2)]
    #[sampler(3)]
    radius: Handle<Image>,
}

impl FancyCircleMaterial {
    pub fn new(color: LinearRgba, radius: Handle<Image>) -> FancyCircleMaterial {
        let time = 0.0;
        FancyCircleMaterial{color, time, radius}
    }
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for FancyCircleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circle.wgsl".into()
    }
}

fn update_timers(
    mut circles: ResMut<Assets<FancyCircleMaterial>>,
    time: Res<Time>
) {
    for c in circles.iter_mut() {
        c.1.time = time.elapsed_seconds();
    }
}
