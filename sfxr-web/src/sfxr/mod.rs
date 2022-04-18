use dasp::{frame::Mono, Frame, Signal};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::hash::Hash;
use std::hash::Hasher;
mod generator;

pub use generator::WaveType;
use generator::{Envelope, Filterable, HighLowPassFilter, Oscillator, Phaser};

#[derive(Clone)]
pub struct GeneratorParameters {
  wave_type: WaveType,
  base_freq: f64,
  freq_limit: f64,
  freq_ramp: f64,
  freq_dramp: f64,
  duty: f32,
  duty_ramp: f32,
  vib_strength: f64,
  vib_speed: f64,
  vib_delay: f32,
  env_attack: f32,
  env_sustain: f32,
  env_decay: f32,
  env_punch: f32,
  lpf_resonance: f32,
  lpf_freq: f32,
  lpf_ramp: f32,
  hpf_freq: f32,
  hpf_ramp: f32,
  pha_offset: f32,
  pha_ramp: f32,
  repeat_speed: f32,
  arp_speed: f32,
  arp_mod: f64,
}
impl GeneratorParameters {
  pub fn reset(&mut self) {
    std::mem::take(self);
  }

  pub fn random_preset(&mut self, preset: &str, seed: u64) {
    let rng = &mut SmallRng::seed_from_u64(seed);
    self.reset();

    match preset {
      "laser" => {
        let wave_types = {
          use WaveType::*;
          [Square, Square, Sine, Sine, Triangle]
        };
        self.wave_type = rand_element(rng, &wave_types);
        if rand_bool(rng, 1, 2) {
          self.base_freq = rand_f64(rng, 0.3, 0.9); // 0 -
          self.freq_limit = rand_f64(rng, 0.0, 0.1);
          self.freq_ramp = rand_f64(rng, -0.35, -0.65);
        } else {
          self.base_freq = rand_f64(rng, 0.5, 1.0);
          self.freq_limit = (self.base_freq - rand_f64(rng, 0.2, 0.8)).max(0.2);
          self.freq_ramp = rand_f64(rng, -0.15, -0.35);
        }
        if rand_bool(rng, 1, 1) {
          self.duty = rand_f32(rng, 0.0, 0.5);
          self.duty_ramp = rand_f32(rng, 0.0, 0.2);
        } else {
          self.duty = rand_f32(rng, 0.4, 0.9);
          self.duty_ramp = rand_f32(rng, 0.0, -0.7);
        }
        self.env_attack = 0.0;
        self.env_sustain = rand_f32(rng, 0.1, 0.3);
        self.env_decay = rand_f32(rng, 0.0, 0.4);
        if rand_bool(rng, 1, 1) {
          self.env_punch = rand_f32(rng, 0.0, 0.3);
        }
        if rand_bool(rng, 1, 2) {
          self.pha_offset = rand_f32(rng, 0.0, 0.2);
          self.pha_ramp = -rand_f32(rng, 0.0, 0.2);
        }
        if rand_bool(rng, 1, 1) {
          self.hpf_freq = rand_f32(rng, 0.0, 0.3);
        }
      }
      "explosion" => {
        self.wave_type = WaveType::Noise;

        if rand_bool(rng, 1, 1) {
          self.base_freq = rand_f64(rng, 0.1, 0.5);
          self.freq_ramp = rand_f64(rng, -0.1, 0.3);
        } else {
          self.base_freq = rand_f64(rng, 0.2, 0.9);
          self.freq_ramp = rand_f64(rng, -0.2, -0.4);
        }

        self.base_freq = self.base_freq.powi(2);

        if rand_bool(rng, 1, 4) {
          self.freq_ramp = 0.0;
        }

        if rand_bool(rng, 1, 2) {
          self.repeat_speed = rand_f32(rng, 0.3, 0.8);
        }

        self.env_attack = 0.0;
        self.env_sustain = rand_f32(rng, 0.1, 0.4);
        self.env_decay = rand_f32(rng, 0.0, 0.5);

        if rand_bool(rng, 1, 1) {
          self.pha_offset = rand_f32(rng, -0.3, 0.6);
          self.pha_ramp = rand_f32(rng, -0.3, 0.0);
        }

        self.env_punch = rand_f32(rng, 0.2, 0.8);

        if rand_bool(rng, 1, 1) {
          self.vib_strength = rand_f64(rng, 0.0, 0.7);
          self.vib_speed = rand_f64(rng, 0.0, 0.6);
        }

        if rand_bool(rng, 1, 2) {
          self.arp_speed = rand_f32(rng, 0.6, 0.9);
          self.arp_mod = rand_f64(rng, -0.8, 0.8);
        }
      }
      "powerup" => {
        if rand_bool(rng, 1, 1) {
          self.wave_type = WaveType::Sine;
        } else {
          self.duty = rand_f32(rng, 0.0, 0.6);
        }
        self.base_freq = rand_f64(rng, 0.2, 0.5);
        if rand_bool(rng, 1, 1) {
          self.freq_ramp = rand_f64(rng, 0.1, 0.5);
          self.repeat_speed = rand_f32(rng, 0.4, 0.8);
        } else {
          self.freq_ramp = rand_f64(rng, 0.05, 0.25);
          if rand_bool(rng, 1, 1) {
            self.vib_strength = rand_f64(rng, 0.0, 0.7);
            self.vib_speed = rand_f64(rng, 0.0, 0.6);
          }
        }
        self.env_attack = 0.0;
        self.env_sustain = rand_f32(rng, 0.0, 0.4);
        self.env_decay = rand_f32(rng, 0.1, 0.5);
      }
      "hit" => {
        self.wave_type = rand_element(rng, &[WaveType::Square, WaveType::Sine, WaveType::Noise]);

        if self.wave_type == WaveType::Square {
          self.duty = rand_f32(rng, 0.0, 0.6);
        }
        self.base_freq = rand_f64(rng, 0.2, 0.8);
        self.freq_ramp = rand_f64(rng, -0.3, -0.7);
        self.env_attack = 0.0;
        self.env_sustain = rand_f32(rng, 0.0, 0.1);
        self.env_decay = rand_f32(rng, 0.1, 0.3);
        if rand_bool(rng, 1, 1) {
          self.hpf_freq = rand_f32(rng, 0.0, 0.3);
        }
      }
      "jump" => {
        self.wave_type = WaveType::Square;
        self.duty = rand_f32(rng, 0.0, 0.6);
        self.base_freq = rand_f64(rng, 0.3, 0.6);
        self.freq_ramp = rand_f64(rng, 0.1, 0.3);
        self.env_attack = 0.0;
        self.env_sustain = rand_f32(rng, 0.1, 0.4);
        self.env_decay = rand_f32(rng, 0.1, 0.3);
        if rand_bool(rng, 1, 1) {
          self.hpf_freq = rand_f32(rng, 0.0, 0.3);
        }
        if rand_bool(rng, 1, 1) {
          self.lpf_freq = rand_f32(rng, 0.4, 1.0);
        }
      }
      "blip" => {
        self.wave_type = rand_element(rng, &[WaveType::Square, WaveType::Sine]);

        if self.wave_type == WaveType::Square {
          self.duty = rand_f32(rng, 0.0, 0.6);
        }

        self.base_freq = rand_f64(rng, 0.2, 0.6);
        self.env_attack = 0.0;
        self.env_sustain = rand_f32(rng, 0.1, 0.2);
        self.env_decay = rand_f32(rng, 0.0, 0.2);
        self.hpf_freq = 0.1;
      }
      "pickup" | _ => {
        self.base_freq = rand_f64(rng, 0.4, 0.9);
        self.env_attack = 0.0;
        self.env_sustain = rand_f32(rng, 0.0, 0.1);
        self.env_decay = rand_f32(rng, 0.1, 0.5);
        self.env_punch = rand_f32(rng, 0.3, 0.6);
        if rand_bool(rng, 1, 1) {
          self.arp_speed = rand_f32(rng, 0.5, 0.7);
          self.arp_mod = rand_f64(rng, 0.2, 0.6);
        }
      }
    }
  }
}
impl Default for GeneratorParameters {
  fn default() -> Self {
    GeneratorParameters {
      wave_type: WaveType::Square,
      base_freq: 0.3,
      env_attack: 0.4,
      env_sustain: 0.1,
      env_decay: 0.5,
      lpf_freq: 1.0,
      freq_limit: 0.,
      freq_ramp: 0.,
      freq_dramp: 0.,
      duty: 0.,
      duty_ramp: 0.,
      vib_strength: 0.,
      vib_speed: 0.,
      vib_delay: 0.,
      env_punch: 0.,
      lpf_resonance: 0.,
      lpf_ramp: 0.,
      hpf_freq: 0.,
      hpf_ramp: 0.,
      pha_offset: 0.,
      pha_ramp: 0.,
      repeat_speed: 0.,
      arp_speed: 0.,
      arp_mod: 0.,
    }
  }
}
impl Hash for GeneratorParameters {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.wave_type.hash(state);
    self.base_freq.to_be_bytes().hash(state);
    self.freq_limit.to_be_bytes().hash(state);
    self.freq_ramp.to_be_bytes().hash(state);
    self.freq_dramp.to_be_bytes().hash(state);
    self.duty.to_be_bytes().hash(state);
    self.duty_ramp.to_be_bytes().hash(state);
    self.vib_strength.to_be_bytes().hash(state);
    self.vib_speed.to_be_bytes().hash(state);
    self.vib_delay.to_be_bytes().hash(state);
    self.env_attack.to_be_bytes().hash(state);
    self.env_sustain.to_be_bytes().hash(state);
    self.env_decay.to_be_bytes().hash(state);
    self.env_punch.to_be_bytes().hash(state);
    self.lpf_resonance.to_be_bytes().hash(state);
    self.lpf_freq.to_be_bytes().hash(state);
    self.lpf_ramp.to_be_bytes().hash(state);
    self.hpf_freq.to_be_bytes().hash(state);
    self.hpf_ramp.to_be_bytes().hash(state);
    self.pha_offset.to_be_bytes().hash(state);
    self.pha_ramp.to_be_bytes().hash(state);
    self.repeat_speed.to_be_bytes().hash(state);
    self.arp_speed.to_be_bytes().hash(state);
    self.arp_mod.to_be_bytes().hash(state);
  }
}

