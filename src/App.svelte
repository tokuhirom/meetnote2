<script lang="ts">
  import {onMount} from "svelte";
  import FileItem from "./lib/FileItem.svelte";
  import {DataRepo} from "./lib/data_repo";
  import type {Entry} from "./lib/entry";
  import SummaryView from "./lib/SummaryView.svelte";
  import {invoke} from "@tauri-apps/api/tauri";
  import type {PostProcessStatus} from "./lib/postprocess";
  import {emit, listen} from "@tauri-apps/api/event";
  import type {Event} from "@tauri-apps/api/helpers/event";

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
          if (await entry.hasVTT()) {
            await invoke("start_postprocess", {dir: entry.path, command: "REGENERATE_SUMMARY"});
          } else if (await entry.hasMicWav()) {
            await invoke("start_postprocess", {dir: entry.path, command: "ALL"});
          } else {
            console.error(`There's no .mic.wav or .vtt, retired post processing... ${entry.path}`);
          }
        }
      }
    }, 0);
  });

  setInterval(async () => {
    postProcessingStatus = await invoke("postprocess_status");
    console.log(`postProcessingStatus: ${JSON.stringify(postProcessingStatus)}`);

    if (postProcessingStatus) {
      if (postProcessingStatus.processed_paths.length > 0) {
        for (let entry of entries) {
          if (postProcessingStatus!!.processed_paths.includes(entry.path)) {
            console.log(`post processed: ${entry.path}`);
            await entry.readSummary(); // reload summary
          }
        }

        for (let path of postProcessingStatus.processed_paths) {
          // notice to VTT view, etc.
          await emit("postprocessed_entry", path);
        }
      }
    }
    entries = entries; // notice to svelte.
  }, 1000);

  function onSelectEntry(file: Entry) {
    selectedEntry = file;
  }

  listen("deleted_entry", async (event: Event<String>) => {
    let path = event.payload;
    console.log(`deleted_entry: ${path}`)
    entries = entries.filter((entry) => {
      console.log(`deleted_entry: ${path} != ${entry.path}`)
      return entry.path != path;
    });

    if (entries.length > 0) {
      selectedEntry = entries[0];
    }
  })

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
  let recordingEntry : undefined| Entry = undefined;
  setInterval(async () => {
    if (await isThereTargetWindow()) {
      if (!isRecording) {
        isRecording = true;
        recordingEntry = await data_repo.new_entry();
        await invoke("call_recording_process", {"command": "START", path: recordingEntry.path});
        entries.unshift(recordingEntry);

        entries = entries; // notice to svelte
      }
    } else {
      if (isRecording) {
        recordingEntry = undefined;
        isRecording = false;
        await invoke("call_recording_process", {"command": "STOP", path: null});
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
      {#each entries as entry}
        <FileItem entry={entry} onSelectEntry={onSelectEntry} recordingEntry={recordingEntry}
                  postProcessingStatus={postProcessingStatus} />
      {/each}
    </div>
    <div class="vtt">
      {#if selectedEntry}
        <SummaryView entry="{selectedEntry}" recordingEntry={recordingEntry} />
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
    padding-left: 4px;
    overflow-x: hidden;
    word-break: break-word;
    white-space: normal;
  }
  .vtt {
    flex: 1;
    overflow-y: auto;
    padding-left: 9px;
  }
</style>
