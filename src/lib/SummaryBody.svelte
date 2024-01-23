<script lang="ts">
    import {Entry} from "./entry.js";
    import StatusIndicator from "./StatusIndicator.svelte";
    import {dialog} from "@tauri-apps/api";
    import {listen} from "@tauri-apps/api/event";
    import {invoke} from "@tauri-apps/api/tauri";
    import type {PostProcessStatus} from "./postprocess";
    import {onMount} from "svelte";
    import {EditorState, Transaction} from "@codemirror/state";
    import {defaultKeymap} from "@codemirror/commands";
    import {EditorView, keymap} from "@codemirror/view";
    import {markdown} from "@codemirror/lang-markdown"
    import {solarizedDark} from "cm6-theme-solarized-dark";

    export let entry: Entry;
    export let recordingEntry: Entry | undefined;
    export let postProcessingStatus: PostProcessStatus | undefined;

    let view: EditorView;

    $: if (entry) {
        if (view) {
            let state = view.state;
            let transaction = state.update({
                changes: {from: 0, to: state.doc.length, insert: entry.summary},
                annotations: Transaction.userEvent.of("program")
            });
            view.dispatch(transaction);
        }
    }

    onMount(() => {
        let container = document.getElementById("goodeditor")

        let startState = EditorState.create({
            doc: entry.summary,
            extensions: [
                keymap.of(defaultKeymap),
                markdown(),
                solarizedDark,
                EditorView.updateListener.of(update => {
                    if (update.changes) {
                        let isUserInput = update.transactions.some(tr => tr.annotation(Transaction.userEvent) !== "program");
                        if (isUserInput) {
                            console.log(`テキストが変更されました ${isUserInput}`);
                            saveItem();
                        }
                    }
                })
            ]
        })

        view = new EditorView({
            state: startState,
            parent: container
        })
    })

    listen("do_regenerate_summary", async () => {
        console.log("Received regenerate_summary");
        await regenerateSummaryItem();
    })

    async function regenerateSummaryItem() {
        try {
            await invoke("start_postprocess", {dir: entry.path, command: "REGENERATE_SUMMARY"});
            entry = entry;
        } catch (e) {
            console.log(e);
            await dialog.message(`${e}`);
        }
    }

    async function saveItem() {
        console.log(`Save file: ${JSON.stringify(entry)}`)
        let state = view.state;
        let doc = state.doc;
        let text = doc.toString();
        await entry.saveSummary(text);
    }
</script>

<div>
    {#if entry && recordingEntry && recordingEntry.path === entry.path}
        <div class="now-recording">Now recording this entry...</div>
    {:else if !entry.summary}
        <div class="summary-wip">Summary not available... yet.
            {#if entry && postProcessingStatus && postProcessingStatus.path === entry.path}
                <div class="now-postprocessing">{postProcessingStatus.message}</div>
            {/if}
            <StatusIndicator entry={entry} />
        </div>
    {/if}

    <div id="goodeditor"></div>

    <div class="path">{entry.path}</div>
</div>

<style>
    .path {
        color: darkslategray;
        font-size: 80%;
        background: none;
    }

    .summary-wip {
        margin: 8px;
        color: yellowgreen;
    }

    .now-recording {
        color: red;
    }
    .now-postprocessing {
        color: #396cd8;
    }
</style>
