use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::prelude::*;

use ringbuf::traits::Split;
use ringbuf::HeapRb;

use rodio::source::SineWave;
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};

pub struct LoopTunes;

impl Plugin for LoopTunes {
    fn build(&self, app: &mut App) {
        println!("Enabling LoopTunes audio backend Plugin!");
        let audio = AudioBackend::default();
        let buffer = HeapRb::<f32>::new(4096);
        let (mut producer, mut consumer) = buffer.split();

        app.insert_resource(audio);
    }
}

#[derive(Resource)]
pub struct AudioBackend {
    //stream_handle: OutputStreamHandle,
}

impl Default for AudioBackend {
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Box::leak(Box::from(stream));
        let sink = Sink::try_new(&stream_handle).unwrap();
        let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(1.0)).amplify(0.20);
        sink.append(source);
        Box::leak(Box::from(sink));
        println!("Stuff should be playing.");
        
        Self { 
            //stream_handle, 
        }
    }
}

impl AudioBackend {

}
