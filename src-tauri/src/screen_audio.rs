use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Deref;
use anyhow::anyhow;
use screencapturekit::cm_sample_buffer::CMSampleBuffer;
use screencapturekit::sc_content_filter::{InitParams, SCContentFilter};
use screencapturekit::sc_error_handler::StreamErrorHandler;
use screencapturekit::sc_output_handler::{SCStreamOutputType, StreamOutput};
use screencapturekit::sc_shareable_content::SCShareableContent;
use screencapturekit::sc_stream::SCStream;
use screencapturekit::sc_stream_configuration::SCStreamConfiguration;


struct ErrorHandler;

impl StreamErrorHandler for ErrorHandler {
    fn on_error(&self) {
        eprintln!("ERROR!")
    }
}

struct StoreAudioHandler {
    file_prefix: String,
}

impl StreamOutput for StoreAudioHandler {
    fn did_output_sample_buffer(&self, sample: CMSampleBuffer, _of_type: SCStreamOutputType) {
        // let format_description = sample.sys_ref.get_format_description()
        //     .expect("format description");
        // let description = format_description.audio_format_description_get_stream_basic_description()
        //     .expect("Get AudioStreamBasicDescription");
        // println!("format_description={:?}", description);

        let audio_buffers = sample.sys_ref.get_av_audio_buffer_list();
        // println!("audio buffer list: number={:?}", audio_buffers.len());
        for i in 0..audio_buffers.len() {
            let buffer = &audio_buffers[i];
            // println!("  {}: channels={}, size={}", i, buffer.number_channels, buffer.data.len());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)  // Use append mode
                .open(format!("{}-{}.raw", self.file_prefix, i))
                .expect("failed to open file");

            if let Err(e) = file.write_all(buffer.data.deref()) {
                eprintln!("failed to write to file: {:?}", e);
            }
        }
    }
}

pub struct ScreenAudioRecorder {
    pub stream: SCStream,
}

impl ScreenAudioRecorder {
    pub fn new(file_prefix: String) -> anyhow::Result<ScreenAudioRecorder> {
        let current = SCShareableContent::current();
        let display = current.displays.first()
            .ok_or(anyhow!("Cannot get display information"))?;

        let config = SCStreamConfiguration {
            width: 100,
            height: 100,
            captures_audio: true,
            excludes_current_process_audio: true,
            ..Default::default()
        };
        let filter = SCContentFilter::new(InitParams::Display(display.clone()));
        let mut stream = SCStream::new(filter, config, ErrorHandler);
        stream.add_output(StoreAudioHandler { file_prefix }, SCStreamOutputType::Audio);

        Ok(ScreenAudioRecorder {
            stream,
        })
    }

    pub fn start_recording(&self) {
        self.stream.start_capture()
    }

    pub fn stop_recording(&self) {
        self.stream.stop_capture()
    }
}