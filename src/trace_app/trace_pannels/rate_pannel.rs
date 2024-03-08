use eframe::egui::{ self };

use crate::trace_app::miti_ws::MitiTrace;

pub struct RatePannel {
    mtrace: std::rc::Rc<MitiTrace>,
}

impl Default for RatePannel {
    fn default() -> Self {
        Self {
            mtrace: std::rc::Rc::new(MitiTrace::default()),
        }
    }
}
impl RatePannel {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
        ui.label("Rate Control");
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) {
        let progress = (self.mtrace.rate as f32) / (std::u16::MAX as f32);
        let progress_bar = egui::ProgressBar::new(progress).show_percentage();
        ui.add(progress_bar);
    }
}

impl super::TracePannel for RatePannel {
    fn name(&self) -> &'static str {
        "Rate Pannel"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use super::View as _;
        egui::Window
            ::new(self.name())
            .open(open)
            .vscroll(false)
            .resizable(false)
            .default_size([300.0, 350.0])
            .show(ctx, |ui: &mut egui::Ui| self.ui(ui));
    }
    fn update_trace(&mut self, miti_trace: std::rc::Rc<MitiTrace>) {
        self.mtrace = miti_trace;
    }
}

impl super::View for RatePannel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        self.ui_control(ui);
        ui.separator();
        self.ui_content(ui);
    }
}
