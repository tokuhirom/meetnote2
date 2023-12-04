<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {onMount} from "svelte";

  let files: any[] = []
  let inputDevices: string[] = []

  onMount(async () => {
    files = await invoke("load_files", { name })
    inputDevices = await invoke("get_input_devices");
  });
</script>

<main class="container">
  <h1>MeetNote2</h1>

  <div>
    <ul>
    {#each inputDevices as device}
      <li>{device}</li>
    {/each}
    </ul>
  </div>

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