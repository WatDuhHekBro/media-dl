use crate::util::{spawn_downloader, Message, Mode};
use derivative::Derivative;
use eframe::egui;
use std::sync::mpsc::{channel, Receiver};
use tokio::task::spawn_local;

pub fn init_gui() {
    eframe::run_native(
        &format!(
            "yaytdlg - Yet Another youtube-dl GUI (v{})",
            env!("CARGO_PKG_VERSION")
        ),
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(App::default())),
    );
}

#[derive(Derivative)]
#[derivative(Default)]
struct App {
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
    console_receiver: Option<Receiver<Message>>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

                                // Setup parsed list of links, done once here instead of the event loop
                                // Setup processing flags, done once here instead of the event loop
                                // Detect playlist URLs and handle them accordingly
                                //let parsed_links = self.links.split("\n").collect::<Vec<&str>>();
                                let args = Vec::new();

                                // Start an async thread via tokio. Messages from youtube-dl will be passed via a channel.
                                let (tx, rx) = channel();
                                spawn_local(spawn_downloader(tx, args)); // FIX: Panics if called outside tokio::task::LocalSet
                                self.console_receiver = Some(rx);
                            }
                        },
                    );
                }
                Mode::Processing => {
                    // If console_inbox is still None for whatever reason, ignore it instead of crashing the program.
                    if let Some(receiver) = &self.console_receiver {
                        let status = receiver.try_recv();
                    }

                    ui.add_enabled_ui(false, |ui| {
                        ui.text_edit_multiline(&mut self.console);
                    });

                    if ui.button("tmp").clicked() {
                        self.mode = Mode::Done;
                    }
                }
                Mode::Done => {
                    ui.label("Your batch has finished processing.");
                }
            };
        });
    }
}
