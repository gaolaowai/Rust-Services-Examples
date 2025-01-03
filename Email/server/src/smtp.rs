use std::io::{BufRead, BufReader, Read};

//
//
//
//
#[derive(Default, Debug, Clone)]
struct SMTPObject {
    to: Vec<String>,
    from: String,
    body: String,
    attachments: Vec<Vec<u8>>
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum SMTPState {
    #[default] StartingState,
    UNSUPPORTED,
    DATA, // Respond with 354 if ok
    BDAT, // BDAT 67 LAST
    NOOP, // respond with 250 if ok
    HELP,
    HELO, // plaintext interactions
    EHLO, // Server returns authentication options
    MAILFROM,
    RCPTTO,
    RSET,
    QUIT,
    AUTH,
    STARTTLS,
    ATRN,
    VRFY, // verify existence of user account/mailbox
    EXPN,  // verify distribution list
    AUTHLOGIN // https://mailtrap.io/blog/smtp-auth/
}

#[derive(Default, Debug, Clone)]
pub struct SMTPStateMachine {
    input_buffer: Vec<u8>, // This approach may be inefficient (taking data copied out of pipe), but allows for testing.
    pub output_buffer: Vec<u8>, // We then need to write this to stream
    mail_object: SMTPObject,
    pub current_state: SMTPState,
    pub in_message: bool
}

impl SMTPStateMachine {
    pub fn load_buffer(
        &mut self, 
        input_bytes: &[u8]
    ) -> Result<(),StateError> {
        for some_bytes in input_bytes {
            self.input_buffer.push(*some_bytes);
        }

        Ok(())
    }

    //
    // Admittedly, this is a more complex approach, but it does set us up for being able to actually test our
    // functions in an offline manner and allow the processing of SMTP state to be separate from the
    // transport medium being used.
    //
    pub fn check_for_state_change(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let received = std::str::from_utf8(&self.input_buffer).unwrap_or("");
        let mut reader = BufReader::new(received.as_bytes());
        for line_result in reader.lines() {
            let line = line_result?;
            if !self.in_message {
                println!("C: {}", line);

                let command = line.to_uppercase();
                let parts: Vec<&str> = command.split_whitespace().collect();
                let previous_state = self.current_state.clone();
                match parts[0] {
                    // Supporting original SMTP commands to begin with.
                    // TODO: update to support ESMTP ELHO start, including auth
                    //
                    "HELO" => { self.current_state = SMTPState::HELO; },
                    "MAIL" => { self.current_state = SMTPState::MAILFROM; },
                    "RCPT" => { self.current_state = SMTPState::RCPTTO; },
                    "DATA" => { self.current_state = SMTPState::DATA; },
                    "RSET" => { self.current_state = SMTPState::RSET; },
                    "NOOP" => { self.current_state = SMTPState::NOOP; },
                    "QUIT" => { self.current_state = SMTPState::QUIT; },
                    _ => { self.current_state = SMTPState::UNSUPPORTED; },
                }
                println!("previous state: {:?}, current state: {:?}", previous_state, self.current_state);
            } else {
                println!("Still in DATA state.");
            }
        }

        Ok(())
    }

    pub fn handle_current_state(&mut self) -> Result<Option<SMTPState>, Box<dyn std::error::Error>> {
        let received = std::str::from_utf8(&self.input_buffer).unwrap_or("");
        let mut reader = BufReader::new(received.as_bytes());
        for line_result in reader.lines() {
            let line = line_result?;
            if !self.in_message {
                println!("C: {}", line);

                let parts: Vec<&str> = line.split_whitespace().collect();
                println!("parts: {:?}", parts);

                match self.current_state {
                    SMTPState::HELO => { self.output_buffer = b"250 Ok\r\n".to_vec(); },
                    SMTPState::MAILFROM => { self.output_buffer = b"250 Ok\r\n".to_vec(); self.mail_object.from = parts[1].to_string();},
                    SMTPState::RCPTTO => { self.output_buffer = b"250 Ok recipient\r\n".to_vec(); self.mail_object.to.push(parts[1].to_string()); },
                    SMTPState::DATA => { self.output_buffer = b"354 Continue\r\n".to_vec(); self.in_message = true;},
                    SMTPState::RSET => { self.output_buffer = b"250 Ok reset\r\n".to_vec(); self.mail_object = SMTPObject::default(); },
                    SMTPState::NOOP => { self.output_buffer = b"250 Ok noop\r\n".to_vec(); },
                    SMTPState::QUIT => { self.output_buffer = b"221 Ok\r\n".to_vec(); return Ok(Some(SMTPState::QUIT)); },
                    _ => { self.output_buffer = b"502 Command Not Implemented\r\n".to_vec(); },
                }
            } else {
                // We're still in the middle of a DATA command and reading in bytes.
                println!("C: {}", line);
                if line == "." {
                    self.output_buffer = b"250 Ok\r\n".to_vec();
                    // stream.write_all(bufferout.as_bytes())?;
                    self.in_message = false;
                    println!("EMAIL TO SEND:\n\n{:?}\n\n", self.mail_object);
                } else {
                    self.mail_object.body.push_str(&std::str::from_utf8(&self.input_buffer).unwrap());
                }
            }
        }
        self.input_buffer.clear();

        Ok(None)

    }

}


pub enum StateError {

}

