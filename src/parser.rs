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
pub enum SysexMsg<'a> {
    QueryFirmware {
        major: Option<&'a u8>,
        minor: Option<&'a u8>,
        firmware_name: Option<&'a [u8]>
    },
    CapabilityQuery,
    CapabilityResponse(Vec<Vec<PinCapability>>),
}


#[derive(Debug,PartialEq)]
pub enum FirmataMsg<'a> {
    Sysex(SysexMsg<'a>)
}


named!(capability_query<&[u8], SysexMsg>,
       map!(tag!(&[CAPABILITY_QUERY]), |_| SysexMsg::CapabilityQuery));


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
                   n @ _ => PinMode::Other(n)
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


named!(capability_response<&[u8], SysexMsg>,
       do_parse!(
           tag!(&[CAPABILITY_RESPONSE]) >>
           pair: many_till!(
               call!(capability_response_list),
               peek!(tag!(&[END_SYSEX]))
           ) >> (SysexMsg::CapabilityResponse(pair.0))
       )
);


named!(query_firmware<&[u8], SysexMsg>,
       alt_complete!(
           do_parse!(
               tag!(&[QUERY_FIRMWARE])                           >>
               major: opt!(take!(1))                             >>
               minor: opt!(take!(1))                             >>
               name: opt!(take_while!(|chr: u8| chr.is_ascii())) >>
               (SysexMsg::QueryFirmware {
                   major: major.map(|b| &b[0]),
                   minor: minor.map(|b| &b[0]),
                   firmware_name: name
               })
           )
           | map!(tag!(&[QUERY_FIRMWARE]), |_| {
               SysexMsg::QueryFirmware {
                   major: None,
                   minor: None,
                   firmware_name: None,
               }}
           )
       )
);


named!(sysex<&[u8], SysexMsg>,
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


#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &'static [u8] = b"";

    #[test]
    fn parses_sysex_query_firmware_cmd() {
        let msg = b"\xF0\x79\xF7";

        assert_eq!(
            sysex(&msg[..]),
            Ok((
                EMPTY,
                SysexMsg::QueryFirmware {
                    major: None,
                    minor: None,
                    firmware_name: None,
                }
            ))
        );
    }

    #[test]
    fn parses_sysex_query_firmware_resp() {
        let msg = b"\xF0\x79\x02\x04StandardFirmata.ino\xF7";

        assert_eq!(
            sysex(&msg[..]),
            Ok((
                EMPTY,
                SysexMsg::QueryFirmware {
                    major: Some(&2),
                    minor: Some(&4),
                    firmware_name: Some(&b"StandardFirmata.ino"[..]),
                }
            ))
        );
    }

    #[test]
    fn parses_sysex_capability_query() {
        let msg = b"\xF0\x6B\xF7";

        assert_eq!(
            sysex(&msg[..]),
            Ok((EMPTY, SysexMsg::CapabilityQuery))
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
            Ok((EMPTY, SysexMsg::CapabilityResponse(pin_capabilities)))
        );
    }

}
