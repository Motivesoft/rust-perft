fn main() {
    display_details();
}

fn display_details() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!( "{name} {version}" );
}
