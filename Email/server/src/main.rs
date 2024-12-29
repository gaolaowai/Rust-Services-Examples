use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead};
use std::thread;
use std::time::Duration;
use std::str;

use server::smtp;

const PORT: &str = "2525";
const DOMAIN: &str = "test.ourdomain.org";
const BACKLOG_MAX: usize = 10;
const BUF_SIZE: usize = 4096;

fn handle_smtp(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Starting thread for connection from {}", stream.peer_addr()?);

    let mut buffer = [0; BUF_SIZE];
    let mut inmessage = false;

    let mut bufferout = format!("220 {} SMTP CCSMTP\r\n", DOMAIN);
    print!("{}", bufferout);
    stream.write_all(bufferout.as_bytes())?;
    let mut smtp_object = smtp::SMTPStateMachine::default();
    smtp_object.in_message = false;

    loop {
        stream.set_read_timeout(Some(Duration::from_secs(120)))?;

        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            println!("Remote host closed socket");
            break;
        }

        let _result = smtp_object.load_buffer(&buffer[..bytes_read]);

        let _result = smtp_object
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT))?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_smtp(stream) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}