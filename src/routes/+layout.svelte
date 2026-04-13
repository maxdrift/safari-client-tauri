<script lang="ts">
  import { browser } from "$app/environment";
  import "../app.css";
  import { initTheme } from "$lib/app.svelte";
  import { onMount } from "svelte";

  if (browser) initTheme();

  onMount(() => {
    const prevent = (e: DragEvent) => {
      e.preventDefault();
      e.stopPropagation();
    };
    window.addEventListener("dragover", prevent);
    window.addEventListener("drop", prevent);
    return () => {
      window.removeEventListener("dragover", prevent);
      window.removeEventListener("drop", prevent);
    };
  });
</script>

<svelte:head>
  <title>Safari Client</title>
</svelte:head>

<slot />
