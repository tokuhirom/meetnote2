pub trait Transcriber {
    fn transcribe(&self, in_file: &str, out_file: &str) -> anyhow::Result<()>;
}
