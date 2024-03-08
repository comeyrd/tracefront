use eframe::egui::{ self };
mod trace_pannels;
mod miti_ws;
use trace_pannels::TracePannels;
use self::miti_ws::MitiTrace;
use self::miti_ws::MitiWs;

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
                if ui.button("Connect to server").clicked() {
                    self.mitiws = MitiWs::new(self.url.clone(), ctx.clone());
                }
                match &mut self.mitiws {
                    Some(mws) => {
                        match mws.is_connected() {
                            Some(is_conn) => {
                                if is_conn {
                                    ui.label("✔");
                                    ui.separator();
                                    if ui.button("send").clicked() {
                                        match &mut self.mitiws {
                                            Some(conn) => { conn.send(&MitiTrace::default()) }
                                            None => {}
                                        }
                                    }
                                } else {
                                    ui.spinner();
                                }
                            }
                            None => {
                                self.mitiws = None;
                                ui.label("🚫");
                            }
                        }
                    }
                    None => {
                        ui.label("🚫");
                    }
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |_| {});
        self.pannels.ui(ctx);

        match &mut self.mitiws {
            Some(conn) => {
                match &mut conn.try_receive() {
                    Some(t) => {
                        self.mtrace = std::rc::Rc::new(t.clone());
                        self.pannels.update_trace(self.mtrace.clone());
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}
