import {BaseDirectory, createDir, exists, readDir} from "@tauri-apps/api/fs";
import {Entry} from "./entry";
import {invoke} from "@tauri-apps/api/tauri";

export class DataRepo {
    constructor() {
    }

    async new_entry() : Promise<Entry> {
        let path : string = await invoke('new_entry_path');
        return new Entry(path)
    }

    async list_entries(): Promise<Entry[]> {
        // mkdir -p
        if (!await exists('data', { dir: BaseDirectory.AppData })) {
            await createDir('data', {dir: BaseDirectory.AppData, recursive: true });
        }

        const result: Entry[] = [];
        const entries = await readDir('data', { dir: BaseDirectory.AppData, recursive: false });
        for (const fileEntry of entries) {
            if (fileEntry.path.endsWith(".DS_Store")) { // Ignore .DS_Store
                continue;
            }

            if (await exists(fileEntry.path)) {
                let entry = await Entry.fromPath(fileEntry.path);
                result.push(entry);
            }
        }
        result.sort((a, b) => b.path.localeCompare(a.path));
        return result;
    }
}
