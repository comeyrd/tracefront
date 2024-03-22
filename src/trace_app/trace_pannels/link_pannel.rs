use eframe::egui::{self};

use crate::trace_app::miti_ws::MitiTrace;

pub struct LinkPannel {
    mtrace: std::rc::Rc<MitiTrace>,
}

impl Default for LinkPannel {
    fn default() -> Self {
        Self {
            mtrace: std::rc::Rc::new(MitiTrace::default()),
        }
    }
}
impl LinkPannel {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
        ui.label("Link Control : ");
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) {
        let size = egui::Vec2::new(100.0, 100.0);
        let upblack = egui::Image::new(egui::include_image!("../../../assets/uplink_black.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let downblack =
            egui::Image::new(egui::include_image!("../../../assets/downlink_black.png"))
                .max_size(size)
                .maintain_aspect_ratio(true);
        let downgreen =
            egui::Image::new(egui::include_image!("../../../assets/downlink_green.png"))
                .max_size(size)
                .maintain_aspect_ratio(true);
        let upblue = egui::Image::new(egui::include_image!("../../../assets/uplink_blue.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);

        //ui.vertical(|ui| match self.mtrace.direction.as_str() {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| match self.mtrace.direction.as_str() {
            "upload" => {
                ui.add(upblue);
                ui.add(downblack);
            }
            "download" => {
                ui.add(upblack);
                ui.add(downgreen);
            }
            _ => {
                ui.add(upblack);
                ui.add(downblack);
            }
        });
    }
}

impl super::TracePannel for LinkPannel {
    fn name(&self) -> &'static str {
        "Link Pannel"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use super::View as _;
        let size = egui::Vec2::new(215.0, 200.0);
        egui::Window::new(self.name())
            .open(open)
            .fixed_size(size)
            .show(ctx, |ui| self.ui(ui));
    }
    fn update_trace(&mut self, miti_trace: std::rc::Rc<MitiTrace>) {
        self.mtrace = miti_trace;
    }
}

impl super::View for LinkPannel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        self.ui_control(ui);
        ui.separator();
        self.ui_content(ui);
    }
}