# MeetNote2

This is a successor of the [MeetNote](https://github.com/tokuhirom/meetnote).
Fully rewritten by Rust.

This application is built top of `Tauri + Svelte + Typescript`.

## Required Environment

 * This application uses Mac OSX APIs.
 * Use lame for mp3 encoding.
   * Please install it via `brew install lame`
 * `brew install make `
 * `brew install ffmpeg` ffmpeg で、音声ファイルのサンプリングレートを変換しています。
 * `xcode` 的なサムシングが必要です
   * `xcode-select --install`

## Permissions

This application uses ScreenCaptureKit API to detect window names.
Please allow it.

## Features

 * Detect window names to start the recording.
 * Record audio to wave file.
   * Convert it to mp3 file automatically.
   * Transcribe it to the text using whisper.cpp.
   * Summarize it with OpenAI's API
 * Edit summary file

## Future plan

 * Show VTT file
 * Play mp3 file(or, use webm or something other format)
 * use ScreenCaptureKit API
 * drop external dependencies
   * ffmpeg, etc.
 * TODO: Summarize it in the local machine.
   * but so, chatgpt can generate excellent summary...
   * I can't generate the same level summary with OSS and or local LLM.

## How do I build this?

    bun install
    bun run tauri dev

## LICENSE

    The MIT License (MIT)

    Copyright © 2023 Tokuhiro Matsuno, http://64p.org/ tokuhirom@gmail.com

    Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

### Scap

Some code are taken from [scap](https://github.com/clearlysid/scap).

    MIT License

    Copyright (c) 2022 Siddharth

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

https://github.com/clearlysid/scap/blob/main/LICENSE
