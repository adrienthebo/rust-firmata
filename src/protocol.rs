//! Firmata protocol definitions.

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
    pub mode: PinMode,
    pub res: u8
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
