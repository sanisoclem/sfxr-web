pub enum SoundType {
  SquareWave,
  Sawtooth,
  SineWave,
  Noise,
}

pub struct SoundEffectParameters {
  pub p_base_freq: f32,
  pub p_freq_limit: f32,
  // slide
  pub p_freq_ramp: f32,
  // delta slide
  pub p_freq_dramp: f32,

  // only applicable for square wave oscillator
  pub p_duty: f32,
  pub p_duty_ramp: f32,

  pub p_vib_strength: f32,
  pub p_vib_speed: f32,
  pub p_vib_delay: f32,

  pub p_env_attack: f32,
  pub p_env_sustain: f32,
  pub p_env_decay: f32,
  pub p_env_punch: f32,

  pub filter_on: bool,

  pub p_lpf_resonance: f32,
  pub p_lpf_freq: f32,
  pub p_lpf_ramp: f32,

  pub p_hpf_freq: f32,
  pub p_hpf_ramp: f32,

  // phaser
  pub p_pha_offset: f32,
  pub p_pha_ramp: f32,

  //arpeggiator
  pub p_arp_speed: f32,
  pub p_arp_mod: f32,

  // repeat playback, don't repeat if zero
  pub p_repeat_speed: f32,
}