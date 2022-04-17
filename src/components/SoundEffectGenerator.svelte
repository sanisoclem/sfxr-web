<script lang="ts">
  import init, { SoundEffectGenerator } from 'sfxr-web';
  import Button from '../components/Button.svelte';
  import Slider from '../components/Slider.svelte';
  import { LayerCake, Svg } from 'layercake';
  import Line from '../components/Line.svelte';
  import { onMount } from 'svelte';

  let generator: SoundEffectGenerator;
  let data = [];

  onMount(async () => {
    await init();
    generator = new SoundEffectGenerator();
  });

  const randomize = (name: string) => () => {
    if (!generator) return;
    generator.randomize(name);
    dump();
  };

  function mutate() {
    if (!generator) return;
    generator.mutate();
    dump();
  }

  function dump() {
    const result = generator.dump();
    data = result.raw.map((v, i) => {
      return {
        x: i,
        y: v
      };
    });
  }
</script>

<style>
  .wave-display {
    width: 100%;
    height: 300px;
  }
</style>

<section class="controls w-full flex flex-row justify-center gap-4 flex-wrap">
  <Button on:click={mutate}>Mutate</Button>
  <Button>Play</Button>
  <Button>Export</Button>
  <Button>Share</Button>
</section>
<section class="seed">
  <div class="text-center">
    <label>
      Seed:
      <input
        class="text-gray-500 tracking-wide p-2 border-solid border-2 border-emerald-500"
        type="text"
        placeholder="Seed"
        value="0xFF562TODO7448" />
    </label>
  </div>
</section>
<section class="wave-display">
  <LayerCake x="x" y="y" yDomain={[-1, 1]} {data}>
    <Svg>
      <Line />
    </Svg>
  </LayerCake>
</section>
<section class="advanced-generator my-8 border-2 border-rose-500 p-8 hidden">
  <Slider label="Attack" value={0} units="samples" />
  <Slider label="Sustain" value={0} units="samples" />
  <Slider label="Sustain Punch" value={0} max="1" step="0.01" />
  <Slider label="Decay" value={0} units="samples" />

  <Slider label="Start Frequency" value={0} units="hz" />
  <Slider label="Min Frequency" value={0} units="hz" />
  <Slider label="Ramp" value={0} min="-1" max="1" step="0.01" />
  <Slider label="DRamp" value={0} min="-1" max="1" step="0.01" />

  <Slider label="Vibrato strength" value={0} max="1" step="0.01" />
  <Slider label="Speed" value={0} max="1" step="0.01" />
  <Slider label="Delay" value={0} max="1" step="0.01" />

  <Slider label="Low Pass Filter Resonance" value={0} max="1" step="0.01" />
  <Slider label="Frequence" value={0} units="Hz" />
  <Slider label="Ramp" value={0} max="1" min="-1" step="0.01" />

  <Slider label="High Pass Filter Frequency" value={0} units="Hz" />
  <Slider label="Ramp" value={0} max="1" min="-1" step="0.01" />

  <Slider label="Phase Offset" value={0} min="-1" max="1" step="0.01" />
  <Slider label="Ramp" value={0} min="-1" max="1" step="0.01" />

  <Slider label="Repeat Speed" value={0} max="1" step="0.01" />

  <Slider label="Arpeggio Speed" value={0} max="1" step="0.01" />
  <Slider label="Arpeggio Mod" value={0} min="-1" max="1" step="0.01" />
</section>
<section class="presets w-full flex flex-row justify-center gap-4 flex-wrap">
  <Button on:click={randomize('pickup')}>Pickup</Button>
  <Button on:click={randomize('laser')}>Laser</Button>
  <Button on:click={randomize('explosion')}>Explosion</Button>
  <Button on:click={randomize('powerup')}>Powerup</Button>
  <Button on:click={randomize('hit')}>Hit</Button>
  <Button on:click={randomize('jump')}>Jump</Button>
  <Button on:click={randomize('blip')}>Blip</Button>
</section>