// TODO: refactor
// the signal chain structure need to be separated from the dynamic parameters
// the seed value should only be used for dynamic parameters (not the signal chain)
// the generator will be replaced by the signal chain
// the signal chain is determined by presets (or in the future, data)
// the signal chain takes in the dynamic parameters and generates a `Signal`
// we can randomize the params using a seed value to allow sharing
// a seed value is only valid for a particular signal chain
// for a sound generated using a preset signal chain, we can append the preset to the seed
// to allow us to fully reconstruct the audio from the seed alone
// for custom signal chains, the structure has to be sent along with the seed.
// we can append the sha of the structure to the seed to ensure that it is loaded using the same chain
// repeats should just be a node in the signal chain
// also use the dasp crate
pub struct Generator {
  params: GeneratorParameters,
  volume: f32,
  oscillator: Oscillator,
  hlpf: HighLowPassFilter,
  envelope: Envelope,
  phaser: Phaser,
  rep_time: i32,
  rep_limit: i32,
  exhausted: bool,
}
impl Generator {
  pub fn from_params(params: GeneratorParameters) -> Generator {
    let wave_type = params.wave_type;
    let mut g = Generator {
      params,
      volume: 0.2,
      oscillator: Oscillator::new(wave_type),
      hlpf: HighLowPassFilter::new(),
      envelope: Envelope::new(),
      phaser: Phaser::new(),
      rep_time: 0,
      rep_limit: 0,
      exhausted: false,
    };

    g.reset();

    g
  }

