pub trait Summarizer {
    /**
     * Summarize the webvtt. Input is the webvtt in string.
     * returns the summarized text in markdown format.
     */
    fn summarize(&self, webvtt: &str) -> anyhow::Result<String>;
}
