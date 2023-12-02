use dasp::Signal;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{Cursor, Read, Seek, SeekFrom};
use wasm_bindgen::prelude::*;
use web_sys::{AudioBufferSourceNode, AudioContext};

mod sfxr;

#[derive(Serialize, Deserialize)]
pub struct DumpOutput {
  pub raw: Option<Vec<f32>>,
}

#[wasm_bindgen]
pub struct SoundEffectGenerator {
  ctx: AudioContext,
  src: AudioBufferSourceNode,
  params: sfxr::GeneratorParameters,
  raw_hash: u64,
  raw: Option<Vec<f32>>,
}

#[wasm_bindgen]
impl SoundEffectGenerator {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Result<SoundEffectGenerator, JsValue> {
    console_error_panic_hook::set_once();
    let ctx = web_sys::AudioContext::new()?;
    let src = ctx.create_buffer_source()?;

    Ok(SoundEffectGenerator {
      ctx,
      src,
      params: sfxr::GeneratorParameters::default(),
      raw_hash: 0,
      raw: None,
    })
  }

  #[wasm_bindgen]
  pub fn play(&mut self) -> Result<(), JsValue> {
    let mut hasher = DefaultHasher::new();
    self.params.hash(&mut hasher);
    let hash = hasher.finish();

    if hash != self.raw_hash {
      let signal = sfxr::Generator::from_params(self.params.clone());
      self.raw = Some(signal.until_exhausted().map(|[a]| a).collect::<Vec<_>>());
      self.raw_hash = hash;
    }

    if let Some(raw) = &self.raw {
      let sample_rate = self.ctx.sample_rate();
      let abuffer = self.ctx.create_buffer(1, raw.len() as u32, sample_rate)?;
      abuffer.copy_to_channel(&raw, 0)?;
      let src = self.ctx.create_buffer_source()?;
      src.set_buffer(Some(&abuffer));
      src.connect_with_audio_node(&self.ctx.destination())?;
      src.start()?;

      self.src.disconnect()?;
      self.src = src;
    }

    Ok(())
  }

  #[wasm_bindgen]
  pub fn preset(&mut self, name: &str, seed: u64) -> Result<(), JsValue> {
    self.params.random_preset(name, seed);
    self.play()
  }

  #[wasm_bindgen]
  pub fn export_raw(&mut self) -> JsValue {
    JsValue::from_serde(&self.raw).unwrap()
  }

  #[wasm_bindgen]
  pub fn export_wav(&mut self) -> JsValue {
    let data = wav::BitDepth::ThirtyTwoFloat(self.raw.clone().unwrap());
    let mut buf = Cursor::new(Vec::new());
    wav::write(
      wav::Header::new(
        wav::header::WAV_FORMAT_IEEE_FLOAT,
        1,
        self.ctx.sample_rate() as u32,
        32,
      ),
      &data,
      &mut buf,
    ).expect("im lazy");
    let mut out = Vec::new();
    buf.seek(SeekFrom::Start(0)).unwrap();
    buf.read_to_end(&mut out).unwrap();
    JsValue::from_serde(&out).unwrap()
  }
}

impl Drop for SoundEffectGenerator {
  fn drop(&mut self) {
    let _ = self.ctx.close();
  }
}
