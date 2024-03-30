use anyhow::Result;

// TODO: avoid constants declaration duplication
pub const UDP_MESSAGE_BYTES_DELIMITER: u8 = 0x1E;

pub struct DNSParser;

impl DNSParser {
    pub fn parse_packets(bytes: Vec<u8>) -> Vec<String> {
        let packets: Vec<&[u8]> = bytes
            .split(|&byte| byte == UDP_MESSAGE_BYTES_DELIMITER)
            .collect();

        let dns_records: Vec<String> = packets
            .iter()
            .filter_map(|&packet| match Self::parse_packet(packet) {
                Ok(parsed_str) if !parsed_str.is_empty() => Some(parsed_str),
                _ => None,
            })
            .collect();

        dns_records
    }

    // TODO: zero copy parsing?
    // https://itnext.io/rust-the-joy-of-safe-zero-copy-parsers-8c8581db8ab2
    fn parse_packet(packet_payload: &[u8]) -> Result<String> {
        let question_section: &[u8] = &packet_payload[12..];

        // two pointers algorithm
        let mut curr_dilimiter = 0;
        let mut next_dilimiter = question_section[curr_dilimiter] as usize + 1;

        let mut is_parsed = false;

        // TODO: is it possible to avoid using heap allocation?
        let mut result: Vec<&str> = vec![];

        while !is_parsed && next_dilimiter < question_section.len() {
            if question_section[next_dilimiter] == 0 {
                is_parsed = true;
            }

            let segment =
                std::str::from_utf8(&question_section[curr_dilimiter + 1..next_dilimiter])?;

            result.push(segment);

            curr_dilimiter = next_dilimiter;
            next_dilimiter += question_section[next_dilimiter] as usize + 1;
        }

        Ok(result.join("."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_query_parsing() {
        let payload = [
            116, 100, 129, 128, 0, 1, 0, 0, 0, 1, 0, 0, 4, 112, 108, 97, 121, 6, 103, 111, 111,
            103, 108, 101, 3, 99, 111, 109, 0, 0, 65, 0, 1, 192, 17, 0, 6, 0, 1, 0, 0, 0, 59, 0,
            38, 3, 110, 115, 49, 192, 17, 9, 100, 110, 115, 45, 97, 100, 109, 105, 110, 192, 17,
            36, 225, 222, 251, 0, 0, 3, 132, 0, 0, 3, 132, 0, 0, 7, 8, 0, 0, 0, 60,
        ];

        let parsed_dns = DNSParser::parse_packet(&payload).unwrap();

        assert_eq!(parsed_dns, String::from("play.google.com"));
    }
}
