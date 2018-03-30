error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        IO(::std::io::Error);
    }

    errors {
        UnreadableMsg {
            description("Incomplete or interrupted Firmata message")
        }
        CommandFailed {
            description("Firmata command could not be processed")
        }
        UnexpectedResponse {
            description("Unexpected Firmata response")
        }
    }
}
