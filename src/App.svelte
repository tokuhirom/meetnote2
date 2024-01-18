<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {onMount} from "svelte";
  import FileItem from "./lib/FileItem.svelte";
  import NowRecordingIndicator from "./lib/NowRecordingIndicator.svelte";
  import VttView from "./VttView.svelte";

  let selectedFile: {filename: string, content: string|null} | undefined = undefined;
  let files: {filename: string, content: string}[] = []

  onMount(async () => {
    files = await invoke("load_files")
  });

  // todo: better polling logic
  setInterval(async () => {
    files = await invoke("load_files");
  }, 3000);

  function onSelectFile(file: {filename: string, content: string|null}) {
    selectedFile = file;
  }
</script>

<main class="container">
  <NowRecordingIndicator />

  <div class="main-container">
    <div class="files">
      {#each files as file}
        <FileItem file={file} onSelectFile={onSelectFile}/>
      {/each}
    </div>
    <div class="vtt">
      {#if selectedFile}
        <VttView file={selectedFile} />
      {/if}
    </div>
  </div>
</main>

<style>
  .main-container {
    display: flex;
  }
  .files {
    flex: 0 0 30%;
    overflow-y: auto;
  }
  .vtt {
    flex: 1;
  }
</style>