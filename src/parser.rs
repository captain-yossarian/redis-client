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
    pub fn parse(response: Vec<u8>) {
        let first_byte = response.first();
        match first_byte {
            Some(byte) => match byte {
                36 => {
                    if let Some(length) = response.get(1) {
                        let ascii = length.to_string();
                        let num = ascii.parse::<usize>();
                        println!("ASCI {}", ascii);

                        match num {
                            Ok(n) => println!("Response {:?}", n),
                            Err(_) => println!("Err"),
                        }
                    }
                }
                _ => println!("Response not an Array"),
            },
            None => println!("Response"),
        }
    }
}
