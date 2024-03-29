# NOTES

This is a successor of the [MeetNote](https://github.com/tokuhirom/meetnote).
Fully rewritten by Rust.

This application is built top of `Tauri + Svelte + Typescript`.

How do I build this?

    bun install
    bun run tauri dev

- サンプルがある
- ディレクトリ構造変えたい
- `~/Library/Application Support/meetnote2/%Y%m%d/%Y%m%d%H%M%S/` にファイルを配置する
  - これを entry と呼称する
  - まずは書く部分を直していくか。
  - data_repo::new_mic_wave_file_name がそうだ。
    - ここはなおした
    - が、そもそもここの方針がおかしい
    - entry::new() を実装して、そこに mic_wave_path() メソッドを生やすべき。
    - いや、data_repo::new_entry() が必要だ
- 可能な限り、JS 側にメインのロジックを移していく。そのほうが管理しやすい。
- 別ファイルを開いたときに、audio tag をクリアする必要がありそう。
- メニューバーを用意したほうがいいかもしれない。
- [x] audio 録音状態の管理を js 側に持ってくる
- [x] 後処理やりなおすくん、JS側に実装しなおす
  - これはやったほうがいいな。
  - でもこれ、実際のところ、token 長すぎるやつが結構違う気はする
- [x] 長すぎる場合は summarize を分離する。
  - tiktoken?
- postprocess の状態を UI で表現したい
  - そうするなら、postprocess 用の worker thread が必要か?
  - postprocess まわり、Entry object も保持したほうが良いかもしれない。
- [x] Regenerate summary も遅すぎてUIがスタックする
- だいぶ見た目が整理されてきた。
- [x] 編集がそのままできるようにしたい。
  - CodeMirror とか?
  - milkdown?
- [ ] 録音時点からミックスすることに挑戦してもいいかも。。ただし、別にそれで使い勝手が向上するわけではない。
- [x] Zoom を全画面にしたときに zoom ディテクションが動いてない
  - ウィンドウタイトルが変わってしまうため
  - `kAudioDevicePropertyDeviceIsRunningSomewhere` を確認する必要あり。
  - active じゃない window になってただけだった
- [x] 設定画面で、summarizer, transcriber を選べるようにしたい
- [x] use dirs::config_dir

後処理
- convert to MP3
- convert to VTT
- Summarize VTT
- cleanup files
