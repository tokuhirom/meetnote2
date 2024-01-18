use std::path::PathBuf;

pub struct Entry {
    pub dir: PathBuf,
    pub basename: String,
}

impl Entry {
    pub fn new(dir: PathBuf) -> Self {
        let basename = dir.as_path().file_name().unwrap().to_str().unwrap().to_string();
        Entry { dir, basename }
    }

    pub fn mic_wav_path(&self) -> PathBuf {
        self.path("mic.wav")
    }

    pub fn webvtt_path(&self) -> PathBuf {
        self.path("vtt")
    }

    pub fn mp3_path(&self) -> PathBuf {
        self.path("mp3")
    }

    pub fn md_path(&self) -> PathBuf {
        self.path("md")
    }

    fn path(&self, ext: &str) -> PathBuf {
        let filename = format!("{}.{}", self.basename, ext);
        self.dir.join(filename)
    }
}
