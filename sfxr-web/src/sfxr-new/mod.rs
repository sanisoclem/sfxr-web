// use dasp::{frame::Mono, Frame, Sample, Signal};
// use std::f32::consts::PI;

// pub struct GeneratorParameters {
//   pub wave_type: WaveType,
//   pub base_freq: f64,
//   pub freq_limit: f64,
//   pub freq_ramp: f64,
//   pub freq_dramp: f64,
//   pub duty: f32,
//   pub duty_ramp: f32,
//   pub vib_strength: f64,
//   pub vib_speed: f64,
//   pub vib_delay: f32,
//   pub env_attack: f32,
//   pub env_sustain: f32,
//   pub env_decay: f32,
//   pub env_punch: f32,
//   pub lpf_resonance: f32,
//   pub lpf_freq: f32,
//   pub lpf_ramp: f32,
//   pub hpf_freq: f32,
//   pub hpf_ramp: f32,
//   pub pha_offset: f32,
//   pub pha_ramp: f32,
//   pub repeat_speed: f32,
//   pub arp_speed: f32,
//   pub arp_mod: f64,
// }

// pub fn test()

// pub struct FnSignalSource<F, D, S> {
//   data: D,
//   func: fn(&mut D) -> F,
//   param_signal: Option<S>,
// }

// impl<F, D, S> Signal for FnSignalSource<F, D, S>
// where
//   F: Frame,
//   S: Signal,
//   D: From<<S as Signal>::Frame>,
// {
//   type Frame = F;

//   fn next(&mut self) -> Self::Frame {
//     if let Some(param_signal) = self.param_signal {
//       self.data = param_signal.next().into()
//     }
//     (self.func)(&mut self.data);
//   }
// }

// impl<F, Data> FnSignalSource<F, Data> {
//   pub fn new(data: Data, func: fn(&mut Data) -> F) -> FnSignalSource<F, Data> {
//     FnSignalSource {
//       data,
//       func,
//       reset,
//       sampling_rate,
//     }
//   }
// }

// impl<S> FnSignalSource<Mono<S>, S>
// where
//   S: Sample,
// {
//   pub fn constant(value: S, sampling_rate: SamplingRate) -> FnSignalSource<Mono<S>, S>
//   where
//     S: Clone,
//   {
//     FnSignalSource::new(
//       value,
//       |d, _, sample| *sample = [d.clone()],
//       |_| {},
//       sampling_rate,
//     )
//   }
// }


// impl FnSignalSource<Mono<f32>, Period> {
//   pub fn sine(freq: FrequencyHz, sampling_rate: SamplingRate) -> FnSignalSource<Mono<f32>, Period> {
//     let period = freq_to_period(freq, sampling_rate);
//     FnSignalSource::new(
//       period,
//       |period, phase, sample| {
//         let fp = phase as f32 / *period as f32;
//         *sample = [(fp * 2.0 * PI).sin()]
//       },
//       |_| {},
//       sampling_rate,
//     )
//   }

//   pub fn triangle(
//     freq: FrequencyHz,
//     sampling_rate: SamplingRate,
//   ) -> FnSignalSource<Mono<f32>, Period> {
//     let period = freq_to_period(freq, sampling_rate);
//     FnSignalSource::new(
//       period,
//       |period, phase, sample| {
//         let fp = phase as f32 / *period as f32;
//         *sample = [1.0 - fp * 2.0]
//       },
//       |_| {},
//       sampling_rate,
//     )
//   }

//   pub fn sawtooth(
//     freq: FrequencyHz,
//     sampling_rate: SamplingRate,
//   ) -> FnSignalSource<Mono<f32>, Period> {
//     let period = freq_to_period(freq, sampling_rate);
//     FnSignalSource::new(
//       period,
//       |period, phase, sample| {
//         let fp = phase as f32 / *period as f32;
//         *sample = [fp * 2.0 - 1.0]
//       },
//       |_| {},
//       sampling_rate,
//     )
//   }
// }

// pub struct Automator<TParamSource, TSource, TFrame> {
//   param_source: TParamSource,
//   source: TSource,
//   func: fn(&TFrame, &mut TSource),
//   param_sampling_mod: SamplingRate,
//   param_buffer: TFrame,
// }

// impl<F, FParam, TParamSource, TSource> SignalSource<F> for Automator<TParamSource, TSource, FParam>
// where
//   F: Frame,
//   FParam: Frame,
//   TParamSource: SignalSource<FParam>,
//   TSource: SignalSource<F>,
// {
//   fn get_sampling_rate(&self) -> SamplingRate {
//     self.source.get_sampling_rate()
//   }
//   fn signal_requested(&mut self, phase: Phase, buffer: &mut F) {
//     if (phase % self.param_sampling_mod) == 0 {
//       self
//         .param_source
//         .signal_requested(phase / self.param_sampling_mod, &mut self.param_buffer);
//       (self.func)(&self.param_buffer, &mut self.source);
//     }

//     self.source.signal_requested(phase, buffer);
//   }

//   fn reset(&mut self) {
//     self.source.reset();
//     self.param_source.reset();
//   }
// }

// struct SoundEffectGenerator<T> {
//   source: T,
// }

// impl<T> SoundEffectGenerator<T>
// where
//   T: SignalSource<[f32; 1]>,
// {
//   pub fn new(sampling_rate: SamplingRate) -> Self {
//     let base_signal = FnSignalSource::triangle(0.0, sampling_rate: SamplingRate * 8);
//     let vibrato = FnSignalSource::sine(0.0, sampling_rate: SamplingRate);
//     //let arpeggiator = FnSignalSource::square();

//     unimplemented!()
//   }
// }
