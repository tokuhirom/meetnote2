mod audio;

use core_graphics::{
    access::ScreenCaptureAccess,
};
use screencapturekit::{
    sc_shareable_content::SCShareableContent,
};
use std::process::Command;
use core_graphics::display::{CGDirectDisplayID, CGDisplay, CGMainDisplayID};
use core_video_sys::{CVPixelBufferGetBaseAddressOfPlane, CVPixelBufferGetBytesPerRowOfPlane, CVPixelBufferGetHeightOfPlane, CVPixelBufferGetWidthOfPlane, CVPixelBufferLockBaseAddress, CVPixelBufferRef, CVPixelBufferUnlockBaseAddress};
use screencapturekit::cm_sample_buffer::CMSampleBuffer;
use screencapturekit::sc_content_filter::{InitParams, SCContentFilter};
use screencapturekit::sc_display::SCDisplay;
use screencapturekit::sc_error_handler::StreamErrorHandler;
use screencapturekit::sc_output_handler::{SCStreamOutputType, StreamOutput};
use screencapturekit::sc_stream::SCStream;
use screencapturekit::sc_stream_configuration::SCStreamConfiguration;
// use screencapturekit::{
//     sc_sys::SCFrameStatus,
// };

#[derive(Debug)]
pub enum TargetKind {
    Display,
    Window,
    Audio,
}

#[derive(Debug)]
pub struct Target {
    pub kind: TargetKind,
    pub title: String,
    pub id: u32,
}

#[derive(Debug)]
pub struct Options {
    pub fps: u32,
    pub show_cursor: bool,
    pub show_highlight: bool,
    pub targets: Vec<Target>,

    // excluded targets will only work on macOS
    pub excluded_targets: Option<Vec<Target>>,
}

struct ErrorHandler;

impl StreamErrorHandler for ErrorHandler {
    fn on_error(&self) {
        println!("Error!");
    }
}

pub fn has_permission() -> bool {
    let access = ScreenCaptureAccess::default();
    access.request()
}

pub fn is_supported() -> bool {
    let min_version: Vec<u8> = "12.3\n".as_bytes().to_vec();
    let output = Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .expect("Failed to execute sw_vers command");

    let os_version = output.stdout;

    os_version >= min_version
}

pub fn get_targets() -> Vec<Target> {
    let mut targets: Vec<Target> = Vec::new();

    let content = SCShareableContent::current();
    let displays = content.displays;

    for display in displays {
        // println!("Display: {:?}", display);
        let title = format!("Display {}", display.display_id); // TODO: get this from core-graphics

        let target = Target {
            kind: TargetKind::Display,
            id: display.display_id,
            title,
        };

        targets.push(target);
    }

    let applications = content.applications;
    for application in applications {
        match application.bundle_identifier {
            Some(bundleId) => {
                println!("BundleID: {:?}", bundleId)
            }
            None => {}
        }
    }

    let windows = content.windows;
    for window in windows {
        match window.title {
            Some(title) => {
                let name = title;
                let app = window.owning_application.unwrap().application_name.unwrap();
                println!("Title: {:?}", app);

                let target = Target {
                    kind: TargetKind::Window,
                    id: window.window_id,
                    title: name,
                };

                targets.push(target);
            }
            None => {}
        }
    }

    // println!("Targets: {:?}", targets);
    targets
}

pub struct Recorder {
    audio_recorder: audio::AudioRecorder,
    options: Options,
    recorder: screencapturekit::sc_stream::SCStream,
}

impl Recorder {
    pub fn init(options: Options, output_file: &str) -> Self {
        let audio_recorder = audio::AudioRecorder::new(output_file);

        let recorder = create_recorder(&options);

        Recorder {
            audio_recorder,
            recorder,
            options,
        }
    }

    pub fn start_capture(&mut self) {
        self.audio_recorder.start_recording();
        self.recorder.start_capture();
    }

    pub fn stop_capture(&mut self) {
        self.audio_recorder.stop_recording();
        self.recorder.stop_capture();
    }
}

struct Capturer {}

