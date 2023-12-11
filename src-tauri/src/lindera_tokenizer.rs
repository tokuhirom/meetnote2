use lindera_analyzer::analyzer::Analyzer;

use serde_json::json;

pub  struct LinderaTokenizer {
    analyzer: Analyzer,
}

impl LinderaTokenizer {
    pub(crate) fn new() -> anyhow::Result<LinderaTokenizer> {
        let analyzer = Analyzer::from_value(&json!({
  "character_filters": [
    {
      "kind": "unicode_normalize",
      "args": {
        "kind": "nfkc"
      }
    },
    {
      "kind": "japanese_iteration_mark",
      "args": {
        "normalize_kanji": true,
        "normalize_kana": true
      }
    }
  ],
  "tokenizer": {
    "dictionary": {
      "kind": "ipadic"
    },
    "mode": "normal"
  },
  "token_filters": [
    {
      "kind": "japanese_stop_tags",
      "args": {
        "tags": [
          "接続詞",
          "助詞",
          "助詞,格助詞",
          "助詞,格助詞,一般",
          "助詞,格助詞,引用",
          "助詞,格助詞,連語",
          "助詞,係助詞",
          "助詞,副助詞",
          "助詞,間投助詞",
          "助詞,並立助詞",
          "助詞,終助詞",
          "助詞,副助詞／並立助詞／終助詞",
          "助詞,連体化",
          "助詞,副詞化",
          "助詞,特殊",
          "助動詞",
          "記号",
          "記号,一般",
          "記号,読点",
          "記号,句点",
          "記号,空白",
          "記号,括弧閉",
          "その他,間投",
          "フィラー",
          "非言語音",
          "名詞,非自立,一般", // "の" とか。
          "副詞,助詞類接続",
          "助詞,接続助詞", // て
          "動詞,非自立", // ください
          "感動詞", // "はい"
          "接頭詞,名詞接続", //
            "副詞,一般", // "どうか"
            "動詞,自立", // "し"
            "名詞,代名詞,一般", // "あれ"
        ]
      }
    },
    {
      "kind": "japanese_katakana_stem",
      "args": {
        "min": 3
      }
    }
  ]
        })).unwrap();

        Ok(LinderaTokenizer { analyzer })
    }
}

impl crate::tokenizer::Tokenizer for LinderaTokenizer {
    fn tokenize(&self, src: String) -> anyhow::Result<Vec<String>> {
        let vec = self.analyzer.analyze(src.to_string().as_str())
            .expect("TODO: panic message");
        let vec : Vec<String> = vec.iter().filter(|t| {
            t.text.chars().count() != 1
        }).map(|y| {
            y.text.clone()
        }).collect();
        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    use crate::lindera_tokenizer::LinderaTokenizer;
    use crate::tokenizer::Tokenizer;

    #[test]
    fn it_works() {
        let tokenizer = LinderaTokenizer::new().unwrap();
        let vec = tokenizer.tokenize("私の名前は中野です。".to_string()).unwrap();

        assert_eq!(vec, vec!["名前", "中野"]);
    }

    #[test]
    fn it_works_well() {
        let tokenizer = LinderaTokenizer::new().unwrap();
        let vec = tokenizer.tokenize("そう!Chromeですね。".to_string()).unwrap();

        assert_eq!(vec, vec!["Chrome"]);
    }

    #[test]
    fn test2() {
        let tokenizer = LinderaTokenizer::new().unwrap();
        let vec = tokenizer.tokenize("頑張ってください".to_string()).unwrap();

        assert_eq!(vec, Vec::<String>::new());
    }

    #[test]
    fn test3() {
        let tokenizer = LinderaTokenizer::new().unwrap();
        let vec = tokenizer.tokenize("はいはいはい".to_string()).unwrap();

        assert_eq!(vec, Vec::<String>::new());
    }

    #[test]
    fn test4() {
        let tokenizer = LinderaTokenizer::new().unwrap();
        let vec = tokenizer.tokenize("どうかしないかな".to_string()).unwrap();

        assert_eq!(vec, Vec::<String>::new());
    }

    #[test]
    fn test5() {
        let tokenizer = LinderaTokenizer::new().unwrap();
        let vec = tokenizer.tokenize("おーあれは".to_string()).unwrap();

        assert_eq!(vec, Vec::<String>::new());
    }
}
