<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {onMount} from "svelte";

  let files: any[] = []

  onMount(async () => {
    files = await invoke("load_files")
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

</main>

<style>
</style>