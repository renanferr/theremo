extern crate anyhow;
extern crate cpal;

use std::sync::Mutex;

use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};

use anyhow::anyhow;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;

fn spawn_stdin_channel() -> Receiver<f32> {
  let stdin = 0;
  let mut buffer: [u8; 1] = [0; 1];
  let termios = Termios::from_fd(stdin).unwrap();
  let mut new_termios = termios.clone();

  new_termios.c_lflag &= !(ICANON | ECHO);
  tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

  let (tx, rx) = mpsc::channel::<f32>();

  let stdout = io::stdout();
  let mut reader = io::stdin();

  let mut notes: HashMap<u8, f32> = HashMap::new();
  for k in KEY_NOTES.iter() {
    notes.insert(k.key, k.frequency);
  }

  thread::spawn(move || loop {
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    let key: u8 = buffer[0];

    match notes.get(&key) {
      Some(freq) => tx.send(*freq).unwrap(),
      None => (),
    }
  });
  rx
}


#[derive(Debug)]
struct KeyNote {
  key: u8,
  frequency: f32,
}

const KEY_NOTES: [KeyNote; 8] = [
  KeyNote {
    key: 97,
    frequency: 261.63,
  },
  KeyNote {
    key: 115,
    frequency: 293.66,
  },
  KeyNote {
    key: 100,
    frequency: 329.63,
  },
  KeyNote {
    key: 102,
    frequency: 349.23,
  },
  KeyNote {
    key: 103,
    frequency: 392.0,
  },
  KeyNote {
    key: 104,
    frequency: 440.0,
  },
  KeyNote {
    key: 106,
    frequency: 493.88,
  },
  KeyNote {
    key: 107,
    frequency: 523.25,
  },
];

struct SinWave {
  frequency: f32,
  phase: f32,
  sample_rate: f32,
  clock: f32
}

trait Wave {
  fn new(frequency: f32, sample_rate: f32) -> Self;
  fn next(&mut self) -> f32;
}

impl Wave for SinWave {
  fn new(frequency: f32, sample_rate: f32) -> SinWave {
    SinWave {
      frequency: frequency,
      phase: 0.0,
      sample_rate: sample_rate,
      clock: 0.0,
    }
  }

  fn next(&mut self) -> f32 {
    let delta: f32 = 2.0 * std::f32::consts::PI * self.frequency / self.sample_rate;
    self.clock = (self.clock + 1.0) % self.sample_rate;
    let next = self.phase.sin();
    self.phase += delta;
    // println!("Rendering {:?}", next);
    next
  }

}

fn main() -> Result<(), anyhow::Error> {
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

  let stdin_channel = spawn_stdin_channel();

  event_loop.run(move |id, result| {
    let freq: Option<f32> = match stdin_channel.try_recv() {
      Ok(freq) =>                         Some(freq),
      Err(TryRecvError::Empty) =>         None,
      Err(TryRecvError::Disconnected) =>  panic!("Channel disconnected"),
    };

    if freq.is_some() {
      let mut wave = wave.lock().expect("Could not lock wave");
      *wave = SinWave::new(freq.unwrap(), sample_rate);
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
