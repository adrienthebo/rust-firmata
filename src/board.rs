use protocol::FirmataMsg;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Pin {
    pub value: u16,
}

#[derive(Debug, Default)]
pub struct Firmware {
    pub major: u8,
    pub minor: u8,
    pub name: String,
}

#[derive(Debug, Default)]
pub struct Protocol(pub u8, pub u8);

#[derive(Debug, Default)]
pub struct Board {
    pub firmware: Option<Firmware>,
    pub protocol: Option<Protocol>,
    pub pins: HashMap<u8, Pin>,
}

impl Board {
    pub fn reset(&mut self) {
        *self = Board::default();
    }

    pub fn update(&mut self, msg: FirmataMsg) {
        match msg {
            FirmataMsg::AnalogRead { pin, value } => {
                let pin_value = Pin { value: value };
                self.pins.insert(pin, pin_value);
            }
            FirmataMsg::QueryFirmware {
                major,
                minor,
                firmware_name,
            } => {
                let firmware = Firmware {
                    major: major,
                    minor: minor,
                    name: String::from_utf8_lossy(&firmware_name).into(),
                };
                self.firmware = Some(firmware)
            }
            FirmataMsg::ProtocolVersion { major, minor } => {
                // Note that the protocol version is usually only sent when the Firmata device
                // comes online. Because of this we can consider doing a state reset when this
                // message is received.
                self.reset();
                self.protocol = Some(Protocol(major, minor))
            }
            _ => {
                warn!("Unhandled firmata message {:?}", msg);
            }
        }
    }
}
