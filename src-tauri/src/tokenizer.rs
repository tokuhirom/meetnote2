pub trait Tokenizer {
    fn tokenize(&self, src: String) -> anyhow::Result<Vec<String>>;
}
