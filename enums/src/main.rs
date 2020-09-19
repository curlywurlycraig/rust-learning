enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IpAddr {
    fn ping(&self) {
        match self {
            IpAddr::V4(127, 0, 0, 1) => {
                println!("It's home.");
            },
            IpAddr::V4(_, _, _, _) => {
                println!("It's a v4");
            },
            IpAddr::V6(_) => {
                println!("it's a v6");
            }
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    loopback.ping();
    home.ping();
}
