<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
  import init, { EffectGenerator } from 'sfxr-web';
  import Button from '../components/Button.svelte';
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

<div class="text-white bg-gray-900 min-h-screen flex flex-col justify-center items-center">
	<h1 class="mb-8 text-2xl text-center w-full">Sound Effect Generator</h1>
  <div class="w-full flex flex-row justify-center gap-4 flex-wrap">
    <Button on:click={mutate}>Mutate</Button>
    <Button on:click={randomize("pickup")}>Pickup</Button>
    <Button on:click={randomize("laser")}>Laser</Button>
    <Button on:click={randomize("explosion")}>Explosion</Button>
    <Button on:click={randomize("powerup")}>Powerup</Button>
    <Button on:click={randomize("hit")}>Hit</Button>
    <Button on:click={randomize("jump")}>Jump</Button>
    <Button on:click={randomize("blip")}>Blip</Button>
  </div>
</div>