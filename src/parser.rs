pub const START_SYSEX: u8 = 0xF0;
pub const END_SYSEX: u8 = 0xF7;

pub const QUERY_FIRMWARE: u8 = 0x79;

pub const CAPABILITY_QUERY: u8 = 0x6B;
pub const CAPABILITY_RESPONSE: u8 = 0x6c;
pub const CAPABILITY_RESPONSE_SEP: u8 = 0x7F;

#[derive(Debug,PartialEq)]
pub enum PinMode {
    DigitalInput,
    DigitalOutput,
    AnalogInput,
    PWM,
    Other(u8)
}

#[derive(Debug,PartialEq)]
pub struct PinCapability {
    mode: PinMode,
    res: u8
}


#[derive(Debug,PartialEq)]
pub enum FirmataMsg {
    QueryFirmware {
        major: u8,
        minor: u8,
        firmware_name: Vec<u8>
    },
    CapabilityQuery,
    CapabilityResponse(Vec<Vec<PinCapability>>),
}


named!(capability_query<&[u8], FirmataMsg>,
       map!(tag!(&[CAPABILITY_QUERY]), |_| FirmataMsg::CapabilityQuery));


named!(capability_response_entry<&[u8], PinCapability>,
       do_parse!(
           mode: take!(1) >>
           res: take!(1)  >>
           (PinCapability {
               res: res[0],
               mode: match mode[0] {
                   0x00 => PinMode::DigitalInput,
                   0x01 => PinMode::DigitalOutput,
                   0x02 => PinMode::AnalogInput,
                   0x03 => PinMode::PWM,
                   n    => PinMode::Other(n)
               }
           })
        )
);


named!(capability_response_list<&[u8], Vec<PinCapability>>,
       do_parse!(
           pair: many_till!(
               call!(capability_response_entry),
               tag!(&[CAPABILITY_RESPONSE_SEP])
           ) >> (pair.0)
        )
);


named!(capability_response<&[u8], FirmataMsg>,
       do_parse!(
           tag!(&[CAPABILITY_RESPONSE]) >>
           pair: many_till!(
               call!(capability_response_list),
               peek!(tag!(&[END_SYSEX]))
           ) >> (FirmataMsg::CapabilityResponse(pair.0))
       )
);


named!(query_firmware<&[u8], FirmataMsg>,
       do_parse!(
           tag!(&[QUERY_FIRMWARE])                     >>
           major: take!(1)                             >>
           minor: take!(1)                             >>
           name: take_while!(|chr: u8| chr.is_ascii()) >>
           (FirmataMsg::QueryFirmware {
               major: major[0],
               minor: minor[0],
               firmware_name: name.to_vec()
           })
       )
);


named!(sysex<&[u8], FirmataMsg>,
       delimited!(
           tag!(&[START_SYSEX]),
           alt!(
               query_firmware      |
               capability_query    |
               capability_response
           ),
           tag!(&[END_SYSEX])
       )
);


named!(pub parse<&[u8], FirmataMsg>,
       alt!(
           sysex
        )
);

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &'static [u8] = b"";

    #[test]
    fn parses_sysex_query_firmware_resp() {
        let msg = b"\xF0\x79\x02\x04StandardFirmata.ino\xF7";

        assert_eq!(
            sysex(&msg[..]),
            Ok((
                EMPTY,
                FirmataMsg::QueryFirmware {
                    major: 2,
                    minor: 4,
                    firmware_name: b"StandardFirmata.ino".to_vec(),
                }
            ))
        );
    }

    #[test]
    fn parses_sysex_capability_query() {
        let msg = b"\xF0\x6B\xF7";

        assert_eq!(
            sysex(&msg[..]),
            Ok((EMPTY, FirmataMsg::CapabilityQuery))
        );
    }

    #[test]
    fn parses_pin_capability_entry() {
        let msg = b"\x00\x01";

        assert_eq!(
            capability_response_entry(&msg[..]),
            Ok((
                EMPTY,
                PinCapability {
                    mode: PinMode::DigitalInput,
                    res: 1
                }
            ))
        );
    }


    #[test]
    fn parses_pin_capability_list_0() {
        let msg = [CAPABILITY_RESPONSE_SEP];

        assert_eq!(
            capability_response_list(&msg[..]),
            Ok((
                EMPTY,
                Vec::new()
            ))
        );
    }

    #[test]
    fn parses_pin_capability_list_1() {
        let msg = [
            0x00, 0x01,
            CAPABILITY_RESPONSE_SEP
        ];

        let pin_capabilities = vec![
            PinCapability { mode: PinMode::DigitalInput, res: 1 },
        ];

        assert_eq!(
            capability_response_list(&msg[..]),
            Ok((
                EMPTY,
                pin_capabilities
            ))
        );
    }

    #[test]
    fn parses_pin_capability_list_4() {
        let msg = [
            0x00, 0x01,
            0x01, 0x01,
            0x02, 0x0A,
            0x03, 0x08,
            CAPABILITY_RESPONSE_SEP
        ];

        let pin_capabilities = vec![
            PinCapability { mode: PinMode::DigitalInput, res: 1 },
            PinCapability { mode: PinMode::DigitalOutput, res: 1 },
            PinCapability { mode: PinMode::AnalogInput, res: 10 },
            PinCapability { mode: PinMode::PWM, res: 8 },
        ];

        assert_eq!(
            capability_response_list(&msg[..]),
            Ok((
                EMPTY,
                pin_capabilities
            ))
        );
    }

    #[test]
    fn parses_sysex_capability_response() {
        let msg = [
            START_SYSEX,
            CAPABILITY_RESPONSE,
            0x00, 0x01,
            0x01, 0x01,
            0x02, 0x0A,
            0x03, 0x08,
            CAPABILITY_RESPONSE_SEP,
            0x00, 0x01,
            0x01, 0x01,
            0x02, 0x0A,
            0x03, 0x08,
            CAPABILITY_RESPONSE_SEP,
            0x00, 0x01,
            0x01, 0x01,
            0x02, 0x0A,
            0x03, 0x08,
            CAPABILITY_RESPONSE_SEP,
            END_SYSEX
        ];

        let pin_capabilities = vec![
            vec![
                PinCapability { mode: PinMode::DigitalInput, res: 1 },
                PinCapability { mode: PinMode::DigitalOutput, res: 1 },
                PinCapability { mode: PinMode::AnalogInput, res: 10 },
                PinCapability { mode: PinMode::PWM, res: 8 },
            ],
            vec![
                PinCapability { mode: PinMode::DigitalInput, res: 1 },
                PinCapability { mode: PinMode::DigitalOutput, res: 1 },
                PinCapability { mode: PinMode::AnalogInput, res: 10 },
                PinCapability { mode: PinMode::PWM, res: 8 },
            ],
            vec![
                PinCapability { mode: PinMode::DigitalInput, res: 1 },
                PinCapability { mode: PinMode::DigitalOutput, res: 1 },
                PinCapability { mode: PinMode::AnalogInput, res: 10 },
                PinCapability { mode: PinMode::PWM, res: 8 },
            ]
        ];

        assert_eq!(
            sysex(&msg[..]),
            Ok((EMPTY, FirmataMsg::CapabilityResponse(pin_capabilities)))
        );
    }

}
