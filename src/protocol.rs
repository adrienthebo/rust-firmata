//! Firmata protocol definitions.

pub const START_SYSEX: u8 = 0xF0;
pub const END_SYSEX: u8 = 0xF7;

pub const QUERY_FIRMWARE: u8 = 0x79;

pub const CAPABILITY_QUERY: u8 = 0x6B;
pub const CAPABILITY_RESPONSE: u8 = 0x6C;
pub const CAPABILITY_RESPONSE_SEP: u8 = 0x7F;

pub const SET_PIN_MODE: u8 = 0xF4;
pub const DIGITAL_WRITE: u8 = 0xF5;

pub const ANALOG_REPORT: u8 = 0xC0;

/// The nybble representing an analog read report
pub const ANALOG_READ: u8 = 0xE;

pub const RESET: u8 = 0xFF;


#[derive(Debug,PartialEq)]
pub enum PinMode {
    DigitalInput,
    DigitalOutput,
    AnalogInput,
    PWM,
    Servo,
    Shift,
    I2C,
    Other(u8)
}


impl From<u8> for PinMode {
    fn from(item: u8) -> Self {
        match item {
            0x00 => PinMode::DigitalInput,
            0x01 => PinMode::DigitalOutput,
            0x02 => PinMode::AnalogInput,
            0x03 => PinMode::PWM,
            0x04 => PinMode::Servo,
            0x05 => PinMode::Shift,
            0x06 => PinMode::I2C,
            n    => PinMode::Other(n)
        }
    }
}


impl From<PinMode> for u8 {
    fn from(item: PinMode) -> Self {
        match item {
            PinMode::DigitalInput  => 0x00,
            PinMode::DigitalOutput => 0x01,
            PinMode::AnalogInput   => 0x02,
            PinMode::PWM           => 0x03,
            PinMode::Servo         => 0x04,
            PinMode::Shift         => 0x05,
            PinMode::I2C           => 0x06,
            PinMode::Other(n)      => n
        }
    }
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
    AnalogRead {
        pin: u8,
        value: u16
    }
}
