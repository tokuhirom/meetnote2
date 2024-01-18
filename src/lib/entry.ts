import {fs} from "@tauri-apps/api";
import {exists, removeDir} from "@tauri-apps/api/fs";
import {invoke} from "@tauri-apps/api/tauri";
import {parseWebVTT} from "./webvtt";

export class Entry {
    public path: string; // directory name
    public summary?: string;

    constructor(path: string) {
        this.path = path;
    }

    async readSummary() {
        let md_path = this.mdPath();
        if (await fs.exists(md_path)) {
            this.summary = await fs.readTextFile(this.mdPath());
        }
    }

    basename() : string {
        const pattern = /[^/\\]+$/;
        const matches = this.path.match(pattern);
        if (!matches) {
            throw new Error(`Cannot extract basename from ${this.path}`);
        }
        return matches[0];
    }

    buildPath(ext: string): string {
        return `${this.path}/${this.basename()}.${ext}`;
    }

    mdPath() {
        return this.buildPath("md");
    }

    mp3Path() {
        return this.buildPath("mp3");
    }

    vttPath() {
        return this.buildPath("vtt");
    }

    hasMp3(): Promise<boolean> {
        return exists(this.mp3Path());
    }

    hasVTT(): Promise<boolean> {
        return exists(this.vttPath());
    }

    hasMD(): Promise<boolean> {
        return exists(this.mdPath());
    }

    async readVTT() {
        let vtt  = await fs.readTextFile(this.vttPath());
        return parseWebVTT(vtt);
    }

    async saveSummary(summary: string) {
        let path = this.mdPath();
        await fs.writeTextFile(path, summary)
        this.summary = summary;
    }

    async readMp3AsDataUri(): Promise<string> {
        const mp3Path = this.mp3Path();
        try {
            const mp3Data = await fs.readBinaryFile(mp3Path);
            const blob = new Blob([mp3Data], {
                type: "audio/mp3",
            });
            return new Promise(function (res, rej) {
                const reader = new FileReader();
                reader.onload = function (evt) {
                    const dataurl : string = evt.target!!.result as string;
                    res(dataurl);
                };
                reader.onerror = function (err) {
                    rej(err);
                };
                reader.readAsDataURL(blob);
            });
        } catch (error) {
            console.error(`Failed to read mp3 file at ${mp3Path}:`, error);
            throw new Error(`Could not read mp3 file: ${mp3Path}: ${error}`);
        }
    }


    title(): string {
        let datetimeStr = this.basename();
        const match = datetimeStr.match(/^(\d{4})(\d{2})(\d{2})(\d{2})(\d{2})(\d{2})$/);
        if (!match) {
            throw new Error('Invalid datetime format');
        }

        const [ , year, month, day, hour, minute ] = match.map(Number);

        const date = new Date(year, month - 1, day, hour, minute);

        const weekdays = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

        return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}(${weekdays[date.getDay()]}) ${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`;
    }

    async regenerateSummary(): Promise<string> {
        console.log(`Regenerate summary: ${this.path}`)
        return await invoke("regenerate_summary", {
            vttPath: this.vttPath(),
            mdPath: this.mdPath(),
        });
    }

    async remove() {
        await removeDir(this.path, {
            recursive: true
        })
    }

    static async fromPath(path: string): Promise<Entry> {
        let entry = new Entry(path);
        try {
            await entry.readSummary();
        } catch (e) {
            // This is not a fatal error... maybe.
            console.error(e);
        }
        return entry;
    }
}
