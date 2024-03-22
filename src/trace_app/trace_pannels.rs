use std::collections::BTreeSet;

use eframe::egui::{ self };

use super::miti_ws::MitiTrace;
mod base_pannel;
mod history_pannel;
mod link_pannel;
mod rate_pannel;
mod channel_pannel;
//Import new pannel
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait TracePannel {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
    fn update_trace(&mut self, mtrace: std::rc::Rc<MitiTrace>);
}

pub struct TracePannels {
    pannels: Vec<Box<dyn TracePannel>>,
    open: std::collections::BTreeSet<String>,
    mtrace: std::rc::Rc<MitiTrace>,
}

impl Default for TracePannels {
    fn default() -> Self {
        //How to add pannel
        Self::from_pannels(
            vec![
                Box::<link_pannel::LinkPannel>::default(),
                Box::<rate_pannel::RatePannel>::default(),
                Box::<base_pannel::BasePannel>::default(),
                Box::<channel_pannel::ChannelPannel>::default(),
                Box::<history_pannel::HistoryPannel>::default()
            ]
        )
    }
}

impl TracePannels {
    pub fn from_pannels(pannels: Vec<Box<dyn TracePannel>>) -> Self {
        let mut open: BTreeSet<String> = BTreeSet::new();
        for pannel in &pannels {
            open.insert(pannel.name().to_owned());
        }
        Self {
            pannels,
            open,
            mtrace: std::rc::Rc::new(MitiTrace::default()),
        }
    }
    pub fn windows(&mut self, ctx: &egui::Context) {
        let Self { pannels, open, .. } = self;
        for pannel in pannels {
            let mut is_open = open.contains(pannel.name());
            pannel.show(ctx, &mut is_open);
            set_open(open, pannel.name(), is_open);
        }
    }

    pub fn menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Pannels", |ui| {
            for pannel in &self.pannels {
                let is_open = self.open.contains(pannel.name());
                if
                    ui
                        .button(
                            (if is_open { "Close " } else { "Open " }).to_owned() + pannel.name()
                        )
                        .clicked()
                {
                    set_open(&mut self.open, pannel.name(), !is_open);
                }
            }
        });
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        self.windows(ctx);
    }
    pub fn update_trace(&mut self, miti_trace: std::rc::Rc<MitiTrace>) {
        self.mtrace = miti_trace;
        for pannel in &mut self.pannels {
            pannel.update_trace(self.mtrace.clone());
        }
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}
