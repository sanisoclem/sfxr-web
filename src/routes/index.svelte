<script context="module" lang="ts">
  export const prerender = true;
</script>

<script lang="ts">
  import logo from '../assets/logo.svg';
  import SoundEffectGenerator from '../components/SoundEffectGenerator.svelte';
</script>

<svelte:head>
  <title>Sound Effect Generator</title>
</svelte:head>

<div class="text-white bg-gray-900 min-h-screen flex flex-col justify-center items-center">
  <div class="w-full max-w-screen-lg p-4 flex justify-center flex-col items-center">
    <img class="" src={logo} width="150" height="80" />
    <h1 class="mb-8 text-2xl text-center w-full">Sound Effect Generator</h1>

    <div class="flex flex-col gap-4">
      <SoundEffectGenerator />
    </div>
  </div>
</div>
