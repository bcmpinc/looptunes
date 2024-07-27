use bevy::app::App;
use bevy::asset::Asset;
use bevy::color::LinearRgba;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState, Extent3d, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError, TextureDimension, TextureFormat};
use bevy::sprite::{Anchor, Material2d, Material2dKey, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle};

use rand::{thread_rng, Rng};

fn never() -> bool {false}

pub struct CycleWavePlugin;
impl Plugin for CycleWavePlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_plugins(Material2dPlugin::<WaveMaterial>::default())
            .add_systems(SpawnScene, (update_textures, create_children).chain())
            .add_systems(Update, (update_frequency, rotate_cyclewaves.run_if(never)).chain())
            .add_systems(PostUpdate, clean_orphans)
        ;
    }
}

/**
 * Component that describes an audio cycle. 
 */
#[derive(Component,Clone)]
pub struct Cycle {
    pub frequency: u32,
    pub phase: f32,
    pub color: LinearRgba,
}

impl Cycle {
    /** List of node frequencies as (name, Hz) pairs. */
    const FREQUENCY_LIST : [(&'static str,f64);133] = [
        ("4m16s",0.00390625), ("3m12s",0.005208333333333333), ("2m8s",0.0078125), ("1m36s",0.010416666666666666), ("1m4s",0.015625), ("48s",0.020833333333333332), ("32s",0.03125), ("24s",0.041666666666666664), ("16s",0.0625), ("12s",0.08333333333333333), ("8s",0.125), ("6s",0.16666666666666666), ("4s",0.25), ("3s",0.3333333333333333), ("2s",0.5), 
        ("1.5s",0.6666666666666666), ("1s",1.0), ("3/4",1.3333333333333333), ("1/2",2.0), ("3/8",2.6666666666666665), ("1/4",4.0), ("3/16",5.333333333333333), ("1/8",8.0), ("3/32",10.666666666666666), ("1/16",16.0),
        ("C0",16.351597831287414), ("C#0",17.323914436054505), ("D0",18.354047994837977), ("D#0",19.445436482630058), ("E0",20.601722307054366), ("F0",21.826764464562746), ("F#0",23.12465141947715), ("G0",24.499714748859326), ("G#0",25.956543598746574), ("A0",27.5), ("A#0",29.13523509488062), ("B0",30.86770632850775), 
        ("C1",32.70319566257483), ("C#1",34.64782887210901), ("D1",36.70809598967594), ("D#1",38.890872965260115), ("E1",41.20344461410875), ("F1",43.653528929125486), ("F#1",46.2493028389543), ("G1",48.999429497718666), ("G#1",51.91308719749314), ("A1",55.0), ("A#1",58.27047018976124), ("B1",61.7354126570155), 
        ("C2",65.40639132514966), ("C#2",69.29565774421802), ("D2",73.41619197935188), ("D#2",77.78174593052023), ("E2",82.4068892282175), ("F2",87.30705785825097), ("F#2",92.4986056779086), ("G2",97.99885899543733), ("G#2",103.82617439498628), ("A2",110.0), ("A#2",116.54094037952248), ("B2",123.47082531403103), 
        ("C3",130.8127826502993), ("C#3",138.59131548843604), ("D3",146.83238395870376), ("D#3",155.56349186104046), ("E3",164.81377845643496), ("F3",174.61411571650194), ("F#3",184.9972113558172), ("G3",195.99771799087463), ("G#3",207.65234878997256), ("A3",220.0), ("A#3",233.08188075904496), ("B3",246.94165062806206), 
        ("C4",261.6255653005986), ("C#4",277.1826309768721), ("D4",293.6647679174076), ("D#4",311.1269837220809), ("E4",329.6275569128699), ("F4",349.2282314330039), ("F#4",369.9944227116344), ("G4",391.99543598174927), ("G#4",415.3046975799451), ("A4",440.0), ("A#4",466.1637615180899), ("B4",493.8833012561241), 
        ("C5",523.2511306011972), ("C#5",554.3652619537442), ("D5",587.3295358348151), ("D#5",622.2539674441618), ("E5",659.2551138257398), ("F5",698.4564628660078), ("F#5",739.9888454232688), ("G5",783.9908719634985), ("G#5",830.6093951598903), ("A5",880.0), ("A#5",932.3275230361799), ("B5",987.7666025122483), 
        ("C6",1046.5022612023945), ("C#6",1108.7305239074883), ("D6",1174.65907166963), ("D#6",1244.5079348883237), ("E6",1318.5102276514797), ("F6",1396.9129257320155), ("F#6",1479.9776908465376), ("G6",1567.981743926997), ("G#6",1661.2187903197805), ("A6",1760.0), ("A#6",1864.6550460723597), ("B6",1975.533205024496), 
        ("C7",2093.004522404789), ("C#7",2217.4610478149766), ("D7",2349.31814333926), ("D#7",2489.0158697766474), ("E7",2637.02045530296), ("F7",2793.825851464031), ("F#7",2959.955381693075), ("G7",3135.9634878539946), ("G#7",3322.437580639561), ("A7",3520.0), ("A#7",3729.3100921447194), ("B7",3951.066410048992), 
        ("C8",4186.009044809578), ("C#8",4434.922095629953), ("D8",4698.63628667852), ("D#8",4978.031739553295), ("E8",5274.04091060592), ("F8",5587.651702928062), ("F#8",5919.91076338615), ("G8",6271.926975707989), ("G#8",6644.875161279122), ("A8",7040.0), ("A#8",7458.620184289437), ("B8",7902.132820097988), 
    ];
    pub const DEFAULT_FREQUENCY : u32 = 16;
    pub const NOTE_A4 : u32 = 82;

    pub fn frequency(&self) -> f64 {
        Self::FREQUENCY_LIST[self.frequency as usize].1
    }
    pub fn frequency_name(&self) -> &'static str {
        Self::FREQUENCY_LIST[self.frequency as usize].0
    }
    pub fn size(&self) -> f32 {
        f32::max(4. / self.frequency().sqrt() as f32, 1.)
    }
    #[allow(unused)]
    pub fn phase_in_parent(&self) -> f32 {
        self.phase
    }
    pub fn change_frequency(&mut self, lines: i32) {
        self.frequency = (self.frequency as i32 + lines).clamp(0, Self::FREQUENCY_LIST.len() as i32 - 1) as u32;
    }
}

