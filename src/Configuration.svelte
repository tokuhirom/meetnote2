<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {onMount} from "svelte";
  import {getCurrent} from "@tauri-apps/api/window";

  type WindowPattern = {
    bundle_id: string;
    window_title: string;
  };
  type WindowInfo = {
    bundle_id: string;
    window_title: string;
    width: number;
    height: number;
    is_on_screen: boolean,
  };


  let showWindowList = false;
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
  let windows: WindowInfo[] = [];

  onMount(async () => {
    config = await invoke("load_config");
    devices = await invoke("get_input_devices");
    windows = await invoke("get_windows");
  });

  async function saveConfig() {
    await invoke("save_config", {config: config})
    const window = getCurrent();
    await window.close();
  }

  function addItem(window: WindowInfo) {
    // Check if the window is already in the array
    if (!config.window_patterns.some(w => w.bundle_id === window.bundle_id && w.window_title === window.window_title)) {
      console.log("Pushing window: ", window);
      config.window_patterns = [...config.window_patterns, window];
    } else {
      console.log("Window is already in the array.");
    }
  }

  function deleteItem(window: WindowPattern) {
    config.window_patterns = config.window_patterns.filter(
            item => item.bundle_id !== window.bundle_id || item.window_title !== window.window_title
    );
  }

  async function reloadWindows() {
    windows = await invoke("get_windows");
  }

  function toggleWindowList() {
    showWindowList = !showWindowList;
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

      <button on:click|preventDefault={toggleWindowList}>Add new window pattern</button>

      {#if showWindowList}
        <div class="window_list">
          <div class="header">
            <h4>Window List</h4>
            <button on:click|preventDefault={reloadWindows}>Reload</button>
          </div>
          <table>
            <tr>
              <th>Bundle ID</th>
              <th>Window Title</th>
            </tr>
            {#each windows as window}
              <tr>
                <td>{window.bundle_id}</td>
                <td>{window.window_title}</td>
                <td>{window.width}</td>
                <td>{window.height}</td>
                <td>{window.is_on_screen}</td>
                <td><button on:click|preventDefault={() => addItem(window)}>Add</button></td>
              </tr>
            {/each}
          </table>
        </div>
      {/if}

      <table>
        <tr>
          <th>Bundle ID</th>
          <th>Window title</th>
        </tr>
      {#each config.window_patterns as pattern}
        <tr>
          <td>{pattern.bundle_id}</td>
          <td>{pattern.window_title}</td>
          <td><button on:click|preventDefault={() => deleteItem(pattern)}>Delete</button></td>
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

  .window_list .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .window_list {
    padding: 4px;
    margin: 8px;
    border: 1px solid white;
    background-color: darkslategrey;
  }
  .window_list button {
    padding: 2px;
  }
</style>
