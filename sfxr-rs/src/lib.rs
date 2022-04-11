use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext};

/// Converts a midi note to frequency
///
/// A midi note is an integer, generally in the range of 21 to 108
pub fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

#[wasm_bindgen]
pub struct EffectGenerator {
    ctx: AudioContext,
    sample: sfxr::Sample,
}
#[wasm_bindgen]
impl EffectGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<EffectGenerator, JsValue> {
        let ctx = web_sys::AudioContext::new()?;
        Ok(EffectGenerator {
            ctx,
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
        Ok(())
    }

    #[wasm_bindgen]
    pub fn mutate(&mut self) -> Result<(), JsValue> {
        self.sample
            .mutate(Some(SmallRng::from_entropy().next_u64()));
        self.play()
    }

    pub fn randomize() {
        unimplemented!()
    }
}
impl Drop for EffectGenerator {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}
