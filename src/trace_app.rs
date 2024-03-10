use eframe::egui::{self};
mod miti_ws;
mod trace_pannels;
use self::miti_ws::MitiTrace;
use self::miti_ws::MitiWs;
use trace_pannels::TracePannels;

pub struct TraceFront {
    url: String,
    pannels: TracePannels,
    mitiws: Option<MitiWs>,
    mtrace: std::rc::Rc<MitiTrace>,
}

const DEFAULT_URL: &str = "ws://ws.ceyraud.com:80";
impl Default for TraceFront {
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(), //On defini l'url du serveur ws ici, on peut le modifier ici, ou faire un input text sur le front pour le modifier
            pannels: Default::default(),
            mitiws: None,
            mtrace: std::rc::Rc::new(MitiTrace::default()),
        }
    }
}

impl eframe::App for TraceFront {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //Menu
        egui_extras::install_image_loaders(ctx);

        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            self.pannels.menu(ui);
        });

        egui::TopBottomPanel::top("server").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.url));
                match &mut self.mitiws {
                    Some(mws) => {
                        if mws.is_connected() {
                            ui.label("Connected");
                            ui.separator();
                            if ui.button("send").clicked() {
                                match &mut self.mitiws {
                                    Some(conn) => conn.send(&MitiTrace::default()),
                                    None => {}
                                }
                            }
                        } else {
                            if ui.button("Connect to server").clicked() {
                                self.mitiws = MitiWs::new(self.url.clone(), ctx.clone());
                            }
                        }
                    }
                    None => {
                        if ui.button("Connect to server").clicked() {
                            self.mitiws = MitiWs::new(self.url.clone(), ctx.clone());
                        }
                    }
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |_| {});
        self.pannels.ui(ctx);

        match &mut self.mitiws {
            Some(conn) => {
                match conn.receive() {
                    Some(trace) => {
                        self.mtrace = std::rc::Rc::new(trace.clone());
                        eprint!("{:?}", trace);
                        self.pannels.update_trace(self.mtrace.clone());
                    }
                    None => {}
                };
            }
            None => {}
        }
    }
}
