use std::io::{self, Read, Write};

fn main() {
    // Самый простой echo-агент:
    // что пришло на stdin — то же отправим на stdout.
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let _ = io::stdout().write_all(buf.as_bytes());
}
