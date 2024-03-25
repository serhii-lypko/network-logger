use anyhow::Result;

pub struct DNSParser;

// TODO: tests

impl DNSParser {
    // TODO: does need to return Result?
    // TODO: this algorithm has to be very fast and efficient
    // TODO: zero copy parsing?
    pub fn parse_packet(packet_payload: &[u8]) -> Result<String> {
        dbg!(packet_payload);

        // let a = 10;
        // let b = String::from("heyl");

        // let c = a + b;

        return Ok(String::from("mock"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_query_parsing() {
        let payload = [
            176, 118, 129, 128, 0, 1, 0, 4, 0, 0, 0, 0, //
            //
            4, 109, 105, 114, 111, 3, 99, 111, 109, //
            //
            0, 0, 1, 0, 1, 192, 12, 0, 1, 0, 1, 0, 0, 0, 60, 0, 4, 108, 157, 109, 49, 192, 12, 0, 1,
            0, 1, 0, 0, 0, 60, 0, 4, 108, 157, 109, 66, 192, 12, 0, 1, 0, 1, 0, 0, 0, 60, 0, 4,
            108, 157, 109, 17, 192, 12, 0, 1, 0, 1, 0, 0, 0, 60, 0, 4, 108, 157, 109, 78,
        ];

        let sliced = &payload[12..];

        let is_parsed = false;

        let mut curr_len = sliced[0];
        let mut curr_index = sliced[1];

        // TODO: ideally it's better to avoid any heap memory allocation?
        let mut segments: Vec<String> = vec![];
        let mut curr_segment = String::from("");

        // when encounter zero -> stop; else

        while !is_parsed {}

        dbg!(sliced);
        println!("------- ------- -------");
    }
}
