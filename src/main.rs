#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::{Path, PathBuf};

use clap::Parser;
use eframe::egui::{self, Context, Ui, Window};
use tokio::runtime::Runtime;

pub const LOREM_IPSUM_LONG: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

struct LogViewer {}

fn lorem_ipsum(ui: &mut egui::Ui) {
    ui.with_layout(
        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
        |ui| {
            ui.label(egui::RichText::new(crate::LOREM_IPSUM_LONG));
        },
    );
}

impl eframe::App for LogViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("assigned_issues").show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    lorem_ipsum(ui);
                });
            });
            egui::SidePanel::left("assigned_prs").show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    lorem_ipsum(ui);
                });
            });

            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

impl Default for LogViewer {
    fn default() -> Self {
        LogViewer {}
    }
}

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
struct Args {}

struct GitHub {
    runtime: Runtime,
    client: octocrab::Octocrab,
}

impl GitHub {
    pub fn new(runtime: Runtime) -> Self {
        let client = octocrab::OctocrabBuilder::new().build();
        Self { runtime, client }
    }

    fn fetch_assigned_issues(&self) -> _ {
        todo!()
    }
}

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let github = GitHub::new(runtime);
    let assigned_issues = github.fetch_assigned_issues().unwrap();

    /* TODO GUI
    let args = Args::parse();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };

    let app = LogViewer::default();
    eframe::run_native("Log Viewer", options, Box::new(|_cc| Box::new(app)));
    */
}
