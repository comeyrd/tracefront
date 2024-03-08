use core::panic;

use ewebsock::{ WsEvent, WsMessage, WsReceiver, WsSender };
use serde::{ Deserialize, Serialize };

pub struct MitiWs {
    ws_sender: WsSender,
    ws_receiver: WsReceiver,
    connected: bool,
}

impl MitiWs {
    pub fn new(url: String, ctx: eframe::egui::Context) -> Option<Self> {
        let wakeup = move || ctx.request_repaint();
        match ewebsock::connect_with_wakeup(url, wakeup) {
            Ok((wsender, wreceiver)) => {
                let mut mws = Self {
                    ws_sender: wsender,
                    ws_receiver: wreceiver,
                    connected: false,
                };
                let recv: Option<WsEvent> = mws.ws_receiver.try_recv();
                match recv {
                    Some(event) => {
                        match event {
                            WsEvent::Opened => {
                                mws.connected = true;
                            }
                            WsEvent::Error(_) => {
                                return None;
                            }
                            _ => {}
                        }
                    }
                    None => {}
                }
                return Some(mws);
            }
            Err(error) => {
                panic!("{}", error);
            }
        }
    }
    pub fn try_receive(&mut self) -> Option<MitiTrace> {
        let opt_event = self.ws_receiver.try_recv();
        match opt_event {
            Some(event) => {
                match event {
                    WsEvent::Message(msg) => {
                        match msg {
                            WsMessage::Text(txt) => {
                                let resp: MitiTrace = serde_json::from_str(&txt).unwrap();
                                return Some(resp);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            None => {
                return None;
            }
        }
        return None;
    }
    pub fn send(&mut self, msg: &MitiTrace) {
        self.ws_sender.send(WsMessage::Text(serde_json::to_string(msg).unwrap()));
    }
    pub fn is_connected(&mut self) -> Option<bool> {
        if !self.connected {
            let recv: Option<WsEvent> = self.ws_receiver.try_recv();
            match recv {
                Some(event) => {
                    match event {
                        WsEvent::Opened => {
                            self.connected = true;
                        }
                        WsEvent::Error(_) => {
                            return None;
                        }
                        _ => {}
                    }
                }
                None => {}
            }
        }
        return Some(self.connected);
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
