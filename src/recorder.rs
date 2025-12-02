use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use hound::{WavSpec, WavWriter};
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecorderState {
    Idle,
    Recording,
}

pub struct AudioRecorder {
    state: RecorderState,
    stream: Option<Stream>,
    writer: Option<Arc<Mutex<WavWriter<BufWriter<File>>>>>,
    devices: Vec<Device>,
}

impl AudioRecorder {
    pub fn new() -> Self {
        Self {
            state: RecorderState::Idle,
            stream: None,
            writer: None,
            devices: Self::enumerate_devices(),
        }
    }

    pub fn get_state(&self) -> RecorderState {
        self.state
    }

    fn enumerate_devices() -> Vec<Device> {
        let host = cpal::default_host();
        host.input_devices()
            .map(|devices| devices.collect())
            .unwrap_or_default()
    }

    pub fn get_input_devices(&self) -> Vec<String> {
        self.devices
            .iter()
            .filter_map(|device| device.name().ok())
            .collect()
    }

    pub fn start_recording(&mut self, device_idx: usize, filename: &str) -> Result<(), String> {
        if self.state == RecorderState::Recording {
            return Err("Already recording".to_string());
        }

        let device = self
            .devices
            .get(device_idx)
            .ok_or_else(|| "Invalid device index".to_string())?;

        let config = device
            .default_input_config()
            .map_err(|e| format!("Failed to get default input config: {}", e))?;

        let sample_rate = config.sample_rate().0;
        let channels = config.channels();

        // Create WAV file
        let spec = WavSpec {
            channels,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let writer = WavWriter::create(filename, spec)
            .map_err(|e| format!("Failed to create WAV file: {}", e))?;

        let writer = Arc::new(Mutex::new(writer));
        let writer_clone = Arc::clone(&writer);

        // Build the input stream
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                self.build_input_stream::<f32>(device, &config.into(), writer_clone)
            }
            cpal::SampleFormat::I16 => {
                self.build_input_stream::<i16>(device, &config.into(), writer_clone)
            }
            sample_format => return Err(format!("Unsupported sample format: {}", sample_format)),
        }?;

        stream
            .play()
            .map_err(|e| format!("Failed to start stream: {}", e))?;

        self.stream = Some(stream);
        self.writer = Some(writer);
        self.state = RecorderState::Recording;

        Ok(())
    }

    fn build_input_stream<T>(
        &self,
        device: &Device,
        config: &StreamConfig,
        writer: Arc<Mutex<WavWriter<BufWriter<File>>>>,
    ) -> Result<Stream, String>
    where
        T: cpal::SizedSample + hound::Sample,
    {
        let err_fn = |err| eprintln!("Error during recording: {}", err);

        let stream = device
            .build_input_stream(
                config,
                move |data: &[T], _: &cpal::InputCallbackInfo| {
                    if let Ok(mut writer) = writer.lock() {
                        for &sample in data {
                            if let Err(e) = writer.write_sample(sample) {
                                eprintln!("Error writing sample: {}", e);
                            }
                        }
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| format!("Failed to build input stream: {}", e))?;

        Ok(stream)
    }

    pub fn stop_recording(&mut self) -> Result<(), String> {
        if self.state != RecorderState::Recording {
            return Err("Not currently recording".to_string());
        }

        // Stop and drop the stream
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }

        // Finalize the WAV file
        if let Some(writer) = self.writer.take() {
            if let Ok(writer) = Arc::try_unwrap(writer) {
                if let Ok(writer) = writer.into_inner() {
                    writer
                        .finalize()
                        .map_err(|e| format!("Failed to finalize WAV file: {}", e))?;
                }
            }
        }

        self.state = RecorderState::Idle;
        Ok(())
    }
}

impl Drop for AudioRecorder {
    fn drop(&mut self) {
        if let Err(e) = self.stop_recording() {
            eprintln!("Error stopping recording during cleanup: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder_initialization() {
        let recorder = AudioRecorder::new();
        assert_eq!(recorder.get_state(), RecorderState::Idle);
    }

    #[test]
    fn test_get_input_devices() {
        let recorder = AudioRecorder::new();
        let devices = recorder.get_input_devices();
        // Should return a list (might be empty in CI environments)
        // Just verify it doesn't panic
        let _ = devices.len();
    }

    #[test]
    fn test_state_transitions() {
        let recorder = AudioRecorder::new();
        assert_eq!(recorder.get_state(), RecorderState::Idle);

        // After drop, state should still be idle if no recording started
        drop(recorder);
    }
}
