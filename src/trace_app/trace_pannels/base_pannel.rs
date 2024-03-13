use eframe::egui::{ self };

use crate::trace_app::miti_ws::MitiTrace;

pub struct BasePannel {
    mtrace: std::rc::Rc<MitiTrace>,
}

impl Default for BasePannel {
    fn default() -> Self {
        Self {
            mtrace: std::rc::Rc::new(MitiTrace::default()),
        }
    }
}
impl BasePannel {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
        ui.label("Base Pannel Control Part");
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) {
        ui.label("Base Pannel Content Part");
    }
}

impl super::TracePannel for BasePannel {
    fn name(&self) -> &'static str {
        "Base Pannel"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use super::View as _;
        egui::Window
            ::new(self.name())
            .open(open)
            .show(ctx, |ui| self.ui(ui));
    }
    fn update_trace(&mut self, miti_trace: std::rc::Rc<MitiTrace>) {
        self.mtrace = miti_trace;
    }
}

impl super::View for BasePannel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        self.ui_control(ui);
        ui.separator();
        self.ui_content(ui);
    }
}
