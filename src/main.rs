extern crate anyhow;
extern crate cpal;

mod config;
mod stdin;
mod wave;

use wave::{Wave, SinWave};

use std::sync::Mutex;

use anyhow::anyhow;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use std::sync::mpsc::TryRecvError;

fn main() -> Result<(), anyhow::Error> {
  let configs = config::init();
  let host = cpal::default_host();
  let device = host
    .default_output_device()
    .expect("failed to find a default output device");

  let format = match device.default_output_format() {
    Ok(format) => format,
    Err(err) => {
      eprintln!("an error occurred on format");
      return Err(anyhow!("an error occurred on format {:?}", err));
    }
  };

  let event_loop = host.event_loop();
  let stream_id = match event_loop.build_output_stream(&device, &format) {
    Ok(stream_id) => stream_id,
    Err(err) => {
      eprintln!("an error occurred on stream_id");
      return Err(anyhow!("an error occurred on stream_id {:?}", err));
    }
  };

  match event_loop.play_stream(stream_id.clone()) {
    Ok(_) => (),
    Err(err) => {
      eprintln!("an error occurred on stream play {:?}", err);
    }
  };

  let sample_rate = format.sample_rate.0 as f32;

  let wave: Mutex<SinWave> = Mutex::new(SinWave::new(0.0, sample_rate));

  let stdin_ch = stdin::spawn();

  event_loop.run(move |id, result| {
    let key: Option<u8> = match stdin_ch.try_recv() {
      Ok(key)                         => Some(key),
      Err(TryRecvError::Empty)        => None,
      Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    };

    if key.is_some() {
      let freq = configs.keymappings.get(&key.unwrap());
      if freq.is_some() {
        let mut wave = wave.lock().expect("Could not lock wave");
        wave.set_frequency(*freq.unwrap());
      }
    }

    let data = match result {
      Ok(data) => data,
      Err(err) => {
        eprintln!("an error occurred on stream {:?}: {}", id, err);
        return;
      }
    };

    match data {
      cpal::StreamData::Output {
        buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
      } => {
        let mut wave = wave.lock().expect("Could not lock wave");
        for sample in buffer.chunks_mut(format.channels as usize) {
          let value = ((wave.next() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
          for out in sample.iter_mut() {
            *out = value;
          }
        }
      }
      cpal::StreamData::Output {
        buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
      } => {
        let mut wave = wave.lock().expect("Could not lock wave");
        for sample in buffer.chunks_mut(format.channels as usize) {
          let value = (wave.next() * std::i16::MAX as f32) as i16;
          for out in sample.iter_mut() {
            *out = value;
          }
        }
      }
      cpal::StreamData::Output {
        buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
      } => {
        let mut wave = wave.lock().expect("Could not lock wave");
        for sample in buffer.chunks_mut(format.channels as usize) {
          for out in sample.iter_mut() {
            *out = wave.next();
          }
        }
      }
      _ => (),
    }
  });
}
