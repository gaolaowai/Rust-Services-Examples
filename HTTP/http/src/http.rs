//!
//! This module deals with the parsing of HTTP 1.x parts when given an array of bytes.
//! 
struct RequestParser {
    header: Option<Header>,
    body: Option<Body>
}

struct Header {
    request_method: RequestMethod,
    http_version: HttpVersion,
    path: String,
    host: String,
    raw: Vec<u8>
}

struct Body {
    data_type: DataType,
    body_data: Vec<u8>
}


enum DataType {
    TextHtml,
    ApplicationOctetStream,
    ApplicationPdf,
    ApplicationPkcs8,
    ApplicationZip,
    AudioMpeg,
    AudioVorbis
}

enum HttpVersion {
    V11,
    V2,
    V3
}

#[derive(PartialEq, Debug)]
enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
    UPDATE
}


impl RequestParser {
    fn parse_request_method(&mut self, buffer: &[u8]) -> Option<RequestMethod> {
        let stringified_buffer = String::from_utf8(buffer.to_vec()).unwrap();   

        let lines = stringified_buffer.split("\n").collect::<Vec<&str>>();
        if lines.len() == 0 {
            return None
        }
        let first_line = lines[0];
        let parts = first_line.split(" ").collect::<Vec<&str>>();
        if parts.len() < 3 {
            return None
        }
        match parts[0] {
            "GET" => Some(RequestMethod::GET),
            _ => return None
        }
    }
}

pub fn test_http_request_parsing() {
    let mut rp = RequestParser{ header: None, body: None };
    let res = rp.parse_request_method(b"GET /contact HTTP/1.1
Host: example.com
User-Agent: curl/8.6.0
Accept: */*").unwrap();

    assert_eq!(res, RequestMethod::GET);

}