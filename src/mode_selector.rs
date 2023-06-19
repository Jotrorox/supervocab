pub fn get_if_server_mode() -> bool {
    println!("Run in server mode? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "y" {
        return true;
    } else {
        return false;
    }
}