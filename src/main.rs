use crate::tokenizer::TokensResult;
use egui::{FontId, RichText};
use std::error::Error;

use eframe::egui;

mod str_checker;
mod tokenizer;

fn main() -> Result<(), Box<dyn Error>> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 650.0]),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Tokenizer",
        options,
        Box::new(|_cc| Ok(Box::<GraphicApp>::default())),
    )?;

    Ok(())
}

struct GraphicApp {
    url: String,

    protocol: String,
    host: String,
    directories: String,
    filename: String,
    query: String,

    words: String,
    punctuations: String,
    numbers: String,
    alphanumeric: String,

    is_valid_url: String,

    granular: String,
}

impl Default for GraphicApp {
    fn default() -> Self {
        Self {
            url: String::default(),

            protocol: String::default(),
            host: String::default(),
            directories: String::default(),
            filename: String::default(),
            query: String::default(),

            words: String::default(),
            punctuations: String::default(),
            numbers: String::default(),
            alphanumeric: String::default(),

            is_valid_url: String::default(),

            granular: String::default(),
        }
    }
}

impl eframe::App for GraphicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new("URL Tokenizer")
                        .font(FontId::proportional(40.0))
                        .strong(),
                );
            });

            ui.add_space(20.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Input:").font(FontId::proportional(20.0)));
                    let response = ui.add_sized(
                        [ui.available_width(), 30.0],
                        egui::TextEdit::singleline(&mut self.url),
                    );

                    // Execute if Enter key is pressed.
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let result: TokensResult = TokensResult::new(&self.url);

                        self.protocol = result.protocol.unwrap_or("").to_string();
                        self.host = result.host.unwrap_or("").to_string();
                        self.directories = format!("{:#?}", result.directories);
                        self.filename = result.filename.unwrap_or("").to_string();
                        self.query = result.query.unwrap_or("").to_string();

                        self.words = format!("{:#?}", result.words);
                        self.numbers = format!("{:#?}", result.numbers);
                        self.alphanumeric = format!("{:#?}", result.alphanumeric);
                        self.punctuations = format!("{:#?}", result.punctuations);

                        self.granular = result.granular();

                        self.is_valid_url = if result.is_valid_url {
                            "The input is a valid URL.".to_string()
                        } else {
                            "The input is not a valid URL.".to_string()
                        };
                    }
                });

                ui.add_space(10.0);
                ui.label(&self.is_valid_url);
                ui.add_space(10.0);

                ui.columns(2, |columns| {
                    columns[0].label(RichText::new("Protocol:").font(FontId::proportional(20.0)));
                    columns[0].add(egui::TextEdit::multiline(&mut self.protocol.as_str()));
                    columns[0].add_space(20.0);
                    columns[1].label(RichText::new("Word:").font(FontId::proportional(20.0)));
                    columns[1].add(egui::TextEdit::multiline(&mut self.words.as_str()));
                    columns[1].add_space(20.0);

                    columns[0].label(RichText::new("Host:").font(FontId::proportional(20.0)));
                    columns[0].add(egui::TextEdit::multiline(&mut self.host.as_str()));
                    columns[0].add_space(20.0);
                    columns[1]
                        .label(RichText::new("Punctuation:").font(FontId::proportional(20.0)));
                    columns[1].add(egui::TextEdit::multiline(&mut self.punctuations.as_str()));
                    columns[1].add_space(20.0);

                    columns[0]
                        .label(RichText::new("Directories:").font(FontId::proportional(20.0)));
                    columns[0].add(egui::TextEdit::multiline(&mut self.directories.as_str()));
                    columns[0].add_space(20.0);
                    columns[1].label(RichText::new("Number:").font(FontId::proportional(20.0)));
                    columns[1].add(egui::TextEdit::multiline(&mut self.numbers.as_str()));
                    columns[1].add_space(20.0);

                    columns[0].label(RichText::new("Filename:").font(FontId::proportional(20.0)));
                    columns[0].add(egui::TextEdit::multiline(&mut self.filename.as_str()));
                    columns[0].add_space(20.0);
                    columns[1]
                        .label(RichText::new("Alphanumeric:").font(FontId::proportional(20.0)));
                    columns[1].add(egui::TextEdit::multiline(&mut self.alphanumeric.as_str()));
                    columns[1].add_space(20.0);

                    columns[0].label(RichText::new("Query:").font(FontId::proportional(20.0)));
                    columns[0].add(egui::TextEdit::multiline(&mut self.query.as_str()));
                    columns[0].add_space(20.0);
                    /*columns[1]
                        .label(RichText::new("Other Tokens:").font(FontId::proportional(20.0)));
                    columns[1].add(egui::TextEdit::multiline(&mut self.others.as_str()));
                    columns[1].add_space(20.0);*/
                });

                ui.label(RichText::new("Granular breakdown:").font(FontId::proportional(20.0)));
                ui.add_sized(
                    [ui.available_width(), 256.0],
                    egui::TextEdit::multiline(&mut self.granular.as_str()),
                );
            });
        });
    }
}
