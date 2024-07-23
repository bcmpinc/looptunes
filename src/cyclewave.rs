#![allow(dead_code)]

use bevy::app::App;
use bevy::asset::Asset;
use bevy::color::LinearRgba;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin};

use rand::{thread_rng, Rng};

pub struct CycleWavePlugin;
impl Plugin for CycleWavePlugin {
    fn build(&self, app: &mut App) {
        app .add_plugins(Material2dPlugin::<FancyCircleMaterial>::default())
            .add_systems(Update, update_timers);
    }
}

/** Shader for drawing fancy circles. */
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FancyCircleMaterial {
    #[uniform(0)] color: LinearRgba,
    #[uniform(1)] time: f32,
    #[texture(2)] #[sampler(3)] radius: Handle<Image>,
}

impl FancyCircleMaterial {
    pub fn new(color: LinearRgba, radius: Handle<Image>) -> FancyCircleMaterial {
        let time = 0.0;
        FancyCircleMaterial{color, time, radius}
    }
}

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
        c.1.time = time.elapsed_seconds() % 256.0;
    }
}

/**
 * Component that describes an audio cycle. 
 */
#[derive(Component,Clone)]
pub struct Cycle {
    frequency: u32,
    phase: f32,
}

impl Cycle {
    /** List of node frequencies as (name, Hz) pairs. */
    const FREQUENCY_LIST : [(&'static str,f64);114] = [
        ("4m16s",0.00390625), ("1m6s",0.015625), ("16s",0.0625), ("4s",0.25), ("1s",1.0), ("1/4",4.0), 
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
    const DEFAULT_FREQUENCY : u32 = 4;

    pub fn frequency(&self) -> f64 {
        Self::FREQUENCY_LIST[self.frequency as usize].1
    }
    pub fn frequency_name(&self) -> &'static str {
        Self::FREQUENCY_LIST[self.frequency as usize].0
    }
    pub fn phase_in_parent(&self) -> f32 {
        self.phase
    }
}

impl Default for Cycle {
    fn default() -> Self {
        Self {
            frequency: Self::DEFAULT_FREQUENCY,
            phase: 0.0,
        }
    }
}

/** 
 * Component describes a free-form waveform.
 * It's table is automatically synced with the attached FancyCircleMaterial.
 */
#[derive(Component,Clone)]
pub struct Wave {
    pattern: [f32;Self::LENGTH],
}

impl Wave {
    pub const LENGTH : usize = 1024;
    pub const SINE     : fn(f32) -> f32 = |x| f32::cos(x * std::f32::consts::TAU);
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
            pattern: r
        }
    }
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            pattern: [0.0; 1024]
        }
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
    pub material: Handle<FancyCircleMaterial>,
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
            material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        }
    }
}
