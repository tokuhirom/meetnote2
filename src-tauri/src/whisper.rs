use std::process::Command;
use anyhow::anyhow;
use std::time::Instant;

// language: ja
pub(crate) fn run_whisper(version_tag: &str, model: &str, language: &str, in_file: &str, out_file: &str) -> anyhow::Result<()> {
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
        let output = Command::new("git")
            .current_dir(whisper_dir.clone())
            .arg("checkout")
            .arg(version_tag)
            .output()?;
        if !output.status.success() {
            return Err(anyhow!("Cannot checkout directory: {}", String::from_utf8_lossy(&output.stderr)));
        }
    }

    let model_file = whisper_dir.join(&format!("models/ggml-{}.bin", model));
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

    log::info!("Start transcribing...{} to {}", in_file, out_file);
    let start = Instant::now();
    let output = Command::new("./main")
        .args(&[
            "--language", language,
            "-m", &format!("models/ggml-{}.bin", model),
            "-ovtt",
            "-of", &out_file.replace(".vtt", "").to_string(),
            "-f", in_file
        ])
        .current_dir(&whisper_dir)
        .output()?;
    if !output.status.success() {
        return Err(anyhow!("Cannot run whisper.cpp: {} {}",
            in_file,
            String::from_utf8_lossy(&output.stderr)));
    }

    log::info!("Ran whisper.cpp: {:?}, {:?}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let duration = start.elapsed(); // 経過時間を取得
    log::info!("whisper.cpp execution time: {:?}", duration);

    Ok(())
}
