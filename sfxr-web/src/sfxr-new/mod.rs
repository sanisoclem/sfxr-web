use std::f32::consts::PI;
use dasp::{Sample, Frame, frame::Mono};

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
  fn signal_requested(&mut self, phase: Phase, buffer: &mut F);
  fn reset(&mut self);
}

pub struct FnSignalSource<F, D> {
  data: D,
  func: fn(&D, Phase, &mut F),
  reset: fn(data: &mut D),
  sampling_rate: SamplingRate,
}

impl<F, D> SignalSource<F> for FnSignalSource<F, D> where F: Frame {
  fn get_sampling_rate(&self) -> SamplingRate {
    self.sampling_rate
  }
  fn signal_requested(&mut self, phase: Phase, sample: &mut F) {
    (self.func)(&self.data, phase, sample);
  }
  fn reset(&mut self) {
    (self.reset)(&mut self.data);
  }
}
impl<Data, F> FnSignalSource<F, Data> {
  pub fn new(data: Data, func: fn(&Data, Phase, &mut F), reset: fn(data: &mut Data), sampling_rate: SamplingRate) -> FnSignalSource<F, Data> {
    FnSignalSource {
      data,
      func,
      reset,
      sampling_rate,
    }
  }
}
impl<TSample> FnSignalSource<Mono<TSample>, TSample> {
  pub fn constant(value: TSample, sampling_rate: SamplingRate) -> FnSignalSource<Mono<TSample>, TSample> where TSample: Sample + Clone {
    FnSignalSource::new(value, |d, _, sample| *sample = [d.clone()], |_| {}, sampling_rate)
  }
}

impl FnSignalSource<Mono<f32>, Period> {
  pub fn sine(freq: FrequencyHz, sampling_rate: SamplingRate) -> FnSignalSource<Mono<f32>, Period> {
    let period = freq_to_period(freq, sampling_rate);
    FnSignalSource::new(period, |period, phase, sample| {
      let fp = phase as f32 / *period as f32;
      *sample = [(fp * 2.0 * PI).sin()]
    }, |_| {}, sampling_rate)
  }

  pub fn triangle(freq: FrequencyHz, sampling_rate: SamplingRate) -> FnSignalSource<Mono<f32>, Period> {
    let period = freq_to_period(freq, sampling_rate);
    FnSignalSource::new(period, |period, phase, sample| {
      let fp = phase as f32 / *period as f32;
      *sample = [1.0 - fp * 2.0]
    }, |_| {}, sampling_rate)
  }

  pub fn sawtooth(freq: FrequencyHz, sampling_rate: SamplingRate) -> FnSignalSource<Mono<f32>, Period> {
    let period = freq_to_period(freq, sampling_rate);
    FnSignalSource::new(period, |period, phase, sample| {
      let fp = phase as f32 / *period as f32;
      *sample = [fp * 2.0 - 1.0]
    }, |_| {}, sampling_rate)
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
  fn signal_requested(&mut self, phase: Phase, buffer: &mut F) {
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
  pub fn new(sampling_rate: SamplingRate) -> Self {
    let base_signal = FnSignalSource::triangle(0.0, sampling_rate: SamplingRate * 8);
    let vibrato = FnSignalSource::sine(0.0, sampling_rate: SamplingRate);
    //let arpeggiator = FnSignalSource::square();

    unimplemented!()
  }
}