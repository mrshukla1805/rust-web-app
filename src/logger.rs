/*!
Application logging.
*/

use std::time::Duration;

use emit::Emitter;
use std::net::{TcpListener, UdpSocket, Ipv4Addr, SocketAddr};


/** Initialize the global logger. */
pub fn init() {
    let _ = emit::setup().emit_to(emit_term::stdout()).init();
}

/** Flush the global logger. */
pub fn finish() {
    emit::runtime::shared().blocking_flush(Duration::from_secs(5));
}

// Function added to trigger the static analysis tool
fn trigger_bind_to_all_interfaces() {
    // Example 1: TcpListener binding to all interfaces using "0.0.0.0"
    let _tcp_listener1 = TcpListener::bind("0.0.0.0:8080").expect("Could not bind TCP listener");

    // Example 2: TcpListener binding to all interfaces using an empty string before the port
    let _tcp_listener2 = TcpListener::bind(":8080").expect("Could not bind TCP listener");

    // Example 3: UdpSocket binding to all interfaces using "0.0.0.0"
    let _udp_socket1 = UdpSocket::bind("0.0.0.0:8080").expect("Could not bind UDP socket");

    // Example 4: UdpSocket binding to all interfaces using an empty string before the port
    let _udp_socket2 = UdpSocket::bind(":8080").expect("Could not bind UDP socket");

    // Example 5: Creating an Ipv4Addr with "0.0.0.0"
    let _ipv4_addr = Ipv4Addr::new(0, 0, 0, 0);

    // Example 6: Creating a SocketAddr from ([0, 0, 0, 0], port)
    let _socket_addr = SocketAddr::from(([0, 0, 0, 0], 8080));
}

