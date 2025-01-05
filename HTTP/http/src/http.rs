//!
//! This module deals with the parsing of HTTP 1.x parts when given an array of bytes.
//! 
struct RequestParser {
    header: Option<Header>,
    body: Option<Body>,
    requested_host: Option<String>,
    requested_uri: String
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

#[derive(PartialEq, Debug)]
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
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}


impl RequestParser {
    fn parse_request_method(&mut self, buffer: &[u8]) -> Option<RequestMethod> {
        let buf = {let mut b = [0u8;3]; b.clone_from_slice(&buffer[..3]); b};
        match &buf {
            b"GET" => Some(RequestMethod::GET),
            b"POS" => Some(RequestMethod::POST),
            b"HEA" => Some(RequestMethod::HEAD),
            b"PUT" => Some(RequestMethod::PUT),
            b"DEL" => Some(RequestMethod::DELETE),
            b"CON" => Some(RequestMethod::CONNECT),
            b"OPT" => Some(RequestMethod::OPTIONS),
            b"TRA" => Some(RequestMethod::TRACE),
            b"PAT" => Some(RequestMethod::PATCH),
            _ => return None
        }
    }
    
    fn parse_request_URI(&self, buffer: &[u8]) -> Option<String> {
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
        Some(parts[1].to_string())
    }
    
    fn parse_request_HTTP_version(&self, buffer: &[u8]) -> Option<HttpVersion>{
        let stringified_buffer = String::from_utf8(buffer.to_vec()).unwrap();   

        let lines = stringified_buffer.split("\r\n").collect::<Vec<&str>>();
        if lines.len() == 0 {
            return None
        }
        let first_line = lines[0];
        let parts = first_line.split(" ").collect::<Vec<&str>>();
        if parts.len() < 3 {
            return None
        }
        match parts[2] {
            "HTTP/1.1" => Some(HttpVersion::V11),
            _ => return None
        }
    }
    
    fn get_header(&self, post_request_form_urlencoded: &[u8]) -> Result<Vec<u8>, String> {
        let upper_bound = post_request_form_urlencoded.len();
        let mut slice_start: usize = 0;
        let mut found = false;
        // scan from first set of bytes
        let is_post = post_request_form_urlencoded.starts_with(b"POST");
        if is_post {
            while slice_start + 3 < upper_bound {
                let current_slice = &post_request_form_urlencoded[slice_start..=slice_start+3];
                let current_slice_string = String::from_utf8(current_slice.clone().to_vec()).unwrap();
                println!("current slice: {:?}  v.s. {:?} --> current_slice_string: {}", current_slice, b"\r\n\r\n", current_slice_string);
                if current_slice.starts_with(b"\r\n\r\n") {
                    found = true;
                    break;
                }
    
                slice_start += 1;
            }
        }
        

        if found && is_post {
            let header = post_request_form_urlencoded[..slice_start].to_vec();
            Ok(header)
        } else {
            Ok(post_request_form_urlencoded[..slice_start].to_vec())
        }
    }
}

//
// Ideally, I would use a proper test block, but at the moment I just want to get everything up and running.
// Each of these input/output pairs will be tested against assert statements. If any fail, the test fails.
// At this stage of development, that's what I need.
//
pub fn test_http_request_parsing() {
    let mut rp = RequestParser{ header: None, body: None, requested_host: None, requested_uri: String::new() };
    
    // 
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/GET
    //
    let get_request = b"GET /contact HTTP/1.1\r\nHost: example.com\r\nUser-Agent: curl/8.6.0\r\nAccept: */*";

        
    let res = rp.parse_request_method(get_request).unwrap();
    assert_eq!(res, RequestMethod::GET);

    let res : String = rp.parse_request_URI(get_request).unwrap();
    assert_eq!("/contact".to_string(), res);

    let res : HttpVersion = rp.parse_request_HTTP_version(get_request).unwrap();
    assert_eq!(HttpVersion::V11, res);


/*
https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST

POST encodings can be:

application/x-www-form-urlencoded
multipart/form-data
text/plain


*/
    let post_request_form_urlencoded = b"POST /test HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: 27\r\n\r\nfield1=value1&field2=value2";

    let header: Vec<u8> = rp.get_header(post_request_form_urlencoded).unwrap();

    println!("Found header:\n{}", String::from_utf8(header).unwrap());

    let post_request_multipart_form_data = b"POST /test HTTP/1.1\r\nHost: example.com\r\nContent-Type: multipart/form-data;boundary=\"delimiter12345\"\r\n\r\n--delimiter12345\r\nContent-Disposition: form-data; name=\"field1\"\r\n\r\nvalue1\r\n--delimiter12345\r\nContent-Disposition: form-data; name=\"field2\"; filename=\"example.txt\"\r\n\r\nvalue2\r\n--delimiter12345--";
    let header: Vec<u8> = rp.get_header(post_request_multipart_form_data).unwrap();
    println!("Found header:\n{}", String::from_utf8(header).unwrap());




    println!("\n\nTests passed!");
}