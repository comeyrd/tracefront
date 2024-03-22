use eframe::egui::{self, Color32, TextFormat};

use crate::trace_app::miti_ws::MitiTrace;

pub struct ChannelPannel {
    mtrace: std::rc::Rc<MitiTrace>,
}

impl Default for ChannelPannel {
    fn default() -> Self {
        Self {
            mtrace: std::rc::Rc::new(MitiTrace::default()),
        }
    }
}
impl ChannelPannel {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
        ui.label("Channel Control : ");
    }
    pub fn print_on_grid(&mut self, ui: &mut egui::Ui, label: &str) {
        ui.vertical_centered(|ui| {
            ui.label(label);
        });
    }
    pub fn make_label(&mut self, ui: &mut egui::Ui, label: &str, state: &str, color: &str) {
        use egui::text::LayoutJob;
        let mut job = LayoutJob::default();
        let (default_color, _strong_color) = (Color32::BLACK, Color32::BLACK);
        let background = if label == state {
            match color {
                "red" => Color32::from_rgb(255, 84, 84),
                "blue" => Color32::from_rgb(68, 143, 255),
                "orange" => Color32::from_rgb(255, 181, 68),
                "green" => Color32::from_rgb(90, 235, 100),
                _ => Color32::from_rgb(90, 235, 100),
            }
        } else {
            Color32::from_rgb(255, 255, 255)
        };

        job.append(
            label,
            0.0,
            TextFormat {
                color: default_color,
                background,
                ..Default::default()
            },
        );
        ui.vertical_centered(|ui| {
            ui.label(job);
        });
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) {
        let dark_mode = ui.visuals().dark_mode;
        let faded_color = ui.visuals().window_fill();
        let faded_color = |color: Color32| -> Color32 {
            use egui::Rgba;
            let t = if dark_mode { 0.95 } else { 0.8 };
            egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
        };
        let etat = "truc";
        egui::Grid::new("some_unique_id")
            .max_col_width(70.0)
            .show(ui, |ui| {
                self.make_label(ui, "PCCH", &etat, "red");
                self.make_label(ui, "BCCH", &etat, "red");
                self.make_label(ui, "CCCH", &etat, "green");
                self.make_label(ui, "DCCH", &etat, "red");
                self.make_label(ui, "DTCH", &etat, "red");
                self.make_label(ui, "MCCH", &etat, "red");
                self.make_label(ui, "MTCH", &etat, "red");
                self.print_on_grid(ui, "----");
                self.print_on_grid(ui, "Canaux Logiques");
                self.print_on_grid(ui, "----");
                self.make_label(ui, "CCCH", &etat, "red");
                self.make_label(ui, "DCCH", &etat, "red");
                self.make_label(ui, "DTCH", &etat, "red");
                ui.end_row();

                self.make_label(ui, "PCH", &etat, "red");
                self.make_label(ui, "BCH", &etat, "red");
                self.print_on_grid(ui, "");
                self.print_on_grid(ui, "");
                self.make_label(ui, "DL-SCH", &etat, "red");
                self.print_on_grid(ui, "");
                self.make_label(ui, "MCH", &etat, "red");
                self.print_on_grid(ui, "----");
                self.print_on_grid(ui, "Canaux de Transport");
                self.print_on_grid(ui, "----");
                self.make_label(ui, "RACH", &etat, "blue");
                self.make_label(ui, "UL-SCH", &etat, "blue");
                ui.end_row();

                self.make_label(ui, "PDSCH", &etat, "blue");
                self.make_label(ui, "PBCH", &etat, "orange");
                self.print_on_grid(ui, "");
                self.print_on_grid(ui, "");
                self.make_label(ui, "PDCCH", &etat, "orange");
                self.print_on_grid(ui, "");
                self.make_label(ui, "PMCH", &etat, "orange");
                self.print_on_grid(ui, "----");
                self.print_on_grid(ui, "Canaux Logiques");
                self.print_on_grid(ui, "----");
                self.make_label(ui, "PRACH", &etat, "blue");
                self.make_label(ui, "PUSCH", &etat, "blue");
                self.make_label(ui, "PUCCH", &etat, "orange");
                ui.end_row();

                self.print_on_grid(ui, "----");
                self.print_on_grid(ui, "----");
                self.print_on_grid(ui, "----");
                self.print_on_grid(ui, "Downlink");
                self.print_on_grid(ui, "----");
                self.print_on_grid(ui, "----");
                self.print_on_grid(ui, "----");
                self.print_on_grid(ui, "");
                self.print_on_grid(ui, "Technologie : LTE");
                self.print_on_grid(ui, "");
                self.print_on_grid(ui, "----");
                self.print_on_grid(ui, "Uplink");
                self.print_on_grid(ui, "----");
                ui.end_row();
            });
    }
}

impl super::TracePannel for ChannelPannel {
    fn name(&self) -> &'static str {
        "Channel Pannel"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use super::View as _;
        let size = egui::Vec2::new(700.0, 700.0);
        egui::Window::new(self.name())
            .open(open)
            .fixed_size(size)
            .show(ctx, |ui| self.ui(ui));
    }
    fn update_trace(&mut self, miti_trace: std::rc::Rc<MitiTrace>) {
        self.mtrace = miti_trace;
    }
}

impl super::View for ChannelPannel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        self.ui_control(ui);
        ui.separator();
        self.ui_content(ui);
    }
}
