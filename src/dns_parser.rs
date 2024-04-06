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
                Some(parsed_str) if !parsed_str.is_empty() => Some(parsed_str),
                _ => None,
            })
            .collect();

        dns_records
    }

    // TODO: zero copy parsing?
    // https://itnext.io/rust-the-joy-of-safe-zero-copy-parsers-8c8581db8ab2
    fn parse_packet(packet_payload: &[u8]) -> Option<String> {
        if packet_payload.len() <= 12 {
            return None;
        }

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
                std::str::from_utf8(&question_section[curr_dilimiter + 1..next_dilimiter]).ok()?;

            result.push(segment);

            curr_dilimiter = next_dilimiter;
            next_dilimiter += question_section[next_dilimiter] as usize + 1;
        }

        Some(result.join("."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packets() {
        let payload = vec![
            190, 181, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5, 114, 101, 97, 108, 109, 7, 109, 111, 110,
            103, 111, 100, 98, 3, 99, 111, 109, 0, 0, 1, 0, 1, 30, 222, 189, 1, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 5, 114, 101, 97, 108, 109, 7, 109, 111, 110, 103, 111, 100, 98, 3, 99, 111,
            109, 0, 0, 65, 0, 1, 30, 190, 181, 129, 128, 0, 1, 0, 6, 0, 0, 0, 0, 5, 114, 101, 97,
            108, 109, 7, 109, 111, 110, 103, 111, 100, 98, 3, 99, 111, 109, 0, 0, 1, 0, 1, 192, 12,
            0, 5, 0, 1, 0, 0, 1, 44, 0, 13, 6, 103, 108, 111, 98, 97, 108, 3, 97, 119, 115, 192,
            12, 192, 47, 0, 5, 0, 1, 0, 0, 1, 44, 0, 28, 6, 103, 108, 111, 98, 97, 108, 4, 108, 98,
            45, 98, 3, 114, 53, 51, 3, 97, 119, 115, 5, 99, 108, 111, 117, 100, 192, 18, 192, 72,
            0, 5, 0, 1, 0, 0, 0, 60, 0, 12, 9, 101, 117, 45, 119, 101, 115, 116, 45, 50, 192, 79,
            192, 112, 0, 1, 0, 1, 0, 0, 0, 60, 0, 4, 18, 132, 19, 44, 192, 112, 0, 1, 0, 1, 0, 0,
            0, 60, 0, 4, 35, 177, 238, 136, 192, 112, 0, 1, 0, 1, 0, 0, 0, 60, 0, 4, 3, 9, 230,
            118, 30, 222, 189, 129, 128, 0, 1, 0, 3, 0, 1, 0, 0, 5, 114, 101, 97, 108, 109, 7, 109,
            111, 110, 103, 111, 100, 98, 3, 99, 111, 109, 0, 0, 65, 0, 1, 192, 12, 0, 5, 0, 1, 0,
            0, 1, 44, 0, 13, 6, 103, 108, 111, 98, 97, 108, 3, 97, 119, 115, 192, 12, 192, 47, 0,
            5, 0, 1, 0, 0, 1, 44, 0, 28, 6, 103, 108, 111, 98, 97, 108, 4, 108, 98, 45, 98, 3, 114,
            53, 51, 3, 97, 119, 115, 5, 99, 108, 111, 117, 100, 192, 18, 192, 72, 0, 5, 0, 1, 0, 0,
            0, 60, 0, 12, 9, 101, 117, 45, 119, 101, 115, 116, 45, 50, 192, 79, 192, 84, 0, 6, 0,
            1, 0, 0, 2, 104, 0, 69, 6, 110, 115, 45, 54, 57, 53, 9, 97, 119, 115, 100, 110, 115,
            45, 50, 50, 3, 110, 101, 116, 0, 17, 97, 119, 115, 100, 110, 115, 45, 104, 111, 115,
            116, 109, 97, 115, 116, 101, 114, 6, 97, 109, 97, 122, 111, 110, 192, 26, 0, 0, 0, 1,
            0, 0, 28, 32, 0, 0, 3, 132, 0, 18, 117, 0, 0, 1, 81, 128, 30, 23, 222, 1, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 6, 101, 120, 116, 101, 110, 100, 8, 118, 105, 109, 101, 111, 99, 100,
            110, 3, 99, 111, 109, 0, 0, 1, 0, 1, 30, 37, 28, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 6, 101,
            120, 116, 101, 110, 100, 8, 118, 105, 109, 101, 111, 99, 100, 110, 3, 99, 111, 109, 0,
            0, 65, 0, 1,
        ];

        let result = DNSParser::parse_packets(payload);

        dbg!(result);
    }

    #[test]
    fn test_parse_packet() {
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
