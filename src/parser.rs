const CRLF: &str = "\r\n";

const PING: &str = "*1\r\n$4\r\nPING\r\n";
const ARRAY_MARKER: &str = "*";

pub struct RESP;

impl RESP {
    pub fn array(args: Vec<&str>) -> String {
        let initial = format!("{}{}{}", ARRAY_MARKER, args.len(), CRLF);
        args.iter().fold(initial, |acc, elem| {
            format!("{}${}{}{}{}", acc, elem.len(), CRLF, elem, CRLF)
        })
    }
}
