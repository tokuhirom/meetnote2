import {BaseDirectory, createDir, exists, readDir} from "@tauri-apps/api/fs";
import {Entry} from "./entry";

export class DataRepo {
    constructor() {
    }

    async list_entries(): Promise<Entry[]> {
        console.log("list_entries");
        // mkdir -p
        if (!await exists('data', { dir: BaseDirectory.AppData })) {
            await createDir('data', {dir: BaseDirectory.AppData, recursive: true });
        }

        const result: Entry[] = [];
        const entries = await readDir('data', { dir: BaseDirectory.AppData, recursive: false });
        for (const fileEntry of entries) {
            console.log(`Entry: ${fileEntry.path}`);
            if (await exists(fileEntry.path)) {
                let entry = await Entry.fromPath(fileEntry.path);
                result.push(entry);
            }
        }
        result.sort((a, b) => b.path.localeCompare(a.path));
        return result;
    }
}