impl StreamOutput for Capturer {
    fn did_output_sample_buffer(&self, sample: CMSampleBuffer, of_type: SCStreamOutputType) {
        match of_type {
            SCStreamOutputType::Screen => {
                let frame_status = &sample.frame_status;
                println!(" did_output_sample_buffer: {:?}",frame_status);

                // match frame_status {
                    // SCFrameStatus::Complete => {
                    //     let ptr = sample.ptr;
                    //     let timestamp = ptr.get_presentation_timestamp().value;
                    //     let buffer = ptr.get_image_buffer().get_raw() as CVPixelBufferRef;
                    //
                    //     let (width, height, data) = unsafe { get_data_from_buffer(buffer) };
                    //
                    //     println!("Frame: {}", timestamp);
                    // }
                    // _ => {}
                // }
            }
            SCStreamOutputType::Audio => {
                let frame_status = &sample.frame_status;
                println!("AUDIO frame_status: {:?}", frame_status)
            }
            _ => {}
        }
    }
}

pub fn create_recorder(options: &Options) -> SCStream {
    println!("Options: {:?}", options);

    let display = get_main_display();
    let display_id = display.display_id;

    let scale = get_scale_factor(display_id) as u32;
    let width = display.width * scale;
    let height = display.height * scale;

    let params = InitParams::Display(display.to_owned());
    let filter = SCContentFilter::new(params);

    let stream_config = SCStreamConfiguration {
        shows_cursor: true,

        // 録音設定
        captures_audio: true,
        sample_rate: 16000,
        channel_count: 2,

        width,
        height,
        ..Default::default()
    };

    let mut stream = SCStream::new(filter, stream_config, ErrorHandler);
    stream.add_output(Capturer {});

    stream
}

fn get_scale_factor(display_id: CGDirectDisplayID) -> u64 {
    let mode = CGDisplay::new(display_id).display_mode().unwrap();
    mode.pixel_width() / mode.width()
}

pub fn get_main_display() -> SCDisplay {
    let content = SCShareableContent::current();
    let displays = content.displays;

    let main_display_id = unsafe { CGMainDisplayID() };
    let main_display = displays
        .iter()
        .find(|display| display.display_id == main_display_id)
        .unwrap_or_else(|| {
            panic!("Main display not found");
        });

    main_display.to_owned()
}

// TEMP: get rgb data from sample buffer
pub unsafe fn get_data_from_buffer(pixel_buffer: CVPixelBufferRef) -> (usize, usize, Vec<u8>) {
    // Lock the base address
    CVPixelBufferLockBaseAddress(pixel_buffer, 0);

    // Check the format of the pixel buffer
    // let format = core_video_sys::CVPixelBufferGetPixelFormatType(pixel_buffer);

    // Currently: 875704438, kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange
    // TODO: Capture in BRGA format instead

    // Plane 1 — Y (Luma)
    let y_width = CVPixelBufferGetWidthOfPlane(pixel_buffer, 0);
    let y_height = CVPixelBufferGetHeightOfPlane(pixel_buffer, 0);
    let y_bytes_row = CVPixelBufferGetBytesPerRowOfPlane(pixel_buffer, 0);
    let y_address = CVPixelBufferGetBaseAddressOfPlane(pixel_buffer, 0);
    let y_stride = y_bytes_row - y_width;

    // Plane 2 — CbCr (Chroma)
    // let c_width = CVPixelBufferGetWidthOfPlane(pixel_buffer, 1);
    let c_height = CVPixelBufferGetHeightOfPlane(pixel_buffer, 1);
    let c_address = CVPixelBufferGetBaseAddressOfPlane(pixel_buffer, 1);
    let c_bytes_row = CVPixelBufferGetBytesPerRowOfPlane(pixel_buffer, 1);

    let y_data = std::slice::from_raw_parts(
        y_address as *const u8,
        y_height as usize * y_bytes_row as usize,
    );

    let c_data = std::slice::from_raw_parts(
        c_address as *const u8,
        c_height as usize * c_bytes_row as usize,
    );

    // unlock base address
    CVPixelBufferUnlockBaseAddress(pixel_buffer, 0);

    // Logs
    // println!("y_width: {:?}", y_width);
    // println!("y_height: {:?}", y_height);
    // println!("y_address: {:?}", y_address);
    // println!("y_bytes_per_row: {:?}", y_bytes_row);
    // println!("c_width: {:?}", c_width);
    // println!("c_height: {:?}", c_height);
    // println!("c_address: {:?}", c_address);
    // println!("c_bytes_per_row: {:?}", c_bytes_row);

    // println!("y_data: {:?}", y_data);
    // println!("c_data: {:?}", c_data);

    // Convert YUV buffer to RGB
    let data = Vec::new();
    // let data = ycbcr_to_rgb(&y_data, &c_data, y_width, y_height, y_stride);

    (y_width, y_height, data)
}
