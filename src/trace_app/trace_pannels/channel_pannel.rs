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

    pub fn make_label(&mut self, ui: &mut egui::Ui, label: &str, state: bool, color: &str) {
        use egui::text::LayoutJob;
        let mut job = LayoutJob::default();
        let (default_color, _strong_color) = (Color32::BLACK, Color32::BLACK);
        let background = if state {
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
        ui.label(job);
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) {
        let dark_mode = ui.visuals().dark_mode;
        let faded_color = ui.visuals().window_fill();
        let faded_color = |color: Color32| -> Color32 {
            use egui::Rgba;
            let t = if dark_mode { 0.95 } else { 0.8 };
            egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
        };

        egui::Grid::new("some_unique_id")
            .max_col_width(120.0)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "CCCH", false, "red");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "DCCH", false, "red");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "DTCH", false, "green");
                });
                ui.vertical_centered(|ui| {
                    ui.label("Logical channels :");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "PCCH", false, "red");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "BCCH", false, "red");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "CCCH", false, "red");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "DCCH", false, "red");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "DTCH", false, "green");
                });

                ui.end_row();

                ui.vertical_centered(|ui| {
                    self.make_label(ui, "RACH", false, "blue");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "DCH", false, "orange");
                });
                ui.label(" ");
                ui.vertical_centered(|ui| {
                    ui.label("Transport channels :");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "PCH", false, "blue");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "BCH", false, "blue");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "FACH", false, "blue");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "DCH", false, "orange");
                });
                ui.end_row();

                ui.vertical_centered(|ui| {
                    self.make_label(ui, "PRACH", false, "blue");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "DPDCH", false, "orange");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "DPCCH", false, "orange");
                });
                ui.vertical_centered(|ui| {
                    ui.label("Physical channels :");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "S-CCPCH", false, "blue");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "P-CCPCH", false, "blue");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "DPDCH", false, "orange");
                });
                ui.vertical_centered(|ui| {
                    self.make_label(ui, "DPCCH", false, "orange");
                });
                ui.end_row();

                ui.label(" ");
                ui.vertical_centered(|ui| {
                    ui.label("UPLINK");
                });
                ui.label(" ");
                ui.label(" ");
                ui.label(" ");
                ui.label(" ");
                ui.vertical_centered(|ui| {
                    ui.label("DOWNLINK");
                });
                ui.end_row();
            });

        /*let size = egui::Vec2::new(100.0, 49.0);
        let aich_on = egui::Image::new(egui::include_image!("../../../assets/AICH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let aich_off = egui::Image::new(egui::include_image!("../../../assets/AICH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let bcch_on = egui::Image::new(egui::include_image!("../../../assets/BCCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let bcch_off = egui::Image::new(egui::include_image!("../../../assets/BCCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let bch_on = egui::Image::new(egui::include_image!("../../../assets/BCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let bch_off = egui::Image::new(egui::include_image!("../../../assets/BCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let ccch_ul_on = egui::Image::new(egui::include_image!("../../../assets/CCCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let ccch_ul_off = egui::Image::new(egui::include_image!("../../../assets/CCCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let ccch_dl_on = egui::Image::new(egui::include_image!("../../../assets/CCCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let ccch_dl_off = egui::Image::new(egui::include_image!("../../../assets/CCCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let cpich_on = egui::Image::new(egui::include_image!("../../../assets/CPICH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let cpich_off = egui::Image::new(egui::include_image!("../../../assets/CPICH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dcch_ul_on = egui::Image::new(egui::include_image!("../../../assets/DCCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dcch_ul_off = egui::Image::new(egui::include_image!("../../../assets/DCCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dcch_dl_on = egui::Image::new(egui::include_image!("../../../assets/DCCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dcch_dl_off = egui::Image::new(egui::include_image!("../../../assets/DCCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dch_ul_on = egui::Image::new(egui::include_image!("../../../assets/DCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dch_ul_off = egui::Image::new(egui::include_image!("../../../assets/DCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dch_dl_on = egui::Image::new(egui::include_image!("../../../assets/DCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dch_dl_off = egui::Image::new(egui::include_image!("../../../assets/DCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dpcch_ul_on = egui::Image::new(egui::include_image!("../../../assets/DPCCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dpcch_ul_off = egui::Image::new(egui::include_image!("../../../assets/DPCCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dpcch_dl_on = egui::Image::new(egui::include_image!("../../../assets/DPCCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dpcch_dl_off = egui::Image::new(egui::include_image!("../../../assets/DPCCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dpdch_ul_on = egui::Image::new(egui::include_image!("../../../assets/DPDCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dpdch_ul_off = egui::Image::new(egui::include_image!("../../../assets/DPDCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dpdch_dl_on = egui::Image::new(egui::include_image!("../../../assets/DPDCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dpdch_dl_off = egui::Image::new(egui::include_image!("../../../assets/DPDCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dtch_ul_on = egui::Image::new(egui::include_image!("../../../assets/DTCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dtch_ul_off = egui::Image::new(egui::include_image!("../../../assets/DTCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dtch_dl_on = egui::Image::new(egui::include_image!("../../../assets/DTCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let dtch_dl_off = egui::Image::new(egui::include_image!("../../../assets/DTCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let fach_on = egui::Image::new(egui::include_image!("../../../assets/FACH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let fach_off = egui::Image::new(egui::include_image!("../../../assets/FACH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let pcch_on = egui::Image::new(egui::include_image!("../../../assets/PCCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let pcch_off = egui::Image::new(egui::include_image!("../../../assets/PCCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let p_ccpch_on = egui::Image::new(egui::include_image!("../../../assets/P-CCPCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let p_ccpch_off = egui::Image::new(egui::include_image!("../../../assets/P-CCPCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let pch_on = egui::Image::new(egui::include_image!("../../../assets/PCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let pch_off = egui::Image::new(egui::include_image!("../../../assets/PCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let pich_on = egui::Image::new(egui::include_image!("../../../assets/PICH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let pich_off = egui::Image::new(egui::include_image!("../../../assets/PICH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let prach_on = egui::Image::new(egui::include_image!("../../../assets/PRACH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let prach_off = egui::Image::new(egui::include_image!("../../../assets/PRACH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let rach_on = egui::Image::new(egui::include_image!("../../../assets/RACH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let rach_off = egui::Image::new(egui::include_image!("../../../assets/RACH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let s_ccpch_on = egui::Image::new(egui::include_image!("../../../assets/S-CCPCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let s_ccpch_off = egui::Image::new(egui::include_image!("../../../assets/S-CCPCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let sch_on = egui::Image::new(egui::include_image!("../../../assets/SCH_ON.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        let sch_off = egui::Image::new(egui::include_image!("../../../assets/SCH_OFF.png"))
            .max_size(size)
            .maintain_aspect_ratio(true);
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.label("Channel type : ");
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.label("UPLINK : ");
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.label("DOWNLINK : ");
            });
        });
        //ui.vertical(|ui| match self.mtrace.direction.as_str() {
        ui.with_layout(
            egui::Layout::left_to_right(egui::Align::TOP),
            |ui| match self.mtrace.direction.as_str() {
                /*"upload" => {
                    ui.add(upblue);
                    ui.add(downblack);
                }
                "download" => {
                    ui.add(upblack);
                    ui.add(downgreen);
                }*/
                _ => {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        //Channel description on the left
                        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                            //logical channels
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                ui.label("Logical channels : ");
                            });
                            //transport channels
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                ui.label("Transport channels : ");
                            });
                            //physical channels
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                ui.label("Physical channels : ");
                            });
                        });
                        //UPLINK in the middle
                        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                            //logical channels
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                ui.add(ccch_ul_off);
                                ui.add(dcch_ul_off);
                                ui.add(dtch_ul_off);
                            });
                            //transport channels
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                ui.add(rach_off);
                                ui.add(dch_ul_off);
                            });
                            //physical channels
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                ui.add(prach_off);
                                ui.add(dpdch_ul_off);
                                ui.add(dpcch_ul_off);
                            });
                        });
                        //DOWNLINK on the right
                        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                            //logical channels
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                ui.add(pcch_off);
                                ui.add(bcch_off);
                                ui.add(ccch_dl_off);
                                ui.add(dcch_dl_off);
                                ui.add(dtch_dl_off);
                            });
                            //transport channels
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                ui.add(pch_off);
                                ui.add(bch_off);
                                ui.add(fach_off);
                                ui.add(dch_dl_off);
                            });
                            //physical channels
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                ui.add(s_ccpch_off);
                                ui.add(p_ccpch_off);
                                ui.add(dpdch_dl_off);
                                ui.add(dpcch_dl_off);
                            });
                        });
                    });
                }
            },
        );*/
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
