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

#[derive(Resource)]
pub struct LoopTunesBackend {
    producer: Sender<f32>,
    position: u32,
}
impl LoopTunesBackend {
    const SAMPLE_RATE: u32 = 48000;
    const FRAME_SIZE: usize = 1024;
    const BUFFER: usize = Self::FRAME_SIZE * 4;

    pub fn reset(&mut self) {
        self.position = 0;
    }

    pub fn send_buffer(&mut self, samples: &Vec<f32>) {
        for &sample in samples.iter() {
            _ = self.producer.send(sample);
        }
    
        // Update playback position
        self.position += samples.len() as u32;
        self.position %= Self::SAMPLE_RATE * 256 * 3;
    }

    pub fn has_free_space(&self) -> bool {
        Self::BUFFER - self.producer.len() >= Self::FRAME_SIZE
    }

    pub fn time_chunk(&self) -> Vec<f64> {
        (0..Self::FRAME_SIZE as u32).map(|i| (self.position + i) as f64 / Self::SAMPLE_RATE as f64).collect()
    }

    pub fn elapsed_seconds(&self) -> f32 {
        (self.position as f32 - (16 * Self::FRAME_SIZE) as f32) / Self::SAMPLE_RATE as f32
    }
}
impl Default for LoopTunesBackend {
    fn default() -> Self {
        // Init rodio
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Box::leak(Box::from(stream)); // Keep stream alive for the duration of the application.

        // Create a channel
        let (tx,rx) = bounded::<f32>(LoopTunesBackend::BUFFER);
        let source = LoopSource{consumer: rx, last: 0.0};

        // Get something we can send audio to.
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        Box::leak(Box::from(sink));
        
        Self { 
            producer: tx,
            position: 0,
        }
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
        LoopTunesBackend::SAMPLE_RATE
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
