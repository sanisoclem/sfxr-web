<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
  import init, { EffectGenerator } from 'sfxr-web';
  import { onMount } from 'svelte';

  let generator: EffectGenerator;

  onMount(async () => {
    await init();
    generator = new EffectGenerator();
  });


  const randomize = (name: string) => () => {
    if (!generator) return;
    generator.randomize(name);
  }

  function mutate() {
    if (!generator) return;
    generator.mutate();
    console.log("mutating")
  }
</script>

<svelte:head>
	<title>Sound Effect Generator</title>
</svelte:head>

<div class="text-white bg-gray-900 min-h-screen">
	<h1>Sound Effect Generator</h1>
  <div class="mx-auto">
    <button on:click={mutate} type="button">Mutate</button>
    <button on:click={randomize("pickup")} type="button">Pickup</button>
    <button on:click={randomize("laser")} type="button">Laser</button>
    <button on:click={randomize("explosion")} type="button">Explosion</button>
    <button on:click={randomize("powerup")} type="button">Powerup</button>
    <button on:click={randomize("hit")} type="button">Hit</button>
    <button on:click={randomize("jump")} type="button">Jump</button>
    <button on:click={randomize("blip")} type="button">Blip</button>
  </div>
</div>