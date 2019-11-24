use std::thread;
use std::io;
use std::io::{Read, Write};

const GRAD_RATIO: f32 = 0.01;

pub struct SinWave {
    frequency: f32,
    intended: f32,
    phase: f32,
    sample_rate: f32,
    clock: f32,
}

pub trait Wave {
    fn new(frequency: f32, sample_rate: f32) -> Self;
    fn next(&mut self) -> f32;
    fn set_frequency(&mut self, frequency: f32);
}

impl Default for SinWave {
    fn default() -> SinWave {
        SinWave {
            frequency: 0.0,
            intended: 0.0,
            phase: 0.0,
            clock: 0.0,
            sample_rate: 0.0,
        }
    }
}

impl Wave for SinWave {
    fn new(frequency: f32, sample_rate: f32) -> SinWave {
        SinWave {
            frequency: frequency,
            sample_rate: sample_rate,
            intended: frequency,
            phase: 0.0,
            clock: 0.0,
        }
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.phase = 0.0;
        self.intended = frequency;
    }

    fn next(&mut self) -> f32 {
        if self.frequency != self.intended {
            if self.frequency > self.intended {
                // println!("{} > {}", self.frequency, self.intended);
                self.frequency -= GRAD_RATIO;
            } else {
                // println!("{} < {}", self.frequency, self.intended);
                self.frequency += GRAD_RATIO;
            }
        }

        let delta: f32 = 2.0 * std::f32::consts::PI * self.frequency / self.sample_rate;
        self.clock = (self.clock + 1.0) % self.sample_rate;
        let next = self.phase.sin();
        self.phase += delta;
        next
    }
}
