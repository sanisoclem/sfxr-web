<script context="module" lang="ts">
  // import type { LoadInput } from '@sveltejs/kit/types/internal';
  export const prerender = true;
  // export let seed; // this is not updating when navigating stays in the same component
  // export function load(args: LoadInput) {
  //   seed = args.params.seed;
  //   return {
  //     status: 200
  //   };
  // }
</script>

<script lang="ts">
  // import { goto } from '$app/navigation';

  import SoundEffectGenerator from '../components/SoundEffectGenerator.svelte';

  // function randomized(e) {
  //   goto(`/${e.detail.shareSeed}`);
  // }
</script>

<svelte:head>
  <title>Sound Effect Generator</title>
</svelte:head>

<div class="text-white bg-gray-900 min-h-screen flex flex-col justify-center items-center">
  <div class="max-w-screen-lg p-4">
    <h1 class="mb-8 text-2xl text-center w-full">Sound Effect Generator</h1>
    <div class="flex flex-col gap-4">
      <SoundEffectGenerator />
    </div>
  </div>
</div>
