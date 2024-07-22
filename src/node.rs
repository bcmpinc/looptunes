use bevy::app::App;
use bevy::asset::Asset;
use bevy::color::LinearRgba;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin};

struct Frequency{
    seconds: f32,
    name: &'static str,
}

#[allow(non_snake_case)]
const fn F(seconds: f32, name: &'static str) -> Frequency { Frequency{seconds, name} }
const FREQUENCY_LIST : [Frequency;114] = [
    F(256.0,"4m16s"), F(64.0,"1m6s"), F(16.0,"16s"), F(4.0,"4s"), F(1.0,"1s"), F(0.25,"1/4"), 
    F(16.351597831287414,"C0"), F(17.323914436054505,"C#0"), F(18.354047994837977,"D0"), F(19.445436482630058,"D#0"), F(20.601722307054366,"E0"), F(21.826764464562746,"F0"), F(23.12465141947715,"F#0"), F(24.499714748859326,"G0"), F(25.956543598746574,"G#0"), F(27.5,"A0"), F(29.13523509488062,"A#0"), F(30.86770632850775,"B0"), 
    F(32.70319566257483,"C1"), F(34.64782887210901,"C#1"), F(36.70809598967594,"D1"), F(38.890872965260115,"D#1"), F(41.20344461410875,"E1"), F(43.653528929125486,"F1"), F(46.2493028389543,"F#1"), F(48.999429497718666,"G1"), F(51.91308719749314,"G#1"), F(55.0,"A1"), F(58.27047018976124,"A#1"), F(61.7354126570155,"B1"), 
    F(65.40639132514966,"C2"), F(69.29565774421802,"C#2"), F(73.41619197935188,"D2"), F(77.78174593052023,"D#2"), F(82.4068892282175,"E2"), F(87.30705785825097,"F2"), F(92.4986056779086,"F#2"), F(97.99885899543733,"G2"), F(103.82617439498628,"G#2"), F(110.0,"A2"), F(116.54094037952248,"A#2"), F(123.47082531403103,"B2"), 
    F(130.8127826502993,"C3"), F(138.59131548843604,"C#3"), F(146.83238395870376,"D3"), F(155.56349186104046,"D#3"), F(164.81377845643496,"E3"), F(174.61411571650194,"F3"), F(184.9972113558172,"F#3"), F(195.99771799087463,"G3"), F(207.65234878997256,"G#3"), F(220.0,"A3"), F(233.08188075904496,"A#3"), F(246.94165062806206,"B3"), 
    F(261.6255653005986,"C4"), F(277.1826309768721,"C#4"), F(293.6647679174076,"D4"), F(311.1269837220809,"D#4"), F(329.6275569128699,"E4"), F(349.2282314330039,"F4"), F(369.9944227116344,"F#4"), F(391.99543598174927,"G4"), F(415.3046975799451,"G#4"), F(440.0,"A4"), F(466.1637615180899,"A#4"), F(493.8833012561241,"B4"), 
    F(523.2511306011972,"C5"), F(554.3652619537442,"C#5"), F(587.3295358348151,"D5"), F(622.2539674441618,"D#5"), F(659.2551138257398,"E5"), F(698.4564628660078,"F5"), F(739.9888454232688,"F#5"), F(783.9908719634985,"G5"), F(830.6093951598903,"G#5"), F(880.0,"A5"), F(932.3275230361799,"A#5"), F(987.7666025122483,"B5"), 
    F(1046.5022612023945,"C6"), F(1108.7305239074883,"C#6"), F(1174.65907166963,"D6"), F(1244.5079348883237,"D#6"), F(1318.5102276514797,"E6"), F(1396.9129257320155,"F6"), F(1479.9776908465376,"F#6"), F(1567.981743926997,"G6"), F(1661.2187903197805,"G#6"), F(1760.0,"A6"), F(1864.6550460723597,"A#6"), F(1975.533205024496,"B6"), 
    F(2093.004522404789,"C7"), F(2217.4610478149766,"C#7"), F(2349.31814333926,"D7"), F(2489.0158697766474,"D#7"), F(2637.02045530296,"E7"), F(2793.825851464031,"F7"), F(2959.955381693075,"F#7"), F(3135.9634878539946,"G7"), F(3322.437580639561,"G#7"), F(3520.0,"A7"), F(3729.3100921447194,"A#7"), F(3951.066410048992,"B7"), 
    F(4186.009044809578,"C8"), F(4434.922095629953,"C#8"), F(4698.63628667852,"D8"), F(4978.031739553295,"D#8"), F(5274.04091060592,"E8"), F(5587.651702928062,"F8"), F(5919.91076338615,"F#8"), F(6271.926975707989,"G8"), F(6644.875161279122,"G#8"), F(7040.0,"A8"), F(7458.620184289437,"A#8"), F(7902.132820097988,"B8"), 
];

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
