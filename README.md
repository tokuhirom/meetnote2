# MeetNote2
Certainly! Here's the English version of the revised README focusing on the application's user-friendly features:

---

# Zoom Auto-Recording & Transcription App

This application is a powerful tool for automatically recording Zoom meetings and simplifying the documentation and organization of important meeting content. Recorded content is saved as MP3 files, and AI technology is utilized for transcription and summarization. This automation significantly saves time and effort in creating meeting minutes.

## Key Features

- **Automatic Recording**: Automatically starts recording when a Zoom meeting begins.
- **MP3 File Saving**: Recorded audio is saved in the user-friendly MP3 format.
- **AI-Powered Transcription**: Choose between `whisper.cpp` or `OpenAI API` for converting recorded content into text.
- **AI-Powered Summarization**: Offers a summarization feature using either `TF-IDF algorithm` or `OpenAI API`.

## How It Works

- The recording starts automatically with the commencement of a Zoom meeting.
- Once the recording ends, an MP3 file is generated, and the selected AI technology transcribes and summarizes the content.

## Usage Scenarios

- **Meeting Minutes Creation**: Automates detailed record-keeping of meetings with recording and transcription capabilities.
- **Summary Creation**: Useful for busy professionals, providing quick access to the key points of meetings through summarization.

This application aims to enhance meeting efficiency and ease the management of records. In business environments with frequent Zoom meetings, this app proves to be an invaluable tool.

## Usage guide

### Installation

Download the dmg file from github releases(TODO: add link here) 

### Permissions

This application uses ScreenCaptureKit API to detect window names.
It requires accessibility permission. Please allow it.

TODO: add image here.

## Required Environment

 * This application uses Mac OSX APIs.
   * 13.2+ is required.
 * `brew install lame sox make ffmpeg`
   * Use lame for mp3 encoding.
   * Use ffmpeg to convert wave format.
   * Use sox to merge audio files.
 * `xcode` CLI is required to build `whisper.cpp`
   * `xcode-select --install`

(Yes. This is too difficult for non-engineers. I want to bundle these in the future release...)

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
