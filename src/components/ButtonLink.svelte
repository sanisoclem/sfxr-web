<script lang="ts">
  export let url: string;
  export let hidden = false;
  export let download: string | undefined;
</script>

<a
  href={url}
  {hidden}
  {download}
  class="touch-none tracking-wide px-4 py-2 border-solid border-2 border-emerald-500"
  type="button"><slot /></a
>
