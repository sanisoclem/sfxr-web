<script lang="ts">
  import init, { SoundEffectGenerator } from 'sfxr-web';
  import Button from '../components/Button.svelte';
  import Slider from '../components/Slider.svelte';
  import { LayerCake, Svg } from 'layercake';
  import Line from '../components/Line.svelte';
  import { onMount } from 'svelte';
  import { afterNavigate, goto } from '$app/navigation';
  import ButtonLink from './ButtonLink.svelte';

  let generator: SoundEffectGenerator;
  let preset: string | undefined;
  let seed: bigint | undefined;
  let shareSeed: string | undefined;
  let url: string | null = null;
  let data: Array<{ x: number; y: number }> = [];

  onMount(async () => {
    await init();
    generator = new SoundEffectGenerator();
  });

  afterNavigate((nav) => {
    setTimeout(() => {
      const split = nav.to.pathname.split('/').pop()?.split('-');
      if (split && split.length === 2) {
        loadSeed(split[0].toLowerCase(), BigInt(`0x${split[1]}`));
      }
    }, 200);
  });

  const randomizePreset = (name: string, s?: bigint) => () => {
    const seed = s || randombigint();
    goto(`/${name.toUpperCase()}-${seed.toString(16).toUpperCase()}`, {
      noscroll: true
    });
  };

  const loadSeed = (name: string, s: bigint) => {
    seed = s;
    preset = name;
    shareSeed = `${name.toUpperCase()}-${s.toString(16).toUpperCase()}`;

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

  const randombigint = (): bigint => {
    var hex = [...Array(8).keys()]
      .map((_) => Math.floor(Math.random() * 255))
      .map((i) => i.toString(16).padStart(2, '0'))
      .join('');

    return BigInt(`0x${hex}`);
  };

  function dump() {
    const blob = new Blob([Uint8Array.from(generator.export_wav())], {
      type: 'audio/wav'
    });
    url = window.URL.createObjectURL(blob);
    data =
      generator.export_raw()?.map((v: number, i: number) => {
        return {
          x: i,
          y: v
        };
      }) || [];
  }
</script>

<section class="controls w-full flex flex-row justify-center gap-4 flex-wrap">
  <Button on:click={mutate} disabled={!preset}>Mutate</Button>
  <Button on:click={play}>Play</Button>
  <ButtonLink hidden={url === null} url={url ?? ''} download={shareSeed}>Download</ButtonLink>
</section>
<section class="seed">
  <div class="text-center" hidden={!shareSeed}>
    <label>
      <input
        class="text-gray-500 tracking-wide p-2 border-solid border-2 border-emerald-500 text-center"
        type="text"
        placeholder="Seed"
        value={shareSeed}
      />
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
<section class="presets w-full flex flex-row justify-center gap-4 flex-wrap">
  <Button on:click={randomizePreset('pickup')}>Pickup</Button>
  <Button on:click={randomizePreset('laser')}>Laser</Button>
  <Button on:click={randomizePreset('explosion')}>Explosion</Button>
  <Button on:click={randomizePreset('powerup')}>Powerup</Button>
  <Button on:click={randomizePreset('hit')}>Hit</Button>
  <Button on:click={randomizePreset('jump')}>Jump</Button>
  <Button on:click={randomizePreset('blip')}>Blip</Button>
</section>

<style>
  .wave-display {
    width: 100%;
    height: 150px;
  }
</style>
