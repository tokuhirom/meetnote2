<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {onMount} from "svelte";
  import FileItem from "./lib/FileItem.svelte";
  import NowRecordingIndicator from "./lib/NowRecordingIndicator.svelte";

  let files: {filename: string, content: string}[] = []

  onMount(async () => {
    files = await invoke("load_files")
  });

  // todo: better polling logic
  setInterval(async () => {
    files = await invoke("load_files");
  }, 3000);
</script>

<main class="container">
  <NowRecordingIndicator />

  <div class="files">
    {#each files as file}
      <FileItem file={file}/>
    {/each}
  </div>

</main>

<style>
</style>