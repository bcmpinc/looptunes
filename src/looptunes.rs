use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::prelude::*;

use rodio::source::{SeekError, SineWave};
use rodio::{OutputStream, Sink, Source};

use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub struct LoopTunes;

impl Plugin for LoopTunes {
    fn build(&self, app: &mut App) {
        println!("Enabling LoopTunes audio backend Plugin!");
        app.insert_resource(LoopTunesBackend::default());
    }
}

#[derive(Resource)]
pub struct LoopTunesBackend {
    producer: SyncSender<f32>,
    
}
impl Default for LoopTunesBackend {
    fn default() -> Self {
        // Init rodio
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Box::leak(Box::from(stream)); // Keep stream alive for the duration of the application.

        // Create a channel
        let (tx,rx) = sync_channel::<f32>(4096);
        let source = LoopSource{consumer: rx};

        // Get something we can send audio to.
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        Box::leak(Box::from(sink));
        
        Self { producer: tx }
    }
}

pub struct LoopSource {
    consumer: Receiver<f32>
}

impl Iterator for LoopSource {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.consumer.try_recv().ok()
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