impl Default for Cycle {
    fn default() -> Self {
        Self {
            frequency: Self::DEFAULT_FREQUENCY,
            phase: 0.0,
            color: LinearRgba::WHITE,
        }
    }
}

#[derive(Component)] struct WaveSubComponent;

fn create_children(
    mut commands: Commands,
    q: Query<(Entity,Ref<Cycle>,&Wave)>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<WaveMaterial>>,
) {
    for (entity, cycle, wave) in &q {
        if cycle.is_added() {
            let mesh = Rectangle::default();
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    WaveCycleImage,
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(mesh)),
                        material: wave.material.clone(),
                        ..Default::default()
                    },
                    WaveSubComponent,
                ));
                parent.spawn((
                    Text2dBundle{
                        text: Text{
                            sections:vec![
                                TextSection::new(
                                    cycle.frequency_name(),
                                    TextStyle{
                                        font_size: 200.0,
                                        ..default()
                                    }
                                )
                            ],
                            ..default()
                        },
                        text_anchor: Anchor::Center,
                        transform: Transform::from_scale(Vec3::new(0.001,0.001,1.0)),
                        ..default()
                    },
                    WaveSubComponent,
                ));
            });
        }
        if cycle.is_changed() {
            materials.get_mut(&wave.material).unwrap().color = cycle.color;
        }
    }
}

