mod recorder;

use eframe::egui;
use recorder::{AudioRecorder, RecorderState};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MultiRecorder - Modern Audio Recorder",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MultiRecorderApp::new()))
        }),
    )
}

struct MultiRecorderApp {
    recorder: AudioRecorder,
    status_message: String,
    recording_duration: f32,
    input_devices: Vec<String>,
    selected_device_idx: usize,
    output_filename: String,
}

impl MultiRecorderApp {
    fn new() -> Self {
        let recorder = AudioRecorder::new();
        let mut app = Self {
            recorder,
            status_message: "Ready to record".to_string(),
            recording_duration: 0.0,
            input_devices: Vec::new(),
            selected_device_idx: 0,
            output_filename: format!(
                "recording_{}_{}.wav",
                chrono::Local::now().format("%Y%m%d_%H%M%S"),
                chrono::Local::now().timestamp_subsec_millis()
            ),
        };
        app.refresh_devices();
        app
    }

    fn refresh_devices(&mut self) {
        self.input_devices = self.recorder.get_input_devices();
        if self.selected_device_idx >= self.input_devices.len() {
            self.selected_device_idx = 0;
        }
    }

    fn start_recording(&mut self) {
        match self
            .recorder
            .start_recording(self.selected_device_idx, &self.output_filename)
        {
            Ok(_) => {
                self.status_message = "Recording started...".to_string();
                self.recording_duration = 0.0;
            }
            Err(e) => {
                self.status_message = format!("Error: {}", e);
            }
        }
    }

    fn stop_recording(&mut self) {
        match self.recorder.stop_recording() {
            Ok(_) => {
                self.status_message = format!("Recording saved to: {}", self.output_filename);
            }
            Err(e) => {
                self.status_message = format!("Error stopping: {}", e);
            }
        }
    }

    fn is_recording(&self) -> bool {
        matches!(self.recorder.get_state(), RecorderState::Recording)
    }
}

impl eframe::App for MultiRecorderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update recording duration
        if self.is_recording() {
            self.recording_duration += ctx.input(|i| i.stable_dt);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading("🎙️ MultiRecorder");
                ui.label("Modern Multi-Track Audio Recorder");
                ui.add_space(10.0);
            });

            ui.separator();
            ui.add_space(20.0);

            // Device selection
            egui::Grid::new("device_grid")
                .num_columns(2)
                .spacing([40.0, 10.0])
                .show(ui, |ui| {
                    ui.label("Input Device:");
                    egui::ComboBox::from_label("")
                        .selected_text(
                            self.input_devices
                                .get(self.selected_device_idx)
                                .unwrap_or(&"No device".to_string()),
                        )
                        .show_ui(ui, |ui| {
                            for (idx, device) in self.input_devices.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_device_idx, idx, device);
                            }
                        });
                    ui.end_row();

                    ui.label("Output Filename:");
                    ui.text_edit_singleline(&mut self.output_filename);
                    ui.end_row();
                });

            ui.add_space(10.0);

            if ui.button("🔄 Refresh Devices").clicked() {
                self.refresh_devices();
                self.status_message = "Devices refreshed".to_string();
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Recording controls
            ui.horizontal(|ui| {
                ui.add_space(20.0);

                let is_recording = self.is_recording();

                if is_recording {
                    if ui
                        .add_sized([150.0, 50.0], egui::Button::new("⏹ Stop Recording"))
                        .clicked()
                    {
                        self.stop_recording();
                    }
                } else if ui
                    .add_sized([150.0, 50.0], egui::Button::new("⏺ Start Recording"))
                    .clicked()
                {
                    self.start_recording();
                }

                ui.add_space(20.0);

                if is_recording {
                    ui.label(format!("⏱ Duration: {:.1}s", self.recording_duration));

                    // Animated recording indicator
                    let time = ui.input(|i| i.time);
                    let blink = (time * 2.0).sin() > 0.0;
                    if blink {
                        ui.colored_label(egui::Color32::RED, "🔴 REC");
                    }
                }
            });

            ui.add_space(20.0);

            // Status area
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(40, 40, 45))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 65)))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Status:");
                        ui.label(&self.status_message);
                    });
                });

            ui.add_space(20.0);
            ui.separator();

            // Info panel
            ui.collapsing("ℹ️ Information", |ui| {
                ui.label("Features:");
                ui.label("  • Record audio from any input device");
                ui.label("  • High-quality WAV output");
                ui.label("  • Real-time recording indicator");
                ui.label("  • Modern and intuitive interface");
                ui.add_space(10.0);
                ui.label("Made with ❤️ using Rust and egui");
            });
        });

        // Request repaint for animations
        if self.is_recording() {
            ctx.request_repaint();
        }
    }
}
