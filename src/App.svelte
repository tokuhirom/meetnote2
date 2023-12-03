<script lang="ts">
  import Greet from './lib/Greet.svelte'
  import { invoke } from "@tauri-apps/api/tauri"
  import {onMount} from "svelte";

  let files: any[] = []

  onMount(async () => {
    files = await invoke("load_files", { name })
  });
</script>

<main class="container">
  <h1>MeetNote2</h1>

  <div class="files">
    {#each files as { filename, content }, i}
      <div>
        <h2>{filename}</h2>
        <pre>{content}</pre>
      </div>
    {/each}
  </div>

  <div class="row">
    <Greet />
  </div>

</main>

<style>
</style>