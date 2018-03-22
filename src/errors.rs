error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        IO(::std::io::Error);
    }

    errors {
        CommandFailed {
            description("Firmata command could not be processed")
        }
    }
}
