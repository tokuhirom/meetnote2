<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {onMount} from "svelte";
  import {getCurrent} from "@tauri-apps/api/window";

  let config : {
    openai_api_token: string | undefined,
    target_device: string | undefined,
    window_patterns: {bundle_id: string, window_title: string}[]
  } = {
    openai_api_token: undefined,
    target_device: undefined,
    window_patterns: []
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
    <div class="pane">
      <h3>Basic configuration</h3>
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
    </div>
    <div class="pane">
      <h3>Window patterns</h3>
      <table>
        <tr>
          <th>Bundle ID</th>
          <th>Window title</th>
        </tr>
      {#each config.window_patterns as pattern}
        <tr>
          <td>{pattern.bundle_id}</td>
          <td>{pattern.window_title}</td>
        </tr>
      {/each}
      </table>
    </div>
    <button type="submit">Save</button>
  </form>

</main>

<style>
  .pane {
    margin-bottom: 40px;
  }
  td, th {
    border-right: 1px solid white;
    padding: 4px;
  }
</style>
