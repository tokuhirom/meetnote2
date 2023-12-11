use std::collections::{HashMap, HashSet};
use crate::lindera_tokenizer::LinderaTokenizer;
use crate::summarizer::Summarizer;
use crate::tokenizer::Tokenizer;
use crate::webvtt::{Caption, parse_webvtt};

/**
 * TF-IDF を用いて、対象の時間帯において、特徴的な文を抽出する。
 */
pub struct TFIDFSummarizer {
    tokenizer: LinderaTokenizer,
}

impl TFIDFSummarizer {
    pub fn new() -> anyhow::Result<TFIDFSummarizer> {
        let tokenizer = LinderaTokenizer::new()?;

        Ok(TFIDFSummarizer { tokenizer })
    }
}

struct Row {
    start_time: u32,
    tokens: Vec<String>,
    caption: Caption,
}

impl Summarizer for TFIDFSummarizer {
    fn summarize(&self, webvtt: &str) -> anyhow::Result<String> {
        let vec: Vec<Caption> = parse_webvtt(webvtt);
        let rows: Vec<Row> = vec.iter().filter(
            |row| {
                // black list
                // "ご視聴ありがとうございました" をやたらと whisper.cpp は生成する。
                // "(ボタンを押す音)" はタイピング音だけのときに文字起こしされる
                row.text != "ご視聴ありがとうございました"
                && row.text != "(ボタンを押す音)"
            }
        ).map(|row| {
            let start_time = row.parse_start_time();
            let tokens = self.tokenizer.tokenize(row.text.clone()).unwrap().to_vec();
            Row { start_time, tokens, caption: (*row).clone() }
        }).collect();

        // Get total documents and all terms
        let total_documents = rows.len() as f64;
        let mut all_terms: HashSet<String> = HashSet::new();
        for row in &rows {
            for token in &row.tokens {
                all_terms.insert(token.clone());
            }
        }

        let mut tfidf: HashMap<String, f64> = HashMap::new();
        for term in all_terms {
            let mut document_frequency = 0f64;
            let mut term_frequency = 0f64;

            for row in &rows {
                if row.tokens.contains(&term) {
                    document_frequency += 1.0;
                    term_frequency += row.tokens.iter().filter(|t| *t == &term).count() as f64;
                }
            }

            let tf = term_frequency / total_documents; // term frequency
            let idf = (total_documents / document_frequency).ln(); // inverse document frequency
            tfidf.insert(term, tf * idf);
        }

        let mut sectioned_captions: HashMap<u32, Row> = HashMap::new();
        for row in rows {
            let bucket = row.start_time / (3 * 60 * 1000);
            if sectioned_captions.contains_key(&bucket) {
                let current = sectioned_captions.get(&bucket).unwrap();
                let current_score:f64 = current.tokens.iter().map(|token| tfidf.get(token).unwrap()).sum();
                let new_score = row.tokens.iter().map(|token| tfidf.get(token).unwrap()).sum();
                if current_score < new_score {
                    sectioned_captions.insert(bucket, row);
                }
            } else {
                sectioned_captions.insert(bucket, row);
            }
        }

        let mut keys: Vec<&u32> = sectioned_captions.keys().collect();
        keys.sort();

        Ok(keys.iter().map(|f| {
            let row = sectioned_captions.get(*f).unwrap();
            format!("[{}] {}", row.caption.start_time, row.caption.text)
        }).collect::<Vec<_>>().join("\n"))
    }
}


#[cfg(test)]
mod tests {
    use crate::summarizer::Summarizer;
    use crate::tf_idf_summarizer::TFIDFSummarizer;

    #[test]
    fn test_summarize() {
        let summarizer = TFIDFSummarizer::new().unwrap();
        let webvtt = r#"
WEBVTT

00:00:00.000 --> 00:00:06.000
こんにちは〜。

00:00:07.000 --> 00:00:10.000
こんにちは。

00:00:11.000 --> 00:00:16.000
最近はどういうプログラミング言語が熱いですか？

00:00:20.000 --> 00:00:25.000
やっぱり rust ですかねえ。。

00:00:26.000 --> 00:00:29.000
rust ってなにがいいの？

00:00:26.000 --> 00:00:29.000
やっぱり、高速なことかなぁ。メモリをちゃんと管理できるのもいいね。

00:00:30.000 --> 00:00:32.000
あー

00:00:33.000 --> 00:00:39.000
速度がやっぱりいいのと、tauri 使えるのもいいかも。

00:03:03.000 --> 00:03:39.000
ところで、最近焼肉食べに行っている？

00:03:40.000 --> 00:03:49.000
あー

00:04:00.000 --> 00:04:19.000
焼肉だと何が好き？

00:04:20.000 --> 00:04:39.000
カルビっすかねぇ

00:04:40.000 --> 00:04:55.000
カルビはうまいよな

00:05:10.000 --> 00:05:25.000
タン塩も好きですがねぇ

"#;
        let result = summarizer.summarize(webvtt).unwrap();
        assert_eq!(
            result,
            "[00:00:33.000] 速度がやっぱりいいのと、tauri 使えるのもいいかも。\n[00:03:03.000] ところで、最近焼肉食べに行っている？"
        );
    }

    #[test]
    fn test_summarize_en() {
        let summarizer = TFIDFSummarizer::new().unwrap();
        let webvtt = r#"
WEBVTT

00:00:00.000 --> 00:00:06.000
Hello~

00:00:07.000 --> 00:00:10.000
Hello.

00:00:11.000 --> 00:00:16.000
What programming languages are hot recently?

00:00:20.000 --> 00:00:25.000
I guess it would be Rust, right?

00:00:26.000 --> 00:00:29.000
What's so good about Rust?

00:00:26.000 --> 00:00:29.000
Well, its speed, of course. And it can handle memory efficiently.

00:00:30.000 --> 00:00:32.000
Ah.

00:00:33.000 --> 00:00:39.000
The speed is definitely good, and it's also nice that it supports Tauri.

00:03:03.000 --> 00:03:09.000
By the way, have you been going out for Yakiniku (Japanese BBQ) lately?

00:03:10.000 --> 00:03:15.000
Ah.

00:04:00.000 --> 00:04:06.000
What do you like when it comes to Yakiniku?

00:04:07.000 --> 00:04:14.000
I guess it's Kalbi, right？

00:04:15.000 --> 00:04:22.000
Kalbi is indeed delicious.

00:05:10.000 --> 00:05:16.000
I also like Tan-shio (salted tongue).
"#;
        let result = summarizer.summarize(webvtt).unwrap();
        assert_eq!(
            result,
            "[00:00:33.000] The speed is definitely good, and it's also nice that it supports Tauri.\n[00:03:03.000] By the way, have you been going out for Yakiniku (Japanese BBQ) lately?"
        );
    }
}
