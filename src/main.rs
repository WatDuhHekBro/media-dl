// Hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use derivative::Derivative;
use eframe::egui;
use std::process::Command;

// youtube-dl --all-subs --skip-download
// https://github.com/ytdl-org/youtube-dl/issues/8114

fn main() {
    let a = Command::new("youtube-dl").spawn();
    match a {
        Ok(child) => {}
        Err(error) => {
            println!(
                "You most likely don't have \"youtube-dl\" in your PATH variable.\n{:?}",
                error
            );
        }
    }

    /*let options = eframe::NativeOptions::default();

    eframe::run_native(
        &format!(
            "yaytdlg - Yet Another youtube-dl GUI (v{})",
            env!("CARGO_PKG_VERSION")
        ),
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );*/
}

enum Mode {
    Default,
    Flags,
    Processing,
    Done,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Default
    }
}

#[derive(Derivative)]
#[derivative(Default)]
struct MyApp {
    mode: Mode,
    links: String,
    error: String,
    get_video: bool,
    get_audio: bool,
    get_desc: bool,
    get_subs: bool,
    get_thumb: bool,
    #[derivative(Default(value = "true"))]
    is_audio_hq: bool,
    embed_subtitles: bool,
    #[derivative(Default(value = r#"String::from("general")"#))]
    download_directory: String,
    console: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.mode {
                Mode::Default => {
                    ui.heading("Enter your links");
                    ui.text_edit_multiline(&mut self.links);

                    if ui.button("Submit").clicked() {
                        if self.links.is_empty() {
                            self.error = String::from("You must enter some links first!");
                        } else {
                            self.error = String::from("");
                            self.mode = Mode::Flags;

                            // Setup parsed list of links, done once here instead of the event loop
                        }
                    }

                    if !self.error.is_empty() {
                        ui.label(&self.error);
                    }
                }
                Mode::Flags => {
                    ui.heading("Choose the settings you want");

                    ui.horizontal(|ui| {
                        ui.label("Components:");
                        ui.checkbox(&mut self.get_video, "Video");
                        ui.checkbox(&mut self.get_audio, "Audio");
                        ui.checkbox(&mut self.get_desc, "Description");
                        ui.checkbox(&mut self.get_subs, "Subtitles");
                        ui.checkbox(&mut self.get_thumb, "Thumbnail");
                    });

                    ui.horizontal(|ui| {
                        if self.get_audio {
                            ui.vertical(|ui| {
                                ui.heading("Audio Settings");
                                ui.checkbox(&mut self.is_audio_hq, "HQ Audio?")
                            });
                        }

                        if self.get_thumb {
                            ui.vertical(|ui| {
                                ui.heading("Thumbnail Settings");
                                ui.checkbox(&mut self.embed_subtitles, "Embed Thumbnail?")
                            });
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Download Directory:");
                        ui.text_edit_singleline(&mut self.download_directory);
                    });

                    // Only allow processing to begin if at least one component is selected.
                    ui.add_enabled_ui(
                        self.get_video
                            || self.get_audio
                            || self.get_desc
                            || self.get_subs
                            || self.get_thumb,
                        |ui| {
                            if ui.button("Begin Processing").clicked() {
                                self.mode = Mode::Processing;

                                // Setup processing flags, done once here instead of the event loop
                            }
                        },
                    );
                }
                Mode::Processing => {
                    ui.add_enabled_ui(false, |ui| {
                        ui.text_edit_multiline(&mut self.console);
                    });

                    // Detect playlist URLs and handle them accordingly

                    if ui.button("tmp").clicked() {
                        self.mode = Mode::Done;
                    }
                }
                Mode::Done => {
                    ui.label("Your batch has finished processing. Would you like to submit another batch?");

                    if ui.button("Yes").clicked() {
                        self.mode = Mode::Default; // [TODO] Add mutable reset function called at beginning of Mode::Default, eliminates need for derivate
                    }

                    if ui.button("No").clicked() {
                        frame.close();
                    }
                }
            };
        });
    }
}
