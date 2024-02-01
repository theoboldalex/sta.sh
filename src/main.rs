fn main() {
    let cmd = std::env::args().nth(1).expect("No command given. What should I do?");
    let name = std::env::args().nth(2).expect("Stashed directories require a name");

    println!("cmd: {:?}, name: {:?}", cmd, name);
}
