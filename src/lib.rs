use fix::session;
use gateway::Gateway;
pub mod fix;
mod parsers;
struct iLinkSession(String);

pub struct MarketSegment {
    segment: usize,
    session: iLinkSession,
}

impl MarketSegment {
    fn new(segment: usize, session: iLinkSession) -> Self {
        Self { segment, session }
    }
}

pub type MSGW = MarketSegment;

//impl fix::session::Session for MarketSegment {}

pub struct Convenience {
    session: iLinkSession,
}

impl Convenience {
    fn new(session: iLinkSession) -> Self {
        Self { session }
    }
}
pub type CGW = Convenience;

//impl fix::session::Session for Convenience {}

impl Gateway for MarketSegment {
    fn receive(&self, message: String) {
        todo!()
    }

    fn send(&self) -> String {
        todo!()
    }
}

impl Gateway for Convenience {
    fn receive(&self, message: String) {
        todo!()
    }

    fn send(&self) -> String {
        todo!()
    }
}

mod gateway {
    pub trait Gateway {
        fn receive(&self, message: String);
        fn send(&self) -> String;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
