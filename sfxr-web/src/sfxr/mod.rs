use std::f32::consts::PI;
use dasp::{Sample, Frame};

type FrequencyHz = f32;
type SamplingRate = usize;
type Phase = usize;
type Period = usize;

pub fn freq_to_period(freq: FrequencyHz, samples: SamplingRate) -> Period {
  // cannot synth freqs that cannot be expressed in whole samples
  (freq / samples as f32).round() as Period
}

pub trait SignalSource<F> where F: Frame {
  fn get_sampling_rate(&self) -> SamplingRate;
  fn signal_requested(&mut self, phase: Phase, buffer: &F);
  fn reset(&mut self);
}

pub enum WaveType {
  Sine,
  Triangle,
  Sawtooth,
}

pub struct WaveOscillator {
  wave_type: WaveType,
  period: Period,
  sampling_rate: SamplingRate,
}

impl SignalSource<[f32; 1]> for WaveOscillator {
  fn get_sampling_rate(&self) -> SamplingRate {
    self.sampling_rate
  }
  fn signal_requested(&mut self, phase: Phase, sample: &[f32; 1]) {
      let fp = phase as f32 / self.period as f32;
      *sample = [match self.wave_type {
        WaveType::Triangle => 1.0 - fp * 2.0,
        WaveType::Sine => (fp * 2.0 * PI).sin(),
        WaveType::Sawtooth => fp * 2.0 - 1.0,
      }];
  }
  fn reset(&mut self) {

  }
}

impl WaveOscillator {
  pub fn new(wave_type: WaveType, freq: FrequencyHz, sampling_rate: SamplingRate) -> Self {
    WaveOscillator {
      wave_type,
      period: freq_to_period(freq: FrequencyHz, sampling_rate: SamplingRate),
      sampling_rate,
    }
  }
}

pub struct Automator<TParamSource, TSource, TFrame> {
  param_source: TParamSource,
  source: TSource,
  func: fn(&TFrame, &mut TSource),
  param_sampling_mod: SamplingRate,
  param_buffer: TFrame,
}

impl<F, FParam, TParamSource, TSource> SignalSource<F> for Automator<TParamSource, TSource, FParam>
where F: Frame,
  FParam: Frame,
  TParamSource: SignalSource<FParam>,
  TSource: SignalSource<F> {

  fn get_sampling_rate(&self) -> SamplingRate {
    self.source.get_sampling_rate()
  }
  fn signal_requested(&mut self, phase: Phase, buffer: &F) {
    if (phase % self.param_sampling_mod) == 0 {
      self.param_source.signal_requested(phase / self.param_sampling_mod, &mut self.param_buffer);
      (self.func)(&self.param_buffer, &mut self.source);
    }

    self.source.signal_requested(phase, buffer);
  }

  fn reset(&mut self) {
    self.source.reset();
    self.param_source.reset();
  }
}


struct SoundEffectGenerator<T> {
  source: T
}

impl<T> SoundEffectGenerator<T> where T: SignalSource<[f32; 1]> {
  pub fn new() -> Self {
    let osc =
      create_oscillator()
      .automate_params(|p| p.frequency.automate(|f| f.set_value(440.0)));
    let hpf = create_filter(FilterType::HighPass);

  }
}