<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {onMount} from "svelte";
  import FileItem from "./lib/FileItem.svelte";

  let files: any[] = []

  onMount(async () => {
    files = await invoke("load_files")
  });

  // todo: better polling logic
  setInterval(async () => {
    files = await invoke("load_files");
  }, 3000);
</script>

<main class="container">
  <h1>MeetNote2</h1>

  <div class="files">
    {#each files as file}
      <FileItem file={file}/>
    {/each}
  </div>

</main>

<style>
</style>