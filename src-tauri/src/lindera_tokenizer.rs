use lindera_analyzer::analyzer::Analyzer;
use serde_json::json;

struct LinderaTokenizer {
    analyzer: Analyzer,
}

impl LinderaTokenizer {
    fn new() -> anyhow::Result<LinderaTokenizer> {
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
      "kind": "japanese_compound_word",
      "args": {
        "kind": "ipadic",
        "tags": [
          "名詞,数"
        ],
        "new_tag": "名詞,数"
      }
    },
    {
      "kind": "japanese_number",
      "args": {
        "tags": [
          "名詞,数"
        ]
      }
    },
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
          "非言語音"
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
        let vec = self.analyzer.analyze(src.to_string().as_str()).expect("TODO: panic message");
        Ok(vec.iter().map(|y| {y.text.clone()}).collect())
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

        assert_eq!(vec, vec!["私", "名前", "中野"]);
    }
}
