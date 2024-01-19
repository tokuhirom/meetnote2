<script lang="ts">
  import {onMount} from "svelte";
  import FileItem from "./lib/FileItem.svelte";
  import NowRecordingIndicator from "./lib/NowRecordingIndicator.svelte";
  import VttView from "./VttView.svelte";
  import {DataRepo} from "./lib/data_repo";
  import type {Entry} from "./lib/entry";
  import SummaryView from "./lib/SummaryView.svelte";
  import {invoke} from "@tauri-apps/api/tauri";
  import PostProcessingIndicator from "./lib/PostProcessingIndicator.svelte";
  import type {PostProcessStatus} from "./lib/postprocess";

  let selectedEntry: Entry | undefined = undefined;
  let entries: Entry[] = []

  let data_repo = new DataRepo();

  let postProcessingStatus: PostProcessStatus | undefined = undefined;

  onMount(async () => {
    entries = await data_repo.list_entries();

    if (entries.length > 0) {
      selectedEntry = entries[0];
    }

    setTimeout(async () => {
      for (let entry of entries) {
        if (!await entry.hasMD()) {
          console.log(`Running postprocess... ${entry.path}`);
          await invoke("start_postprocess", {dir: entry.path});
        }
      }
    }, 0);
  });

  // todo: better polling logic
  setInterval(async () => {
    entries = await data_repo.list_entries();
  }, 3000);

  setInterval(async () => {
    postProcessingStatus = await invoke("postprocess_status");
  }, 1000);

  function onSelectEntry(file: Entry) {
    selectedEntry = file;
  }

  async function onDelete() {
    entries = await data_repo.list_entries();

    if (entries.length > 0) {
      selectedEntry = entries[0];
    }
  }

  interface WindowPattern {
    bundle_id: string,
    window_title: string,
  }
  interface Configuration {
    window_patterns: WindowPattern[]
  }
  interface WindowInfo {
    bundle_id: string,
    window_title: string,
  }

  let isRecording = false;
  setInterval(async () => {
    if (await isThereTargetWindow()) {
      if (!isRecording) {
        isRecording = true;
        await invoke("call_recording_process", {"command": "START"});
      }
    } else {
      if (isRecording) {
        isRecording = false;
        await invoke("call_recording_process", {"command": "STOP"});
      }
    }
  }, 1000);

  async function isThereTargetWindow() {
    let config = await invoke("load_config") as Configuration; // TODO cache
    let windows = await invoke("get_windows") as WindowInfo[];

    for (let windowPattern of config.window_patterns) {
      for (let window of windows) {
        if (window.window_title == windowPattern.window_title && window.bundle_id == windowPattern.bundle_id) {
          return true;
        }
      }
    }
    return false;
  }
</script>

<main class="container">
  <div class="main-container">
    <div class="files">
      <NowRecordingIndicator isRecording={isRecording} />

      {#if postProcessingStatus && postProcessingStatus.message !== ""}
      <PostProcessingIndicator status={postProcessingStatus} />
      {/if}

      {#each entries as entry}
        <FileItem entry={entry} onSelectEntry={onSelectEntry}/>
      {/each}
    </div>
    <div class="vtt">
      {#if selectedEntry}
        <h2>{selectedEntry.title()}</h2>
        <SummaryView entry="{selectedEntry}" onDelete={onDelete} />
        <hr class="separator" />
        <VttView entry={selectedEntry} />
      {/if}
    </div>
  </div>
</main>

<style>
  .main-container {
    display: flex;
    flex-direction: row;
    height: 100vh;
  }
  .files {
    flex: 0 0 30%;
    overflow-y: auto;
    padding-right: 9px;
    overflow-x: hidden;
    word-break: break-word;
    white-space: normal;
  }
  .vtt {
    flex: 1;
    overflow-y: auto;
    padding-left: 9px;
  }

  .separator {
    margin-bottom: 10px;
    /*border-bottom:#396cd8 1px solid;*/
  }
</style>
