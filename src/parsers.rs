use crate::fix::encoding::Encoding;
use crate::fix::FixMessage;
use crate::fix::FixTag;

enum ParsingError {
    ImproperFormat,
}

pub(crate) fn parse_str(word: &str, ifs: char) -> FixMessage {
    //header

    let message = word.split(ifs);

    let message = word
        .split(ifs)
        .into_iter()
        .map(|a| {
            let pairs: Vec<&str> = a.split("=").into_iter().take(2).collect();
            if pairs.len() != 2 {
                todo!()
            }
            println!("{:?}, {:?}", &pairs, &pairs.get(0));
            let (tag, value) = (pairs.get(0), pairs.get(1));
            if let (Some(tag), Some(value)) = (tag, value) {
                let tag = tag.parse().unwrap();
                return FixTag::new(tag, value);
            }

            panic!("Invalid Format!")
        })
        .collect();

    FixMessage::new(message, Encoding::TagValue)
}

#[cfg(test)]
mod test {
    use super::parse_str;

    #[test]
    fn simple_message() {}
}
