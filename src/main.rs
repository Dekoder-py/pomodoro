use egui_thematic::ThemeConfig;
use std::time::{Duration, Instant};

use eframe::egui::{self, Color32, ProgressBar};
struct PomoApp {
    start_time: Instant,
    time: u64,
    time_input: String,
    timer_running: bool,
    has_run: bool,
}

impl PomoApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            start_time: Instant::now(),
            time: 1500, // default timer is 25 minutes (the standard for pomodoro)
            time_input: String::new(),
            timer_running: false,
            has_run: false,
        }
    }
}

impl eframe::App for PomoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = ThemeConfig::catppuccin_mocha_preset(); // THE BEST COLORSCHEME OF ALL TIME
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_visuals(theme.to_visuals());
            ui.heading(format!("{} Minute Pomodoro", self.time / 60));

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Time (mins):");
                let input_field = ui.text_edit_singleline(&mut self.time_input);
                if ui.button("Set").clicked()
                    || (input_field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                        && !self.time_input.is_empty()
                {
                    let time: u64 = self.time_input.parse().expect("Failed to parse time input");
                    self.time = time * 60;
                }
            });

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

            ui.add_space(10.0);
            ui.spacing();
            ui.add_space(5.0);

            if self.timer_running {
                let elapsed = self.start_time.elapsed();
                let secs = elapsed.as_secs();
                let remaining = self.time - secs;
                let remaining_mins = remaining / 60;
                let remaining_secs = remaining % 60;

                if remaining <= 0 {
                    self.timer_running = false;
                    self.has_run = true;
                } else {
                    let label_text = format!("{remaining_mins:02}:{remaining_secs:02}");
                    ui.label(egui::RichText::new(&label_text).size(18.0));
                    let progress = 1 as f32 - ((remaining as f32 / self.time as f32 * 100 as f32) / 100 as f32);
                    let bar = ProgressBar::new(progress)
                        .fill(Color32::from_hex("#cba6f7").expect("Failed to convert hex code"));
                    ui.add(bar);

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
