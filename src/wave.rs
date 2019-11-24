pub struct SinWave {
    frequency: f64,
    target_freq: f64,
    phase: f64,
    sample_rate: f64,
    clock: f64,
    delta_freq: f64,
}

pub trait Wave {
    fn new(frequency: f64, sample_rate: f64) -> Self;
    fn next(&mut self) -> f64;
    fn prog_frequency(&mut self, frequency: f64, glide_ratio: f64);
}

impl Default for SinWave {
    fn default() -> SinWave {
        SinWave {
            frequency: 0.0,
            target_freq: 0.0,
            phase: 0.0,
            clock: 0.0,
            sample_rate: 0.0,
            delta_freq: 0.0,
        }
    }
}

impl Wave for SinWave {
    fn new(frequency: f64, sample_rate: f64) -> SinWave {
        SinWave {
            frequency: frequency,
            sample_rate: sample_rate,
            target_freq: frequency,
            phase: 0.0,
            clock: 0.0,
            delta_freq: 0.0,
        }
    }

    fn prog_frequency(&mut self, frequency: f64, glide_ratio: f64) {
        self.phase = 0.0;
        self.target_freq = frequency;
        self.delta_freq = (self.target_freq - self.frequency) * glide_ratio;
    }

    fn next(&mut self) -> f64 {
        if self.frequency != self.target_freq {
            self.frequency = match self.delta_freq.abs() > (self.target_freq - self.frequency).abs() {
                true => self.target_freq,
                false => self.frequency + self.delta_freq,
            };
        }

        let phase_delta: f64 = 2.0 * std::f64::consts::PI * self.frequency / self.sample_rate;
        self.clock = (self.clock + 1.0) % self.sample_rate;
        let next = self.phase.sin();
        self.phase += phase_delta;
        next
    }
}
