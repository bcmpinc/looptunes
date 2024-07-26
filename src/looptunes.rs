use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::prelude::*;

use rodio::source::SeekError;
use rodio::{OutputStream, Sink, Source};

use crossbeam_channel::{bounded, Receiver, Sender};

pub struct LoopTunes;

impl Plugin for LoopTunes {
    fn build(&self, app: &mut App) {
        println!("Enabling LoopTunes audio backend Plugin!");
        app.insert_resource(LoopTunesBackend::default());
        //app.add_systems(Update, play_anything);
    }
}

pub const PLAY_CHUNK: usize = 1024;
pub fn backend_has_capacity(
    backend: Res<LoopTunesBackend>,
) -> bool {
    backend.producer.capacity().unwrap() - backend.producer.len() >= PLAY_CHUNK
}

#[derive(Resource)]
pub struct LoopTunesBackend {
    pub producer: Sender<f32>,
}
impl Default for LoopTunesBackend {
    fn default() -> Self {
        // Init rodio
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Box::leak(Box::from(stream)); // Keep stream alive for the duration of the application.

        // Create a channel
        let (tx,rx) = bounded::<f32>(PLAY_CHUNK*4);
        let source = LoopSource{consumer: rx, last: 0.0};

        // Get something we can send audio to.
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        Box::leak(Box::from(sink));
        
        Self { producer: tx }
    }
}

pub struct LoopSource {
    consumer: Receiver<f32>,
    last: f32, // Add bit of ease when stopped.
}

impl Iterator for LoopSource {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        if let Ok(value) = self.consumer.try_recv() {
            self.last = value;
            return Some(value);
        }
        self.last *= 0.99;
        return Some(self.last);
    }
}

impl Source for LoopSource {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }

    #[inline]
    fn try_seek(&mut self, _: Duration) -> Result<(), SeekError> {
        Ok(())
    }
}

impl Drop for LoopSource {
    fn drop(&mut self) {
        println!("LoopTunes Backend source died. RIP.")
    }
}
