<script lang="ts">
  export let disabled = false;
  export let hidden = false;
</script>

<button hidden={hidden} disabled={disabled} class="touch-none tracking-wide px-4 py-2 border-solid border-2 border-emerald-500" on:click type="button"><slot/></button>