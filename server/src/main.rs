use clap::Parser;
use easytcp::server::Result;
use easytcp::token::Token;
use gilrs::{Button, Gilrs};
use shared::{ClientName, ClientPacket, DEFAULT_TOKEN, ServerPacket};
use std::sync::Arc;

type Server = easytcp::server::Server<ClientPacket, ServerPacket, ClientName>;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,

    #[arg(short, long, default_value_t = 5259)]
    port: usize,

    #[arg(short, long, default_value = DEFAULT_TOKEN)]
    token: String,
}

#[derive(Default)]
struct State {
    server: Arc<Server>,
    curmod: Option<ClientName>,
}

impl State {
    const fn new(server: Arc<Server>) -> Self {
        let curmod = None;
        Self { server, curmod }
    }

    async fn handle(&mut self, packet: ServerPacket) -> Result<()> {
        match packet {
            ServerPacket::ButtonUpdate(b, i) => self.handle_button(b, i).await,
            packet @ ServerPacket::AxisChange(..) => self.send_packet(packet).await,
        }
    }

    async fn handle_button(&mut self, button: Button, is_press: bool) -> Result<()> {
        if let Ok(client) = button.try_into() {
            let client = if is_press { Some(client) } else { None };

            // do not switch clients, while a client is selected
            let client = if self.curmod.is_none() { client } else { None };

            self.curmod = client;
        } else {
            let packet = ServerPacket::ButtonUpdate(button, is_press);
            self.send_packet(packet).await?;
        }

        Ok(())
    }

    async fn send_packet(&self, packet: ServerPacket) -> Result<()> {
        if let Some(client) = &self.curmod {
            self.server.send_packet(client, packet).await
        } else {
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    let mut gilrs = Gilrs::new().unwrap();

    let addr = format!("{}:{}", args.ip, args.port);
    let token = Token::new(&args.token);

    let server = Arc::new(Server::new());

    let server_start = server.clone();
    tokio::spawn(async move { server_start.start(&addr, token).await.unwrap() });

    let mut state = State::new(server);

    loop {
        let Some(gilrs::Event { event, .. }) = gilrs.next_event() else {
            continue;
        };

        if let Ok(packet) = event.try_into() {
            if let Err(err) = state.handle(packet).await {
                log::error!("{err}");
            }
        }
    }
}
