use std::time::{Duration, Instant};

use eframe::egui;

struct PomoApp {
    start_time: Instant,
    timer_running: bool,
    has_run: bool,
}

impl PomoApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            start_time: Instant::now(),
            timer_running: false,
            has_run: false,
        }
    }
}

impl eframe::App for PomoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pomodoro!");

            ui.add_space(10.0);
            if !self.timer_running {
                if ui.button("Start").clicked() {
                    self.start_time = Instant::now();
                    self.timer_running = true;
                }
            } else {
                if ui.button("Stop").clicked() {
                    self.has_run = true;
                    self.timer_running = false;
                }
            }

            if self.timer_running {
                let elapsed = self.start_time.elapsed();
                let secs = elapsed.as_secs();
                let remaining = 10 - secs;
                let remaining_mins = remaining / 60;
                let remaining_secs = remaining % 60;

                if remaining <= 0 {
                    // 5 minutes
                    self.timer_running = false;
                    self.has_run = true;
                } else {
                    ui.label(format!("{remaining_mins}:{remaining_secs}"));
                    let next_sec = Duration::from_secs(secs + 1);

                    ctx.request_repaint_after(next_sec - elapsed);
                }
            } else {
                // timer has finished or timer is not started yet
                if self.has_run {
                    ui.label("Great work! Have a break :)");
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
