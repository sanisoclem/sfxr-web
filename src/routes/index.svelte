<script context="module" lang="ts">
  export const prerender = true;
</script>

<script lang="ts">
  import init, { EffectGenerator } from 'sfxr-web';
  import Button from '../components/Button.svelte';
  import Slider from '../components/Slider.svelte';
  import { onMount } from 'svelte';

  let generator: EffectGenerator;

  onMount(async () => {
    await init();
    generator = new EffectGenerator();
  });

  const randomize = (name: string) => () => {
    if (!generator) return;
    generator.randomize(name);
  };

  function mutate() {
    if (!generator) return;
    generator.mutate();
    console.log('mutating');
  }
</script>

<svelte:head>
  <title>Sound Effect Generator</title>
</svelte:head>

<div class="text-white bg-gray-900 min-h-screen flex flex-col justify-center items-center">
  <div class="max-w-screen-lg p-4">
    <h1 class="mb-8 text-2xl text-center w-full">Sound Effect Generator</h1>
    <div class="w-full flex flex-row justify-center gap-4 flex-wrap">
      <Button on:click={mutate}>Mutate</Button>
      <Button on:click={randomize('pickup')}>Pickup</Button>
      <Button on:click={randomize('laser')}>Laser</Button>
      <Button on:click={randomize('explosion')}>Explosion</Button>
      <Button on:click={randomize('powerup')}>Powerup</Button>
      <Button on:click={randomize('hit')}>Hit</Button>
      <Button on:click={randomize('jump')}>Jump</Button>
      <Button on:click={randomize('blip')}>Blip</Button>
    </div>
    <div class="my-8 border-2 border-rose-500 p-8">
      <Slider label="Attack" value={0} units="samples"></Slider>
      <Slider label="Sustain" value={0} units="samples"></Slider>
      <Slider label="Sustain Punch" value={0} max="1" step="0.01"></Slider>
      <Slider label="Decay" value={0} units="samples"></Slider>

      <Slider label="Start Frequency" value={0} units="hz"></Slider>
      <Slider label="Min Frequency" value={0} units="hz"></Slider>
      <Slider label="Ramp" value={0} min="-1" max="1" step="0.01"></Slider>
      <Slider label="DRamp" value={0} min="-1" max="1" step="0.01"></Slider>

      <Slider label="Vibrato strength" value={0} max="1" step="0.01"></Slider>
      <Slider label="Speed" value={0} max="1" step="0.01"></Slider>
      <Slider label="Delay" value={0} max="1" step="0.01"></Slider>

      <Slider label="Low Pass Filter Resonance" value={0} max="1" step="0.01"></Slider>
      <Slider label="Frequence" value={0} units="Hz"></Slider>
      <Slider label="Ramp" value={0} max="1" min="-1" step="0.01"></Slider>

      <Slider label="High Pass Filter Frequency" value={0} units="Hz"></Slider>
      <Slider label="Ramp" value={0} max="1" min="-1" step="0.01"></Slider>

      <Slider label="Phase Offset" value={0} min="-1" max="1" step="0.01"></Slider>
      <Slider label="Ramp" value={0} min="-1" max="1" step="0.01"></Slider>

      <Slider label="Repeat Speed" value={0} max="1" step="0.01"></Slider>

      <Slider label="Arpeggio Speed" value={0} max="1" step="0.01"></Slider>
      <Slider label="Arpeggio Mod" value={0} min="-1" max="1" step="0.01"></Slider>
    </div>
  </div>
</div>