  /// Resets the generator to the beginning of the sound effect.
  pub fn reset(&mut self) {
    self.restart();
    self.exhausted = false;
    self.envelope.reset(
      self.params.env_attack,
      self.params.env_sustain,
      self.params.env_decay,
      self.params.env_punch,
    );
    self
      .phaser
      .reset(self.params.pha_offset, self.params.pha_ramp);

    self.oscillator.reset_phase();
    self
      .oscillator
      .reset_vibrato(self.params.vib_speed, self.params.vib_strength);
    self.oscillator.reset_noise();

    self.rep_time = 0;
    self.rep_limit = ((1.0 - self.params.repeat_speed).powi(2) * 20_000.0 * 32.0) as i32;

    if self.params.repeat_speed == 0.0 {
      self.rep_limit = 0;
    }
  }

  /// Resets only the oscillator and band pass filter.
  fn restart(&mut self) {
    self.hlpf.reset(
      self.params.lpf_resonance,
      self.params.lpf_freq,
      self.params.lpf_ramp,
      self.params.hpf_freq,
      self.params.hpf_ramp,
    );
    self.oscillator.reset(
      self.params.wave_type,
      self.params.base_freq,
      self.params.freq_limit,
      self.params.freq_ramp,
      self.params.freq_dramp,
      self.params.duty,
      self.params.duty_ramp,
      self.params.arp_speed,
      self.params.arp_mod,
    );
  }
}
impl Signal for Generator {
  type Frame = Mono<f32>;
  fn next(&mut self) -> Self::Frame {
    self.rep_time += 1;

    if self.rep_limit != 0 && self.rep_time >= self.rep_limit {
      self.rep_time = 0;
      self.restart();
    }

    self.oscillator.advance();
    self.envelope.advance();
    self.phaser.advance();

    let samples = self
      .oscillator
      .by_ref()
      .chain_filter(&mut self.envelope)
      .chain_filter(&mut self.hlpf)
      .chain_filter(&mut self.phaser)
      .take(8)
      .collect::<Vec<_>>();

    let len = samples.len() as f32;
    if len > 0. {
      let sample = samples.into_iter().sum::<f32>() / len;

      [(sample * self.volume).min(1.0).max(-1.0)]
    } else {
      if !self.exhausted {
        self.exhausted = true;
      }

      Self::Frame::EQUILIBRIUM
    }
  }
  fn is_exhausted(&self) -> bool {
    self.exhausted
  }
}

/// Generate a random `f32` using `rng` in the range [`from`...`until`).
fn rand_f32(rng: &mut SmallRng, from: f32, until: f32) -> f32 {
  from + (until - from) * rng.gen::<f32>()
}
/// Generate a random `f64` using `rng` in the range [`from`...`until`).
fn rand_f64(rng: &mut SmallRng, from: f64, until: f64) -> f64 {
  from + (until - from) * rng.gen::<f64>()
}
/// Generate a random `bool` using `rng` with `chance_true`:`chance_false` odds of being true.
fn rand_bool(rng: &mut SmallRng, chance_true: u32, chance_false: u32) -> bool {
  rng.gen::<u32>() % (chance_true + chance_false) < chance_true
}
/// Pick a random element from `slice` using `rng`.
fn rand_element<T: Copy>(rng: &mut SmallRng, slice: &[T]) -> T {
  slice[rng.gen::<u32>() as usize % slice.len()]
}
