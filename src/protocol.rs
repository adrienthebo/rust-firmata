//! Firmata protocol definitions.

/// Firmata analog pin value reporting
pub const ANALOG_REPORT: u8 = 0xC0;

/// The nybble representing an analog write or read report
pub const ANALOG_MESSAGE: u8 = 0xE;
/// The nybble representing a digitital write or read report
pub const DIGITAL_MESSAGE: u8 = 0x9;

pub const SET_PIN_MODE: u8 = 0xF4;
pub const DIGITAL_WRITE: u8 = 0xF5;

/// Firmata/MIDI sysex message begin
pub const START_SYSEX: u8 = 0xF0;
/// Firmata/MIDI sysex message end
pub const END_SYSEX: u8 = 0xF7;

/// Firmata sysex firmware query.
pub const QUERY_FIRMWARE: u8 = 0x79;

/// Firmata sysex pin capability query
pub const CAPABILITY_QUERY: u8 = 0x6B;
/// Firmata sysex pin capability response
pub const CAPABILITY_RESPONSE: u8 = 0x6C;
/// Firmata capability response record separator
pub const CAPABILITY_RESPONSE_SEP: u8 = 0x7F;

/// The Firmata protocol version
pub const PROTOCOL_VERSION: u8 = 0xF9;

/// Firmata device reset request
pub const RESET: u8 = 0xFF;

#[derive(Debug, PartialEq)]
pub enum PinMode {
    DigitalInput,
    DigitalOutput,
    AnalogInput,
    PWM,
    Servo,
    Shift,
    I2C,
    Other(u8),
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
            n => PinMode::Other(n),
        }
    }
}

impl From<PinMode> for u8 {
    fn from(item: PinMode) -> Self {
        match item {
            PinMode::DigitalInput => 0x00,
            PinMode::DigitalOutput => 0x01,
            PinMode::AnalogInput => 0x02,
            PinMode::PWM => 0x03,
            PinMode::Servo => 0x04,
            PinMode::Shift => 0x05,
            PinMode::I2C => 0x06,
            PinMode::Other(n) => n,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PinCapability {
    pub mode: PinMode,
    pub res: u8,
}

#[derive(Debug, PartialEq)]
pub enum FirmataMsg {
    QueryFirmware {
        major: u8,
        minor: u8,
        firmware_name: Vec<u8>,
    },
    CapabilityQuery,
    CapabilityResponse(Vec<Vec<PinCapability>>),
    AnalogRead {
        pin: u8,
        value: u16,
    },
    ProtocolVersion {
        major: u8,
        minor: u8,
    },
}
