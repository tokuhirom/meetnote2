import {fs} from "@tauri-apps/api";
import {removeDir} from "@tauri-apps/api/fs";
import {invoke} from "@tauri-apps/api/tauri";
import {parseWebVTT} from "./webvtt";

export class Entry {
    public path: string; // directory name
    public summary?: string;

    constructor(path: string) {
        this.path = path;
    }

    private async readSummary() {
        let md_path = this.md_path();
        if (await fs.exists(md_path)) {
            this.summary = await fs.readTextFile(this.md_path());
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

    build_path(ext: string): string {
        return `${this.path}/${this.basename()}.${ext}`;
    }

    md_path() {
        return this.build_path("md");
    }

    mp3_path() {
        return this.build_path("mp3");
    }

    vtt_path() {
        return this.build_path("vtt");
    }

    async read_vtt() {
        let vtt  = await fs.readTextFile(this.vtt_path());
        return parseWebVTT(vtt);
    }

    async save_summary(summary: string) {
        let path = this.md_path();
        await fs.writeTextFile(path, summary)
        this.summary = summary;
    }

    async readMp3AsDataUri(): Promise<string> {
        const mp3Path = this.mp3_path();
        try {
            const mp3Data = await fs.readBinaryFile(mp3Path);
            const base64Data = btoa(String.fromCharCode(...new Uint8Array(mp3Data)));
            return `data:audio/mp3;base64,${base64Data}`;
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

    async regenerateSummary() {
        console.log(`Regenerate summary: ${this}`)
        await invoke("regenerate_summary", {path: this.path});
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
