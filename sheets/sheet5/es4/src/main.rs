#![allow(dead_code)]

#[derive(Debug, Default)]
struct Task {
    name: String,
    priority: i32,
    done: bool,
}

#[derive(Debug, Default)]
struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    fn push(&mut self, t: Task) {
        self.tasks.push(t);
    }
}

// ? non si capisce cosa cazzo chieda / cosa cazzo voglia fare
impl Iterator for Tasks {
    type Item = Task;

    fn next(&mut self) -> Option<Self::Item> {
        self.tasks.iter()
            .position(|t| !t.done)
            .map(|i| self.tasks.swap_remove(i))
    }
}

fn main() {
    let mut ts: Tasks = Tasks::default();
    ts.push(Task { name: "a".to_string(), priority: 1, done: false });
    ts.push(Task { name: "a".to_string(), priority: 1, done: true });
    ts.push(Task { name: "a".to_string(), priority: 1, done: true });
    ts.push(Task { name: "a".to_string(), priority: 2, done: false });

    println!("{ts:?}");
    for i in &mut ts {
        println!("{i:?}");
    }
    println!("{ts:?}"); // ???
}
