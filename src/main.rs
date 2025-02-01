use std::thread::sleep;
use std::time::Duration;

mod gol;
use gol::{Gol, Pos};

fn main() {
    let mut starting_pos = Vec::new();
    let mut gun = gol::Figures::gospel_glider_gun(2, 2);
    starting_pos.append(&mut gun.figures);
    let mut gol = Gol::new(50, 40, &starting_pos);

    loop {
        gol.print();
        sleep(Duration::from_millis(50));
        gol.step();
        clearscreen::clear().unwrap();
    }
}
