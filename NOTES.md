# NOTES

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
- [ ] 自動的に wav からの自動処理をバックグラウンドスレッドで処理する機能を廃止にしようかなー
- [ ] 後処理やりなおすくん、JS側に実装しなおす




