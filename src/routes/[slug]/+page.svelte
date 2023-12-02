<script lang="ts">
  import { afterNavigate } from '$app/navigation';

  let shareSeed: string | undefined;

  afterNavigate((nav) => {
    const split = nav.to?.url.pathname.split('/').pop()?.split('-');
      if (split && split.length === 2) {
        shareSeed = `${split[0].toUpperCase()}-${BigInt(`0x${split[1]}`).toString(16).toUpperCase()}`
      }
  });
</script>

<svelte:head>
  <title>{shareSeed} - Sound Effect Generator</title>
</svelte:head>