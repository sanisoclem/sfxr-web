<script lang="ts">
  import init, { SoundEffectGenerator } from 'sfxr-web';
  import Button from '../components/Button.svelte';
  import Slider from '../components/Slider.svelte';
  import { LayerCake, Svg } from 'layercake';
  import Line from '../components/Line.svelte';
  import { onMount } from 'svelte';
  // import { createEventDispatcher } from 'svelte';
  import { afterNavigate, goto } from '$app/navigation';

  // const dispatch = createEventDispatcher();

  let generator: SoundEffectGenerator;
  let preset: string | undefined;
  let seed: BigInt | undefined;
  let shareSeed: String | undefined;
  let data = [];

  onMount(async () => {
    await init();
    generator = new SoundEffectGenerator();
  });

  afterNavigate(nav => {
    setTimeout(() => {
      const split = nav.to.pathname.split('/').pop()?.split('-');
      if (split && split.length === 2) {
        loadSeed(split[0].toLowerCase(), BigInt(`0x${split[1]}`));
      }
    }, 200);
  })


  const randomizePreset = (name: string, s?: BigInt) => () => {
    const seed = s || randomBigInt();
    goto(`/${name.toUpperCase()}-${seed.toString(16).toUpperCase()}`);
    // dispatch('randomize', { preset: name, seed, shareSeed: `` });
  };

  const loadSeed = (name: string, s: BigInt) => {
    seed = s;
    preset = name;
    shareSeed = `${name.toUpperCase()}-${s.toString(16)}`;

    if (!generator) return;
    generator.preset(preset, seed);
    dump();
  };

  const mutate = () => {
    if (!preset) return;
    randomizePreset(preset);
  };

  const play = () => {
    if (!generator) return;
    generator.play();
  };

  const randomBigInt = (): BigInt => {
    var hex = [...Array(8).keys()]
      .map((_) => Math.floor(Math.random() * 255))
      .map((i) => i.toString(16).padStart(2, '0'))
      .join('');

    return BigInt(`0x${hex}`);
  };

  function dump() {
    data =
      generator.export_raw()?.map((v, i) => {
        return {
          x: i,
          y: v
        };
      }) || [];
  }
</script>

<style>
  .wave-display {
    width: 100%;
    height: 300px;
  }
</style>

<section class="controls w-full flex flex-row justify-center gap-4 flex-wrap">
  <Button on:click={mutate} disabled={!preset}>Mutate</Button>
  <Button on:click={play}>Play</Button>
  <!-- <Button>Export</Button>
  <Button>Share</Button> -->
</section>
<section class="seed">
  <div class="text-center" hidden={!shareSeed}>
    <label>
      <input
        class="text-gray-500 tracking-wide p-2 border-solid border-2 border-emerald-500 text-center"
        type="text"
        placeholder="Seed"
        value={shareSeed} />
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
  <Button on:click={randomizePreset('pickup')}>Pickup</Button>
  <Button on:click={randomizePreset('laser')}>Laser</Button>
  <Button on:click={randomizePreset('explosion')}>Explosion</Button>
  <Button on:click={randomizePreset('powerup')}>Powerup</Button>
  <Button on:click={randomizePreset('hit')}>Hit</Button>
  <Button on:click={randomizePreset('jump')}>Jump</Button>
  <Button on:click={randomizePreset('blip')}>Blip</Button>
</section>
