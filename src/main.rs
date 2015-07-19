extern crate mio;
extern crate http_muncher;
extern crate sha1;
extern crate rustc_serialize;

mod server;
mod client;

use server::WebSocketServer;
use server::SERVER_TOKEN;
use mio::*;
use mio::tcp::*;

use std::str::FromStr;

fn main() {
    let server_socket = TcpSocket::v4().unwrap();

    let address = FromStr::from_str("0.0.0.0:10000").unwrap();
    server_socket.bind(&address).unwrap();

    let server_socket = server_socket.listen(256).unwrap();

    let mut event_loop = EventLoop::new().unwrap();

    let mut server = WebSocketServer::from_socket(server_socket);

    event_loop.register_opt(
        &server.socket,
        SERVER_TOKEN,
        EventSet::readable(),
        PollOpt::edge()
    ).unwrap();
    event_loop.run(&mut server).unwrap();
}
