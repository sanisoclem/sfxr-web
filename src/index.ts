import init, { FmOsc } from 'sfxr-rs'

init()
  .then((m: any) => {
    console.log('init wasm-pack')
    let fm = null

    const play_button = document.getElementById('play')
    play_button.addEventListener('click', (_event) => {
      if (fm === null) {
        fm = new m.FmOsc()
        fm.set_note(50)
        fm.set_fm_frequency(0)
        fm.set_fm_amount(0)
        fm.set_gain(0.8)
      } else {
        fm.free()
        fm = null
      }
    })

    const primary_slider = document.getElementById('primary_input')
    primary_slider.addEventListener('input', (event) => {
      const target = event.target as HTMLInputElement;
      if (fm) {
        fm.set_note(parseInt(target.value))
      }
    })

    const fm_freq = document.getElementById('fm_freq')
    fm_freq.addEventListener('input', (event) => {
      const target = event.target as HTMLInputElement;
      if (fm) {
        fm.set_fm_frequency(parseFloat(target.value))
      }
    })

    const fm_amount = document.getElementById('fm_amount')
    fm_amount.addEventListener('input', (event) => {
      const target = event.target as HTMLInputElement;
      if (fm) {
        fm.set_fm_amount(parseFloat(target.value))
      }
    })
  })
  .catch(console.error)
