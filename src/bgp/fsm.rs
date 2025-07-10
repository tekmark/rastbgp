// src/bgp/fsm.rs

#[derive(Debug, Clone, Copy)]
pub enum BgpState {
    Idle,
    Connect,
    Active,
    OpenSent,
    OpenConfirm,
    Established,
}

pub struct Fsm {
    state: BgpState,
}

impl Fsm {
    pub fn new() -> Self {
        Fsm {
            state: BgpState::Idle,
        }
    }

    pub fn state(&self) -> BgpState {
        self.state
    }

    pub fn handle_event(&mut self, event: &str) {
        // TODO: implement BGP FSM transitions based on events
        println!("Handling event: {}", event);
    }
}
