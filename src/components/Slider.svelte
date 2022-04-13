<script context="module">
  let counter = 0;
</script>

<script lang="ts">
  export let label;
  export let min = undefined;
  export let max = undefined;
  export let step = undefined;
  export let value = 0;
  export let units = undefined;
</script>

<style>
  input[type='range'] {
    @apply appearance-none w-full h-6 p-0 bg-transparent focus:outline-none focus:ring-0 focus:shadow-none;
  }

  input[type='range']::-webkit-slider-runnable-track {
    @apply w-full h-4 cursor-pointer bg-gray-300;
  }
  input[type='range']::-webkit-slider-thumb {
    @apply shadow-none bg-white h-6 w-3 cursor-pointer rounded-none border-none;
  }
  input[type='range']:focus::-webkit-slider-runnable-track {
    @apply bg-gray-600;
  }
  input[type='range']:focus::-webkit-slider-thumb {
    @apply  bg-rose-500;
  }

  input[type='range']::-moz-range-track {
    @apply w-full h-1 cursor-pointer bg-gray-600;
  }
  input[type='range']::-moz-range-thumb {
    @apply shadow-none bg-white h-6 w-3 cursor-pointer rounded-none border-none;
  }
  input[type='range']:focus::-moz-range-track {
    @apply bg-gray-600;
  }
  input[type='range']:focus::-moz-range-thumb {
    @apply  bg-rose-500;
  }
</style>

<div class="relative pt-1">
  <label for={`slider${counter}`}>{label}</label>
  <input id={`slider${counter}`} type="range" {min} {max} {step} bind:value={value} />
  {value} {units || ''}
</div>
