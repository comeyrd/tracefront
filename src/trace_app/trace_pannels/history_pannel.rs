use eframe::egui::{self};

use crate::trace_app::miti_ws::MitiTrace;

pub struct HistoryPannel {
    mtrace: std::rc::Rc<MitiTrace>,
    history: Vec<std::rc::Rc<MitiTrace>>,
}

impl Default for HistoryPannel {
    fn default() -> Self {
        Self {
            mtrace: std::rc::Rc::new(MitiTrace::default()),
            history: Vec::new(),
        }
    }
}
impl HistoryPannel {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
        if ui.button("Clear History").clicked() {
            self.history.clear();
        }
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.with_layout(
                    egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                    |ui| {
                        for trace in &self.history {
                            ui.label(format!(
                                "{{ {:}, {:}, {:}, {:}}}",
                                trace.direction, trace.rate, trace.roaming, trace.text
                            ));
                        }
                    },
                );
            });
    }
}

impl super::TracePannel for HistoryPannel {
    fn name(&self) -> &'static str {
        "History Pannel"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use super::View as _;
        egui::Window::new(self.name())
            .open(open)
            .show(ctx, |ui| self.ui(ui));
    }
    fn update_trace(&mut self, miti_trace: std::rc::Rc<MitiTrace>) {
        self.history.push(miti_trace.clone());
        self.mtrace = miti_trace;
    }
}

impl super::View for HistoryPannel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        self.ui_control(ui);
        ui.separator();
        self.ui_content(ui);
    }
}
