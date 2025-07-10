#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BgpState {
    Idle,
    Connect,
    Active,
    OpenSent,
    OpenConfirm,
    Established,
}

#[derive(Debug, Clone)]
pub enum BgpEvent {
    ManualStart,
    TcpConnectionSuccess,
    TcpConnectionFail,
    BgpOpenReceived,
    KeepaliveReceived,
    HoldTimerExpired,
    NotificationReceived,
    ManualStop,
}

impl BgpState {
    pub fn on_event(self, event: BgpEvent) -> BgpState {
        use BgpEvent::*;
        use BgpState::*;

        match (self, event) {
            (Idle, ManualStart) => Connect,
            (Connect, TcpConnectionSuccess) => OpenSent,
            (Connect, TcpConnectionFail) => Active,
            (Active, TcpConnectionSuccess) => OpenSent,
            (Active, TcpConnectionFail) => Idle,
            (OpenSent, BgpOpenReceived) => OpenConfirm,
            (OpenConfirm, KeepaliveReceived) => Established,
            (_, ManualStop) => Idle,
            (_, HoldTimerExpired) => Idle,
            (_, NotificationReceived) => Idle,
            _ => self,
        }
    }
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

    pub fn handle_event(&mut self, event: BgpEvent) {
        let old = self.state;
        let new = self.state.on_event(event.clone());
        tracing::info!("FSM: {:?} -> {:?} on {:?}", old, self.state, event);
        self.state = new;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fsm_path_to_established() {
        let mut state = BgpState::Idle;

        let events = vec![
            BgpEvent::ManualStart,
            BgpEvent::TcpConnectionSuccess,
            BgpEvent::BgpOpenReceived,
            BgpEvent::KeepaliveReceived,
        ];

        for e in events {
            state = state.on_event(e);
        }

        assert_eq!(state, BgpState::Established);
    }
}
