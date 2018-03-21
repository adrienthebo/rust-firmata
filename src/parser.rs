#[allow(dead_code)]

pub const START_SYSEX: u8 = 0xF0;
pub const END_SYSEX: u8 = 0xF7;
pub const QUERY_FIRMWARE: u8 = 0x79;

use nom::IResult;


fn is_ascii(chr: u8) -> bool { chr.is_ascii() }

#[derive(Debug,PartialEq)]
pub enum SysexMsg<'a> {
    QueryFirmware {
        major: Option<&'a u8>,
        minor: Option<&'a u8>,
        firmware_name: Option<&'a [u8]>
    }
}


named!(query_firmware<&[u8], SysexMsg>,
       do_parse!(
           tag!(&[QUERY_FIRMWARE])            >>
           major: opt!(take!(1))              >>
           minor: opt!(take!(1))              >>
           name: opt!(take_while!(is_ascii)) >>
           (SysexMsg::QueryFirmware {
               major: major.map(|b| &b[0]),
               minor: minor.map(|b| &b[0]),
               firmware_name: name
           })
           )
      );


named!(sysex<&[u8], SysexMsg>,
       delimited!(
           tag!(&[START_SYSEX]),
           query_firmware,
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
        let msg = b"\xF0\x79\x02\x04\xF7";

        assert_eq!(
            sysex(&msg[..]),
            IResult::Done(
                EMPTY,
                SysexMsg::QueryFirmware {
                    major: Some(&2),
                    minor: Some(&4),
                    firmware_name: Some(&b""[..]),
                }
            )
        );
    }
}
