use std::io::Error;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

use crate::errors::ParseError;
const CR: u8 = 13;
const LF: u8 = 10;
const META_LENGTH: usize = 4;
const CRLF: &str = "\r\n";

const PING: &str = "*1\r\n$4\r\nPING\r\n";
const ARRAY_MARKER: &str = "*";

pub struct RESP;

fn byte_to_char(byte: u8) -> char {
    byte as char
}

fn char_to_number(ch: char) -> Result<usize, ParseError> {
    match (ch.to_string()).parse::<usize>() {
        Ok(len) => Ok(len),
        Err(e) => Err(ParseError::ParseInt(e)),
    }
}

fn get_nth_byte(response: &[u8], nth: usize) -> Result<u8, ParseError> {
    response.get(nth).cloned().ok_or(ParseError::Empty)
}

fn bytes_to_string(response: Vec<u8>) -> Result<String, ParseError> {
    match String::from_utf8(response) {
        Ok(val) => Ok(val),
        Err(e) => Err(ParseError::ParseStr(e)),
    }
}

fn handle_bulk_string(response: Vec<u8>) -> Result<String, ParseError> {
    let data_length = char_to_number(byte_to_char(get_nth_byte(&response, 1)?))?;
    let response_data = &response[META_LENGTH..data_length + META_LENGTH];
    bytes_to_string(response_data.to_vec())
}

impl RESP {
    pub fn make_array(args: Vec<&str>) -> String {
        let initial = format!("{}{}{}", ARRAY_MARKER, args.len(), CRLF);
        args.iter().fold(initial, |acc, elem| {
            format!("{}${}{}{}{}", acc, elem.len(), CRLF, elem, CRLF)
        })
    }
    pub fn parse(response: Vec<u8>) -> Result<String, ParseError> {
        let first_byte = get_nth_byte(&response, 0)?;
        match first_byte {
            36 /*$*/ => handle_bulk_string(response),
            _ => Err(ParseError::Empty),
        }
    }
}
