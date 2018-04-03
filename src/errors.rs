error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
        Serial(::serial_core::Error);
    }

    errors {
        UnreadableMsg {
            description("Interrupted or unparseable Firmata message")
        }
        PartialMsg {
            description("Incomplete Firmata message")
        }
        CommandFailed {
            description("Firmata command could not be processed")
        }
        UnexpectedResponse {
            description("Unexpected Firmata response")
        }
        ConnectionClosed {
            description("Serial connection to Firmata device closed")
        }
    }
}
