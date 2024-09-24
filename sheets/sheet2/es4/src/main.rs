pub fn sub_slice(v1: &Vec<i32>, v2: &Vec<i32>) {
    if v2.len() > v1.len() { 
        println!("not found"); 
        return; 
    }

    let nt: usize = v1.len() - v2.len() + 1;
    for i in 0..nt {
        if &v1[i..(i + v2.len())] == v2 {
            println!("{:?}", &v1[i..(i + v2.len())]);
            return;
        }
    }

    println!("not found");
}

fn main() {
    sub_slice(&vec![1, 2, 3, 4, 5, 6], &vec![1]);
    sub_slice(&vec![1, 2, 3, 4, 5, 6], &vec![1, 2, 3]);
    sub_slice(&vec![1, 2, 3, 4, 5, 6], &vec![1, 3, 4]);
    sub_slice(&vec![1, 2, 3, 4, 5, 6], &vec![45, 6, 65, 666]);
    sub_slice(&vec![1, 2, 3, 4, 5, 6], &vec![45, 6, 65, 666, 1, 1, 1, 1, 1, 1, 1]);
    sub_slice(&vec![], &vec![]);
}
