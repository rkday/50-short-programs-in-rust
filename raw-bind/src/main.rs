use nix::sys::socket::*;

fn main() {
    let s = socket(AddressFamily::Inet, SockType::Raw, SockFlag::empty(), Some(SockProtocol::Udp)).expect("Failed to get socket");
    let addr = SockAddr::new_inet(InetAddr::new(IpAddr::new_v4(127,0, 0, 1), 15000));
    bind(s, &addr).expect("Failed to bind");
    println!("Bound successfully!");
}
