use std::path::PathBuf;
use anyhow::anyhow;
use glob::Paths;

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

    pub fn mic_wav_path_string(&self) -> String {
        let path = self.mic_wav_path();
        return path.to_str().unwrap().to_string();
    }

    pub fn merged_wav_path_string(&self) -> String {
        let path = self.path("wav");
        return path.to_str().unwrap().to_string();
    }

    pub fn list_raw_files(&self) -> anyhow::Result<Paths> {
        let paths = glob::glob(self.dir.join(format!("{}*.raw", self.basename)).to_str().unwrap())
            .map_err(|err| anyhow!("pattern error: {:?}", err))?;
        Ok(paths)
    }

    pub fn raw_prefix_path_string(&self) -> String {
        return self.dir.join(&self.basename).to_str().unwrap().to_string();
    }

    pub fn webvtt_path_string(&self) -> String {
        self.path("vtt").to_str().unwrap().to_string()
    }

    pub fn mp3_path_string(&self) -> String {
        self.path("mp3").to_str().unwrap().to_string()
    }

    pub fn md_path(&self) -> String {
        self.path("md").to_str().unwrap().to_string()
    }

    fn path(&self, ext: &str) -> PathBuf {
        let filename = format!("{}.{}", self.basename, ext);
        self.dir.join(filename)
    }
}
