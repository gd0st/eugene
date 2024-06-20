use std::collections::HashMap;

use encoding::Encoding;

use crate::parsers::parse_str;
pub mod encoding {
    pub enum Encoding {
        SBE,
        TagValue,
    }
}
pub struct FixTag(u32, String);

impl FixTag {
    pub fn new(tag: u32, value: &str) -> Self {
        Self(tag, value.to_string())
    }
}

pub struct FixMessage {
    tags: Vec<FixTag>,
    encoding: Encoding,
}

impl From<&str> for FixMessage {
    fn from(value: &str) -> Self {
        let ifs = '|';
        let message = parse_str(value, ifs);
        todo!()
    }
}

impl FixMessage {
    pub fn new(tags: Option<Vec<FixTag>>, encoding: Encoding) -> Self {
        Self {
            encoding,
            tags: if let Some(tags) = tags { tags } else { vec![] },
        }
    }

    pub fn len(&self) -> usize {
        self.tags.len()
    }
}

impl From<(Vec<FixTag>, Encoding)> for FixMessage {
    fn from(value: (Vec<FixTag>, Encoding)) -> Self {
        FixMessage::new(Some(value.0), value.1)
    }
}

pub mod session {

    use super::*;
    use std::net::Ipv4Addr;
    use std::net::TcpListener;
    use std::net::TcpStream;

    use crate::parsers::parse_str;

    pub trait Session {
        fn heartbeat() -> FixMessage;
        fn test() -> FixMessage;
        fn resend(&self) -> FixMessage;
        fn reject() -> FixMessage;
        fn reset() -> FixMessage;
        fn logout(&self) -> FixMessage;
        fn logon(&self) -> FixMessage;
    }

    pub fn connect(addr: Ipv4Addr, port: usize) -> TcpStream {
        let addr = format!("{}:{}", addr, port);
        TcpStream::connect(addr).unwrap()
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use std::io::BufReader;
        use std::io::BufWriter;
        use std::io::Read;
        use std::io::Write;
        use std::net::TcpListener;
        use std::os::unix::net::SocketAddr;
        use std::thread;

        #[test]
        fn make_send_decode() {
            let server = TcpListener::bind("127.0.0.1:0").unwrap();

            let addr = server.local_addr().unwrap();

            let ifs = '|';
            let fix_message = "8=FIX.4.2|9=178|33=8";

            thread::spawn(move || {
                let connection = server.accept().unwrap();
                let mut message = BufWriter::new(connection.0);

                let buff = String::from(fix_message);

                let _ = message.write(buff.as_bytes()).unwrap();
            });

            if let Ok(client) = TcpStream::connect(addr) {
                let mut buff = String::new();
                let mut reader = BufReader::new(client);
                let _ = reader.read_to_string(&mut buff).unwrap();
                let fix_message = parse_str(buff.as_str(), ifs);
                assert_eq!(fix_message.len(), 3);
            } else {
                println!("FAIL!")
            }
        }
    }
}

struct Dictionary {
    legend: HashMap<usize, String>,
}

#[cfg(test)]
mod test {
    use super::*;
    fn make_fix_message() {
        let tags = vec![FixTag::new(35, "foobar")];
        let _ = FixMessage::new(Some(tags), Encoding::SBE);
    }
}
