// 構造体
struct Philosopher {
    name: String,
}

// 構造体の実装
impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            // to_stringすることでコピーが作られる
            name: name.to_string(),
        }
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Grilles Deleuze"),
    ];

    for p in &philosophers {
        println!("name: {}", p.name);
    }
}
