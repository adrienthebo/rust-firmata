#[allow(dead_code)]

pub const START_SYSEX: u8 = 0xF0;
pub const END_SYSEX: u8 = 0xF7;
pub const QUERY_FIRMWARE: u8 = 0x79;

use nom::IResult;


fn is_ascii(chr: u8) -> bool { chr.is_ascii() }


named!(query_firmware<&[u8], (&[u8], &[u8])>,
       tuple!(
           tag!(&[QUERY_FIRMWARE]),
           take_while!(is_ascii)
           )
      );


named!(sysex<&[u8], (&[u8], &[u8])>,
       delimited!(
           tag!(&[START_SYSEX]),
           alt!(
               query_firmware
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
                (
                    &[QUERY_FIRMWARE][..],
                    &[][..]
                )
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
                (
                    &[QUERY_FIRMWARE][..],
                    &[2, 4][..]
                )
            )
        );
    }
}
