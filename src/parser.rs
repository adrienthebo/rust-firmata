#[allow(dead_code)]

pub const START_SYSEX: u8 = 0xF0;
pub const END_SYSEX: u8 = 0xF7;

pub const QUERY_FIRMWARE: u8 = 0x79;

pub const CAPABILITY_QUERY: u8 = 0x6B;
pub const CAPABILITY_RESPONSE: u8 = 0x6c;
pub const CAPABILITY_RESPONSE_SEP: u8 = 0x7F;

use nom::IResult;


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
}


named!(capability_query<&[u8], SysexMsg>,
       map!(tag!(&[CAPABILITY_QUERY]), |_| SysexMsg::CapabilityQuery));


named!(capability_response_entry<&[u8], PinCapability>,
       do_parse!(
           mode: take!(1)                   >>
           res: take!(1)                    >>
           tag!(&[CAPABILITY_RESPONSE_SEP]) >>
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
               query_firmware |
               capability_query
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
            IResult::Done(
                EMPTY,
                SysexMsg::QueryFirmware {
                    major: None,
                    minor: None,
                    firmware_name: None,
                }
            )
        );
    }

    #[test]
    fn parses_sysex_query_firmware_resp() {
        let msg = b"\xF0\x79\x02\x04StandardFirmata.ino\xF7";

        assert_eq!(
            sysex(&msg[..]),
            IResult::Done(
                EMPTY,
                SysexMsg::QueryFirmware {
                    major: Some(&2),
                    minor: Some(&4),
                    firmware_name: Some(&b"StandardFirmata.ino"[..]),
                }
            )
        );
    }

    #[test]
    fn parses_sysex_capability_query() {
        let msg = b"\xF0\x6B\xF7";

        assert_eq!(
            sysex(&msg[..]),
            IResult::Done(EMPTY, SysexMsg::CapabilityQuery)
        );
    }

    fn parses_pin_capability_entry() {
        let msg = b"\x00\x01\x7F";

        assert_eq!(
            capability_response_entry(&msg[..]),
            IResult::Done(
                EMPTY,
                PinCapability {
                    mode: PinMode::DigitalInput,
                    res: 1
                }
            )
        );
    }
}
