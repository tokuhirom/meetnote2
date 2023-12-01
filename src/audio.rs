use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BuildStreamError, Device, FromSample, Sample, Stream, SupportedStreamConfig};
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

pub struct AudioRecorder {
    pub stream: Stream,
    pub writer: WavWriterHandle,
}

impl AudioRecorder {
    pub fn new(output_file: &str, input_device: &Device) -> Self {
        let config: SupportedStreamConfig = input_device
            .default_input_config()
            .expect("Failed to get default input config");

        let spec = hound::WavSpec {
            channels: config.channels() as _,
            sample_rate: config.sample_rate().0 as _,
            bits_per_sample: (config.sample_format().sample_size() * 8) as _,
            sample_format: sample_format(config.sample_format()),
        };
        println!("PATH: {:?}", output_file);

        let hounder_writer: hound::WavWriter<BufWriter<File>> =
            hound::WavWriter::create(output_file, spec).unwrap();

        /*
        - Wrapping inside a Mutex, which is used for synchronization.
        It allows multiple threads to safely access the Option
        by locking and unlocking the Mutex when needed.

        - Then creating a Arc instance.
        It allows multiple parts of your code to access and modify the Mutex safely
        by keeping track of the number of references to it.
        */
        let writer = Arc::new(Mutex::new(Some(hounder_writer)));
        let writer_cloned = writer.clone();

        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        let stream: cpal::Stream = match config.sample_format() {
            cpal::SampleFormat::I8 => input_device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i8, i8>(data, &writer_cloned),
                err_fn,
                None,
            ),
            cpal::SampleFormat::I16 => input_device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i16, i16>(data, &writer_cloned),
                err_fn,
                None,
            ),
            cpal::SampleFormat::I32 => input_device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i32, i32>(data, &writer_cloned),
                err_fn,
                None,
            ),
            cpal::SampleFormat::F32 => input_device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<f32, f32>(data, &writer_cloned),
                err_fn,
                None,
            ),
            _sample_format => Err(BuildStreamError::DeviceNotAvailable),
        }
            .unwrap();

        AudioRecorder { stream, writer }
    }

    pub fn start_recording(&mut self) {
        println!("Starting audio recording");
        self.stream.play().unwrap();
    }

    pub fn stop_recording(&mut self) {
        self.stream.pause().unwrap();

        // close the writer
        self.writer
            .lock()
            .unwrap()
            .take()
            .unwrap()
            .finalize()
            .unwrap();
    }
}

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    if format.is_float() {
        hound::SampleFormat::Float
    } else {
        hound::SampleFormat::Int
    }
}

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
    where
        T: Sample,
        U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}

pub fn select_input_device_by_name(target_device: Option<String>) -> Device {
    let host = cpal::default_host();
    if let Some(target_device) = target_device {
        match host.input_devices() {
            Ok(devices) => {
                for device in devices {
                    if let Ok(name) = device.name() {
                        if (name == target_device) {
                            println!("Selected audio device: {}", name);
                            return device
                        }
                    }
                }
            }
            Err(err) => {
                println!("Cannot get audio input device list: {}", err)
            }
        }
    }

    println!("Using default input device...");
    return host.default_input_device()
        .expect("There's no available input device.")
}
