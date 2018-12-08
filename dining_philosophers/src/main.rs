use std::thread;
use std::time::Duration;

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

    fn eat(&self) {
        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating", self.name);

    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Grilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    for p in &philosophers {
        println!("name: {}", p.name);
        p.eat();
    }
}
