use clap::ValueEnum;
use gilrs::{Axis, Button, EventType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientPacket {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerPacket {
    ButtonUpdate(Button, bool),
    AxisChange(Axis, i32),
}

impl TryFrom<EventType> for ServerPacket {
    type Error = ();

    fn try_from(value: EventType) -> Result<Self, Self::Error> {
        #[allow(clippy::cast_possible_truncation)]
        let trun = |x| (x * 32767.0) as i32;

        match value {
            EventType::ButtonPressed(button, _) => Ok(Self::ButtonUpdate(button, true)),
            EventType::ButtonReleased(button, _) => Ok(Self::ButtonUpdate(button, false)),
            EventType::AxisChanged(axis, value, _) => Ok(Self::AxisChange(axis, trun(value))),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug, ValueEnum)]
pub enum ClientName {
    LeftTrigger,
    LeftTrigger2,
    RightTrigger,
    RightTrigger2,
}

impl TryFrom<Button> for ClientName {
    type Error = ();

    fn try_from(value: Button) -> Result<Self, Self::Error> {
        match value {
            Button::LeftTrigger => Ok(Self::LeftTrigger),
            Button::LeftTrigger2 => Ok(Self::LeftTrigger2),
            Button::RightTrigger => Ok(Self::RightTrigger),
            Button::RightTrigger2 => Ok(Self::RightTrigger2),
            _ => Err(()),
        }
    }
}

pub const DEFAULT_TOKEN: &str = "Multipad";
