mod client;
mod command;
mod server;

use std::net::SocketAddr;

use argh::FromArgs;
use client::client_main;
use server::server_main;

/// geph5-prototest: A tool with server and client subcommands.
#[derive(FromArgs)]
struct Args {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    /// start the server
    Server(ServerCmd),

    /// start the client
    Client(ClientCmd),
}

/// Start the server with a listening address.
#[derive(FromArgs)]
#[argh(subcommand, name = "server")]
struct ServerCmd {
    /// address to listen on (e.g., 127.0.0.1:8080)
    #[argh(option, long = "listen")]
    listen: SocketAddr,
    /// sosistab3 cookie for obfuscation
    #[argh(option, long = "sosistab3")]
    sosistab3: Option<String>,
}

/// Start the client with a connection address.
#[derive(FromArgs)]
#[argh(subcommand, name = "client")]
struct ClientCmd {
    /// address to connect to (e.g., 127.0.0.1:8080)
    #[argh(option, long = "connect")]
    connect: SocketAddr,
    /// sosistab3 cookie for obfuscation
    #[argh(option, long = "sosistab3")]
    sosistab3: Option<String>,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args: Args = argh::from_env();

    match args.subcommand {
        Subcommand::Server(cmd) => smolscale::block_on(server_main(cmd.listen, cmd.sosistab3)),
        Subcommand::Client(cmd) => smolscale::block_on(client_main(cmd.connect, cmd.sosistab3)),
    }
}
