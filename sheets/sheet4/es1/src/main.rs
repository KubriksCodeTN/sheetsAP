use rand::{distributions::Uniform, Rng};

fn find_equal<'b, 'a>(s1: &'b str, s2: &'a str) -> Option<(&'b str, &'a str)> {
    for i in 0..s1.len() - 1 {
        if let Some(pos) = s2.find(&s1[i..i + 2]) {
            return Some((&s1[i..i + 2], &s2[pos..pos + 2]));
        }
    }

    return None;
}

fn lucky_slice(input_str: &str) -> Option<&str> {
    let s: String = rand::thread_rng()
        .sample_iter(Uniform::new(char::from('a'), char::from('z' as u8 + 1)))
        .take(input_str.len())
        .collect();

    println!("{input_str} {s}");

    let (s1, _) = find_equal(input_str, &s)?;

    return Some(s1);
}

fn main() {
    println!("{}", lucky_slice("fibidvbriubfidvbdfbufdbvid").unwrap_or_default());
}
