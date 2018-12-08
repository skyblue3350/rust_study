use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

// 構造体
struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}
struct Table {
    // Mutexのベクトル
    forks: Vec<Mutex<()>>,
}

// 構造体の実装
impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            // to_stringすることでコピーが作られる
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        // アンダーバーをつけると未使用警告が出ない

        // 左手に持ち替える時間
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));

        // 右手に持ち替える時間
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating", self.name);

    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Grilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    let table = Arc::new(Table {forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
