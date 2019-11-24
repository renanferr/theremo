pub struct SinWave {
    frequency: f32,
    phase: f32,
    sample_rate: f32,
    clock: f32,
}

pub trait Wave {
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
        next
    }
}
