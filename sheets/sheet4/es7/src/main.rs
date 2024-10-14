fn skip_prefix<'a>(tn: &'a str, prefix: &str) -> &'a str {
    tn.strip_prefix(prefix).unwrap_or(tn)
}

fn main() {
    println!("{}", skip_prefix("+39 332 332 3344", "+39 "));
    println!("{}", skip_prefix("+39 332 332 3344", "+34 "));o
}
