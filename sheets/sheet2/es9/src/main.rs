enum Either<A, B> {
    Left(A),
    Right(B),
}

fn main() {
    let mut v: Vec<Either<i32, String>> = Vec::new();
    v.push(Either::Left(3));
    v.push(Either::Right("yooo".to_string()));
}
