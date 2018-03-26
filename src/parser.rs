use protocol::*;


named!(capability_query<&[u8], FirmataMsg>,
       map!(tag!(&[CAPABILITY_QUERY]), |_| FirmataMsg::CapabilityQuery));


named!(capability_response_entry<&[u8], PinCapability>,
       do_parse!(
           mode: take!(1) >>
           res: take!(1)  >>
           (PinCapability {
               res: res[0],
               mode: PinMode::from(mode[0])
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


named!(analog_read<&[u8], FirmataMsg>,
       bits!(
           do_parse!(
               tag_bits!(u8, 4, ANALOG_READ) >>
               pin: take_bits!(u8, 4)        >>
               lsb: take_bits!(u8, 8)        >>
               msb: take_bits!(u8, 8)        >>
               (FirmataMsg::AnalogRead {
                       pin: pin,
                       value: ((msb as u16) << 7) | (lsb as u16)
               })
           )
       )
);


named!(protocol_version<&[u8], FirmataMsg>,
       do_parse!(
           tag!(&[PROTOCOL_VERSION]) >>
           major: take!(1)           >>
           minor: take!(1)           >>
           (FirmataMsg::ProtocolVersion {
               major: major[0],
               minor: minor[0]
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
           sysex       |
           analog_read |
           protocol_version
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

    #[test]
    fn parses_analog_read() {
        let pin = 15;
        let value: u16 = 616;

        let msb = ((value & !0x7F) >> 7) as u8;
        let lsb = (value & 0x7F) as u8;

        let msg: [u8; 3] = [ANALOG_READ << 4 | pin, lsb, msb];

        assert_eq!(
            parse(&msg[..]),
                Ok((EMPTY, FirmataMsg::AnalogRead { pin, value }))
        );
    }

    #[test]
    fn parses_protol_version() {
        assert_eq!(
            parse(&[PROTOCOL_VERSION, 2, 6][..]),
                Ok((EMPTY, FirmataMsg::ProtocolVersion { major: 2, minor: 6 }))
        );
    }
}
