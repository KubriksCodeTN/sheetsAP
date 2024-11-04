use core::str;
use std::io;

fn main() -> io::Result<()> { 
    let input: String = io::stdin().lines().flat_map(|s| s.map(|mut s| {
        s.push('\n');
        s
    })).collect();
    let it: std::slice::Chunks<'_, u8> = input.as_bytes().chunks(16);
    it.enumerate().for_each(|(i, v)| {
        let mut buff = String::new();
        buff.reserve(167);
        buff.push_str(&format!("\x1b[0m{:0>8x}: ", i << 4));
        v.chunks(2).for_each(|v| {
            let hex: String = v.iter().fold("".to_string(), |acc: String, c: &u8| {
                match c {
                    0x0a => acc + "\x1b[1;33m0a",
                    _ => acc + &format!("\x1b[1;32m{c:0>2x}"),
                }
            });
            buff.push_str(&hex);
            buff.push(' ');
        });
        let mut r = (16 - v.len()) << 1;
        r += r >> 2;
        println!("{buff} {}\x1b[1;32m{}", " ".repeat(r), str::from_utf8(v).unwrap().replace('\n', "\x1b[1;33m.\x1b[1;32m"));
    });
    Ok(())
}
