import init, { EffectGenerator } from 'sfxr-rs'

init()
  .then((m: any) => {
    console.log('init wasm-pack')
    let gen = new EffectGenerator();

    const play_button = document.getElementById('play')
    play_button.addEventListener('click', (_event) => {
      if (gen !== null) {
        gen.mutate();
      }
    })
  })
  .catch(console.error)
