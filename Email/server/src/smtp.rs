use std::io::Read;

//
//
//
//
#[derive(Default, Debug)]
struct SMTPObject {
    to: String,
    from: Vec<String>,
    body: String,
    attachments: Vec<Vec<u8>>
}

#[derive(Default, Debug)]
enum SMTPState {
    #[default] StartingState,
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

#[derive(Default, Debug)]
struct SMTPStateMachine {
    input_buffer: Vec<u8>, // This approach may be inefficient (taking data copied out of pipe), but allows for testing.
    mail_object: SMTPObject,
    current_state: SMTPState
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

    pub fn check_state(&mut self) {
        if self.current_state == SMTPState::DATA {
            // Check for solo "." character
            if self.input_buffer[0] == b'.' {

            }
        }
    }
}


enum StateError {

}