<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {onMount} from "svelte";
  import {getCurrent} from "@tauri-apps/api/window";

  let config = {
    openai_api_token: undefined,
    target_device: undefined
  };
  let devices:  string[] = [];

  onMount(async () => {
    config = await invoke("load_config");
    devices = await invoke("get_input_devices");
  });

  async function saveConfig() {
    await invoke("save_config", {config: config})
    const window = getCurrent();
    await window.close();
  }
</script>

<main class="container">
  <h1>Configuration - MeetNote2</h1>

  <form on:submit|preventDefault={saveConfig}>
    <div>
      OpenAI API Key:
      <input type="text" bind:value={config.openai_api_token}>
    </div>
    <div>
      Target device:
      <select bind:value={config.target_device}>
        {#each devices as device}
          <option value={device}>{device}</option>
        {/each}
      </select>
    </div>
    <button type="submit">Save</button>
  </form>

</main>

<style>
</style>