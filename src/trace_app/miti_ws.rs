use ewebsock::{ WsEvent, WsMessage, WsReceiver, WsSender };
use serde::{ Deserialize, Serialize };

pub struct MitiWs {
    ws_sender: WsSender,
    ws_receiver: WsReceiver,
    connected: bool,
    error: bool,
    errorstr: String,
}

impl MitiWs {
    pub fn new(url: String, ctx: eframe::egui::Context) -> Option<Self> {
        let wakeup = move || ctx.request_repaint();
        match ewebsock::connect_with_wakeup(url, wakeup) {
            Ok((wsender, wreceiver)) => {
                let mws = Self {
                    ws_sender: wsender,
                    ws_receiver: wreceiver,
                    connected: false,
                    error: false,
                    errorstr: "".to_owned(),
                };
                return Some(mws);
            }
            Err(_) => {
                return None;
            }
        }
    }

    pub fn receive(&mut self) -> Option<MitiTrace> {
        if let Some(event) = self.ws_receiver.try_recv() {
            match event {
                WsEvent::Message(msg) =>
                    match msg {
                        WsMessage::Text(txt) => {
                            self.error = false;
                            let resp: MitiTrace = serde_json::from_str(&txt).unwrap();
                            return Some(resp);
                        }
                        WsMessage::Unknown(z) => {
                            self.error = true;
                            self.errorstr = z.clone();
                        }
                        WsMessage::Binary(bin) => {
                            self.error = true;
                            self.errorstr = format!("{:?}", bin);
                        }
                        _ => {
                            self.error = true;
                            self.errorstr = "Received Ping-Pong".to_string();
                        }
                    }
                WsEvent::Opened => {
                    self.connected = true;
                }
                WsEvent::Closed => {
                    self.connected = false;
                }
                WsEvent::Error(str) => {
                    self.error = true;
                    self.errorstr = str.clone();
                    self.connected = false;
                }
            }
        }
        None
    }
    pub fn send(&mut self, msg: &MitiTrace) {
        self.ws_sender.send(WsMessage::Text(serde_json::to_string(msg).unwrap()));
    }
    pub fn is_connected(&self) -> bool {
        return self.connected;
    }
    pub fn is_error(&self) -> bool {
        return self.error;
    }
    pub fn error_str(&self) -> String {
        return self.errorstr.clone();
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MitiTrace {
    pub direction: String,
    pub rate: u16,
    pub text: char,
    pub roaming: bool,
}

impl Default for MitiTrace {
    fn default() -> Self {
        Self {
            direction: "test".to_owned(),
            rate: 12,
            text: 'a',
            roaming: false,
        }
    }
}
