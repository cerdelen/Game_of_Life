use std::thread::sleep;
use std::time::Duration;

mod gol;
use gol::{Gol, Pos};

mod figures;
use figures::Figures;

fn main() {
    let mut starting_pos = Vec::new();
    let mut gun = Figures::beacon(20, 20);
    // let mut gun = gol::Figures::gospel_glider_gun(2, 2);
    starting_pos.append(&mut gun.figures);
    let mut gol = Gol::new(80, 40, &starting_pos);

    loop {
        gol.print();
        sleep(Duration::from_millis(100));
        gol.step();
        clearscreen::clear().unwrap();
    }
}
