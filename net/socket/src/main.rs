// use log;
use log::error;
use std::env;

mod tcp_client;
mod tcp_server;
mod udp_server;
mod udp_client;

fn main() {
    //     let protocol: &'static str = "tcp";
    //     let role: &'static str = "server";
    //     let address: &'static str = "127.0.0.1:8999";
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp|udp] [server|client] [addr:port].");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];

    match protocol {
        "tcp" => match role {
            "server" => {
                tcp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                tcp_client::connect(address).unwrap_or_else(|e| error!("{}", e));
            }
            _ => missing_role(),
        },
        "udp" => match role {
            "server" => {
                udp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                udp_client::communicate(address).unwrap_or_else(|e| error!("{}", e));
            }
            _ => {
                missing_role();
            }
        }
        _ => missing_role(),
    }
}

fn missing_role() {
    error!("Please specify serer or client on the 2nd argument."); // log::error
    std::process::exit(1);
}
