use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::anyhow;
use std::time::Instant;
use uuid::Uuid;
use crate::transcriber::Transcriber;

pub struct WhisperTranscriber {
    version: String,
    model: String,
    language: String,
}

impl WhisperTranscriber {
    pub fn new(version: String, model: String, language: String) -> WhisperTranscriber {
        WhisperTranscriber { version, model, language }
     }
}

impl Transcriber for WhisperTranscriber {
    fn transcribe(&self, in_file: &str, out_file: &str) -> anyhow::Result<()> {
        run_whisper(&self.version, &self.model, &self.language, in_file, out_file)
    }
}

// 一意の一時ファイル名を生成する関数
fn generate_temp_file_path(ext: &str) -> PathBuf {
    // TEMPディレクトリを取得
    let temp_dir = std::env::temp_dir();
    // 一意のIDを生成
    let unique_id = Uuid::new_v4();
    // ファイル名を構築
    let file_name = format!("{}.{}", unique_id, ext);
    // 完全なパスを作成
    Path::join(&temp_dir, file_name)
}

fn run_whisper(version_tag: &str, model: &str, language: &str, in_file: &str, out_file: &str) -> anyhow::Result<()> {
    let cache_dir = dirs::cache_dir()
        .ok_or(anyhow!("Cannot get cache directory"))?;

    let whisper_dir = cache_dir.join("whisper.cpp");
    if !whisper_dir.join(".git").exists() {
        log::info!("git clone whisper.cpp to {:?}", whisper_dir);

        let output = Command::new("git")
            .arg("clone")
            .arg("git@github.com:ggerganov/whisper.cpp.git")
            .arg("-b").arg(version_tag)
            .arg(whisper_dir.as_os_str())
            .output()?;
        if !output.status.success() {
            return Err(anyhow!("Cannot download model: {}", String::from_utf8_lossy(&output.stderr)));
        }
    } else {
        {
            let mut command = Command::new("git");
            command.current_dir(whisper_dir.clone())
                .arg("fetch");
            log::info!("Checkout whipser.cpp {}: {:?}", version_tag, command);
            let output = match command.output() {
                Ok(output) => { output }
                Err(err) => {
                    return Err(anyhow!("Cannot fetch remote repository info({}): {:?}", version_tag, err));
                }
            };
            if !output.status.success() {
                return Err(anyhow!("Cannot fetch: {}",
                String::from_utf8_lossy(&output.stderr)));
            }
        }
        {
            let mut command = Command::new("git");
            command.current_dir(whisper_dir.clone())
                .arg("checkout")
                .arg(version_tag);
            log::info!("Checkout whipser.cpp {}: {:?}", version_tag, command);
            let output = match command.output() {
                Ok(output) => { output }
                Err(err) => {
                    return Err(anyhow!("Cannot checkout directory({}): {:?}", version_tag, err));
                }
            };
            if !output.status.success() {
                return Err(anyhow!("Cannot checkout directory: {}",
                String::from_utf8_lossy(&output.stderr)));
            }
        }
    }

    let model_file = whisper_dir.join(format!("models/ggml-{}.bin", model));
    if !model_file.exists() {
        log::info!("download model file... {:?}", model_file);
        let output = Command::new("./models/download-ggml-model.sh")
            .arg(model)
            .current_dir(&whisper_dir)
            .output()?;
        if !output.status.success() {
            return Err(anyhow!("Cannot download model: {}", String::from_utf8_lossy(&output.stderr)));
        }
    }

    let main_file = whisper_dir.join("main");
    if !main_file.exists() {
        log::info!("Build whsisper.cpp");
        let output = Command::new("make")
            .current_dir(&whisper_dir)
            .output()?;
        if !output.status.success() {
            return Err(anyhow!("Cannot build whisper.cpp: {}", String::from_utf8_lossy(&output.stderr)));
        }
    }

    // 16kbps, 16bit is required for whisper.cpp https://github.com/ggerganov/whisper.cpp
    let temp_file_path = generate_temp_file_path("wav");

    // Use ffmpeg to convert the input file to the desired sample rate and bit depth
    log::info!("Starting ffmpeg to convert wave file...");
    let output = match Command::new("ffmpeg")
        .args([
            "-i", in_file,
            "-ar", "16000",
            "-acodec", "pcm_s16le",
            temp_file_path.as_path().to_str().unwrap()
        ])
        .output() {
        Ok(output) => { output }
        Err(err) => {
            return Err(anyhow!("Cannot run ffmpeg: {}", err))
        }
    };
    if !output.status.success() {
        return Err(anyhow!("ffmpeg failed to convert the WAV file: {:?} {:?}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    log::info!("[whisper.cpp] Start transcribing... {} to {}", in_file, out_file);
    let start = Instant::now();
    let output = match Command::new("./main")
        .args([
            "--language", language,
            "-m", &format!("models/ggml-{}.bin", model),
            "-ovtt",
            "-of", &out_file.replace(".vtt", "").to_string(),
            "-f", temp_file_path.to_str().unwrap()
        ])
        .current_dir(&whisper_dir)
        .output() {
        Ok(output) => { output }
        Err(err) => {
            return Err(anyhow!("Cannot run whisper: {}", err))
        }
    };
    if !output.status.success() {
        if let Err(err) = fs::remove_file(temp_file_path.to_str().unwrap()) {
            return Err(anyhow!("Cannot remove file({:?}): {:?}", temp_file_path, err));
        }
        return Err(anyhow!("Cannot run whisper.cpp: {} {}",
            in_file,
            String::from_utf8_lossy(&output.stderr)));
    }

    if let Err(err) = fs::remove_file(temp_file_path.to_str().unwrap()) {
        return Err(anyhow!("Cannot remove file({:?}): {:?}", temp_file_path, err));
    }

    log::info!("Ran whisper.cpp: {:?}, {:?}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let duration = start.elapsed(); // 経過時間を取得
    log::info!("whisper.cpp execution time: {:?}", duration);

    Ok(())
}
