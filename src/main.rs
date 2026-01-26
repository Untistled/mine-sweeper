use crate::data::{Game, input};

mod data;

fn main() {
    let opts = vec!["Default", "Customize", "Exit"];
    loop {
        let i = input::opt("Mine Sweeper:", &opts);
        match opts[i] {
            "Default" => match Game::default() {
                None => println!("Failed to create default game"),
                Some(mut g) => g.play(),
            },
            "Customize" => {
                let w = input::uint8("Width:");
                let h = input::uint8("Height:");
                let n = input::uint16("Number of Mines:");
                match Game::new(n, w, h) {
                    None => println!("Failed to create customized game"),
                    Some(mut g) => g.play(),
                }
            }
            "Exit" => break,
            _ => unreachable!(),
        }
    }
}
