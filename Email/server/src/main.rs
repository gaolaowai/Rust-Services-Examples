use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead};
use std::thread;
use std::time::Duration;
use std::str;

use server::smtp::{self, SMTPState};

const PORT: &str = "2525";
const DOMAIN: &str = "test.ourdomain.org";
const BACKLOG_MAX: usize = 10;
const BUF_SIZE: usize = 4096;

fn handle_smtp(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Starting thread for connection from {}", stream.peer_addr()?);

    let mut buffer = [0; BUF_SIZE];
    let mut inmessage = false;

    let mut bufferout = format!("220 {} SMTP\r\n", DOMAIN);
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

        let _load_result = smtp_object.load_buffer(&buffer[..bytes_read]);

        let _state_result = smtp_object.check_for_state_change();

        let handling_result = smtp_object.handle_current_state();

        stream.write_all(&smtp_object.output_buffer).unwrap();

        if let Some(action_state) = handling_result.unwrap() {
            if action_state == SMTPState::QUIT {
                stream.flush();
                stream.shutdown(std::net::Shutdown::Both);
                break;
            }
        }
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