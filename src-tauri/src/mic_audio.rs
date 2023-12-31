use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BuildStreamError, Device, FromSample, Sample, Stream, SupportedStreamConfig};
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};
use anyhow::anyhow;

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

pub struct MicAudioRecorder {
    pub stream: Stream,
    pub writer: WavWriterHandle,
    pub output_file: String,
}

impl MicAudioRecorder {
    pub fn new(output_file: &str, input_device: &Device) -> anyhow::Result<Self> {
        let config: SupportedStreamConfig = input_device
            .default_input_config()
            .map_err(|e| {
                anyhow!("Failed to get default input config({:?}): {:?}", input_device.name(), e)
            })?;

        let spec = hound::WavSpec {
            channels: config.channels() as _,
            sample_rate: config.sample_rate().0 as _,
            bits_per_sample: (config.sample_format().sample_size() * 8) as _,
            sample_format: sample_format(config.sample_format()),
        };

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
        }?;

        Ok(MicAudioRecorder {
            stream,
            writer,
            output_file: String::from(output_file),
        })
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

pub fn get_input_devices() -> anyhow::Result<Vec<String>> {
    let mut result = Vec::new();

    let host = cpal::default_host();
    match host.input_devices() {
        Ok(devices) => {
            for device in devices {
                if let Ok(name) = device.name() {
                    result.push(name);
                }
            }
            Ok(result)
        }
        Err(err) => {
            Err(anyhow!("Cannot get input devices: {:?}", err))
        }
    }
}

pub fn select_input_device_by_name(target_device: &Option<String>) -> Device {
    log::info!("target device is : {:?}", target_device);

    let host = cpal::default_host();
    if let Some(target_device) = target_device {
        match host.input_devices() {
            Ok(devices) => {
                for device in devices {
                    if let Ok(name) = device.name() {
                        if &name == target_device {
                            log::info!("Selected audio device: {}", name);
                            return device
                        }
                    }
                }
            }
            Err(err) => {
                log::error!("Cannot get audio input device list: {}", err)
            }
        }
    }

    log::info!("Using default input device...");
    log::info!("Available devices are: {:?}", get_input_devices());
    host.default_input_device()
        .expect("There's no available input device.")
}
