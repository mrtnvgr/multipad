use clap::Parser;
use easytcp::token::Token;
use gilrs::{Axis, Button};
use shared::{ClientName, ClientPacket, DEFAULT_TOKEN, ServerPacket};
use std::{thread, time::Duration};
use uinput::Device;
use uinput::event::absolute::Position;
use uinput::event::controller::{DPad, GamePad};

type Client = easytcp::client::Client<ClientPacket, ServerPacket, ClientName>;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    name: ClientName,

    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,

    #[arg(short, long, default_value_t = 5259)]
    port: usize,

    #[arg(short, long, default_value = DEFAULT_TOKEN)]
    token: String,

    #[arg(long)]
    pretend: bool,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    let addr = format!("{}:{}", args.ip, args.port);
    let token = Token::new(&args.token);

    let client = Client::connect(args.name, &addr, token)
        .await
        .expect("Failed to connect to a server");

    let mut device = create_device().unwrap();

    while let Ok(packet) = client.receive_packet().await {
        log::info!("Received packet from server: {packet:?}");

        if args.pretend {
            continue;
        }

        match packet {
            ServerPacket::ButtonUpdate(b, ip) => handle_bu(&mut device, b, ip),
            ServerPacket::AxisChange(axis, value) => handle_axis(&mut device, axis, value),
        }
    }
}

fn create_device() -> Result<Device, uinput::Error> {
    let name = format!("{DEFAULT_TOKEN} Virtual Device");

    let device = uinput::default()?
        .name(name)?
        .event(uinput::event::Controller::All)?
        .create()?;

    // Let's wait for device to initialize
    thread::sleep(Duration::from_secs(1));

    Ok(device)
}

enum UinputButton {
    GamePad(GamePad),
    DPad(DPad),
    Unknown,
}

impl From<Button> for UinputButton {
    fn from(button: Button) -> Self {
        match button {
            Button::South => Self::GamePad(GamePad::South),
            Button::East => Self::GamePad(GamePad::East),
            Button::North => Self::GamePad(GamePad::North),
            Button::West => Self::GamePad(GamePad::West),

            Button::C => Self::GamePad(GamePad::C),
            Button::Z => Self::GamePad(GamePad::Z),

            Button::LeftTrigger => Self::GamePad(GamePad::TL2),
            Button::LeftTrigger2 => Self::GamePad(GamePad::TL),
            Button::RightTrigger => Self::GamePad(GamePad::TR2),
            Button::RightTrigger2 => Self::GamePad(GamePad::TR),

            Button::Select => Self::GamePad(GamePad::Select),
            Button::Start => Self::GamePad(GamePad::Start),
            Button::Mode => Self::GamePad(GamePad::Mode),

            Button::LeftThumb => Self::GamePad(GamePad::ThumbL),
            Button::RightThumb => Self::GamePad(GamePad::ThumbR),

            Button::DPadUp => Self::DPad(DPad::Up),
            Button::DPadDown => Self::DPad(DPad::Down),
            Button::DPadLeft => Self::DPad(DPad::Left),
            Button::DPadRight => Self::DPad(DPad::Right),

            Button::Unknown => Self::Unknown,
        }
    }
}

fn handle_bu(device: &mut Device, button: Button, is_pressed: bool) {
    match button.into() {
        UinputButton::GamePad(button) if is_pressed => device.press(&button).unwrap(),
        UinputButton::DPad(button) if is_pressed => device.press(&button).unwrap(),

        UinputButton::GamePad(button) => device.release(&button).unwrap(),
        UinputButton::DPad(button) => device.release(&button).unwrap(),

        UinputButton::Unknown => log::warn!("Received an unknown button"),
    }
}

fn handle_axis(device: &mut Device, axis: Axis, value: i32) {
    let event = match axis {
        Axis::LeftStickX => Position::X,
        Axis::LeftStickY => Position::Y,
        Axis::LeftZ => Position::Z,

        Axis::RightStickX => Position::RX,
        Axis::RightStickY => Position::RY,
        Axis::RightZ => Position::RZ,

        _ => return,
    };

    device.position(&event, value).unwrap();
}