fn clean_orphans(
    mut commands: Commands,
    q: Query<Entity, (With<WaveSubComponent>, Without<Parent>)>
) {
    for entity in q.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_frequency(
    mut q_cycle: Query<(Ref<Cycle>, &mut Transform)>,
    mut q_text: Query<(&mut Text, &Parent)>,
) {
    for (cycle, mut transform) in q_cycle.iter_mut() {
        if cycle.is_changed() {
            let scale = cycle.size();
            transform.scale = Vec3::new(scale, scale, 1.0);
        }
    }

    for (mut text, parent) in q_text.iter_mut() {
        let Ok((cycle, _)) = q_cycle.get_mut(parent.get()) else {continue};
        if cycle.is_changed() {
            text.sections[0].value = cycle.frequency_name().into();
        }
    }
}

#[derive(Component,Clone)]
pub struct WaveCycleImage;

fn rotate_cyclewaves(
    mut q_child: Query<(&Parent, &mut Transform), With<WaveCycleImage>>,
    q_parent: Query<&Cycle>,
    time: Res<Time>,
) {
    for (parent, mut transform) in q_child.iter_mut() {
        let frequency = q_parent.get(parent.get()).unwrap().frequency() as f32;
        transform.rotation = Quat::from_rotation_z(-std::f32::consts::TAU * time.elapsed_seconds() * frequency);
    }
}
    
/** 
 * Component describes a free-form waveform.
 * It's table is automatically synced with the attached FancyCircleMaterial.
 */
#[derive(Component,Clone)]
pub struct Wave {
    pub pattern: [f32;Self::LENGTH],
    pub material: Handle<WaveMaterial>,
    pub average: f32,
}

impl Wave {
    pub const LENGTH : usize = 1024;
    pub const SINE     : fn(f32) -> f32 = |x| 0.5 + 0.5 * f32::cos(x * std::f32::consts::TAU);
    pub const SQUARE   : fn(f32) -> f32 = |x| if x < 0.5 {1.0} else {0.0};
    pub const TRIANGLE : fn(f32) -> f32 = |x| (1.0-2.0*x).abs();
    pub const SAWTOOTH : fn(f32) -> f32 = |x| 1.0-x;
    pub const NOISE    : fn(f32) -> f32 = |_| thread_rng().gen();

    pub fn new(generator: fn(f32) -> f32) -> Self {
        let mut r = [0.0; Self::LENGTH];
        for i in 0..Self::LENGTH {
            r[i] = generator(i as f32 / Self::LENGTH as f32);
        }
        Self {
            pattern: r,
            material: default(),
            average: 0.0,
        }
    }
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            pattern: [0.0; 1024],
            material: default(),
            average: 0.0,
        }
    }
}

fn update_textures(
    mut waves: Query<&mut Wave>,
    mut materials: ResMut<Assets<WaveMaterial>>,
    mut textures: ResMut<Assets<Image>>,
) {
    for mut item in waves.iter_mut() {
        if item.is_added() {
            let wave = &mut item.bypass_change_detection();
            wave.material = materials.add(WaveMaterial::new(LinearRgba::WHITE, default()));
        }
        if item.is_changed() {
            let wave = &mut item.bypass_change_detection();
            wave.average = wave.pattern.iter().sum::<f32>() / 1024.0;
            fn f32_to_u8(v: &f32) -> u8 {
                f32::clamp(v*256.0,0.0,256.0) as u8
            }
            let grayscale_data = wave.pattern.iter().map(f32_to_u8).collect::<Vec<u8>>();
            let image = Image::new(
                Extent3d {
                    width: 1024,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                grayscale_data,
                TextureFormat::R8Unorm,
                RenderAssetUsages::RENDER_WORLD,
            );
            let image_handle = textures.add(image);
            materials.get_mut(&wave.material).unwrap().radius = image_handle;
            //println!("Updated texture for {}", wave.material.id());
        }
    }
}

/** 
 * Shader for drawing fancy circles. 
 */
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct WaveMaterial {
    #[uniform(0)] color: LinearRgba,
    #[texture(1)] #[sampler(2)] radius: Handle<Image>,
}

impl WaveMaterial {
    pub fn new(color: LinearRgba, radius: Handle<Image>) -> WaveMaterial {
        WaveMaterial{color, radius}
    }
}

const ADD : BlendComponent = BlendComponent{
    src_factor: BlendFactor::One,
    dst_factor: BlendFactor::One,
    operation: BlendOperation::Add,
};

impl Material2d for WaveMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circle.wgsl".into()
    }
    
    fn vertex_shader() -> ShaderRef {
        ShaderRef::Default
    }
    
    fn depth_bias(&self) -> f32 {
        0.0
    }
    
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let original = descriptor.fragment.as_mut().unwrap();
        let target = original.targets[0].as_mut().unwrap();
        target.blend = Some(BlendState{
            color: ADD,
            alpha: ADD,
        });
        Ok(())
    }
    
}

/**
 * Bundle that creates a Cycle Wave component.
 * The 
 */
#[derive(Bundle, Clone)]
pub struct CycleWaveBundle {
    pub cycle: Cycle,
    pub wave: Wave,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl Default for CycleWaveBundle {
    fn default() -> Self {
        Self {
            cycle: Default::default(),
            wave: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        }
    }
}
