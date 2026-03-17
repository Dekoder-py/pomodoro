use std::{thread, time::{Duration, Instant}};

use eframe::egui;

struct PomoApp {
    start_time: Instant,
    timer_running: bool
}

impl PomoApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            start_time: Instant::now(),
            timer_running: false,
        }
    }
}

impl eframe::App for PomoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pomodoro!");

            ui.add_space(10.0);

            if ui.button("Start").clicked() {
                self.start_time = Instant::now();
                self.timer_running = true;
            }

            if self.timer_running {
                if self.start_time.elapsed().as_secs() > 300 { // 5 minutes
                    self.timer_running = false
                } else {
                    ui.label(format!("{}", self.start_time.elapsed().as_secs()));
                    thread::sleep(Duration::from_secs(1));
                }
            }
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pomodoro",
        native_options,
        Box::new(|cc| Ok(Box::new(PomoApp::new(cc)))),
    )
}
