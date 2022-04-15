use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use web_sys::{AudioContext, AudioBufferSourceNode};

mod sfxr;

#[derive(Serialize, Deserialize)]
pub struct DumpOutput {
  pub raw: Vec<f32>
}

#[wasm_bindgen]
pub struct SoundEffectGenerator {
    ctx: AudioContext,
    src: AudioBufferSourceNode,
    sample: sfxr::Sample,
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
            sample: sfxr::Sample::pickup(Some(SmallRng::from_entropy().next_u64())),
        })
    }

    #[wasm_bindgen]
    pub fn play(&mut self) -> Result<(), JsValue> {
        let generator = sfxr::Generator::new(self.sample.clone());
        let buffer = generator.into_iter().collect::<Vec<_>>();
        let sample_rate = self.ctx.sample_rate();
        let abuffer = self
            .ctx
            .create_buffer(1, buffer.len() as u32, sample_rate)?;
        abuffer.copy_to_channel(&buffer, 0)?;
        let src = self.ctx.create_buffer_source()?;

        src.set_buffer(Some(&abuffer));
        src.connect_with_audio_node(&self.ctx.destination())?;
        src.start()?;

        self.src.disconnect()?;
        self.src = src;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn mutate(&mut self) -> Result<(), JsValue> {
        let seed = Some(SmallRng::from_entropy().next_u64());
        self.sample.mutate(seed);
        self.play()
    }

    #[wasm_bindgen]
    pub fn randomize(&mut self, name: String) -> Result<(), JsValue> {
        let seed = Some(SmallRng::from_entropy().next_u64());
        self.sample = match name.as_str() {
            "pickup" => sfxr::Sample::pickup(seed),
            "laser" => sfxr::Sample::laser(seed),
            "explosion" => sfxr::Sample::explosion(seed),
            "powerup" => sfxr::Sample::powerup(seed),
            "hit" => sfxr::Sample::hit(seed),
            "jump" => sfxr::Sample::jump(seed),
            "blip" => sfxr::Sample::blip(seed),
            _ => sfxr::Sample::pickup(seed)
        };

        self.play()
    }

    pub fn dump(&mut self) -> JsValue {
        let generator = sfxr::Generator::new(self.sample.clone());
        let output = DumpOutput {
            raw: generator.into_iter().collect::<Vec<_>>()
        };
        JsValue::from_serde(&output).unwrap()
    }
}

impl Drop for SoundEffectGenerator {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}
