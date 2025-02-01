use std::io::{self, Write, BufWriter};

const CHAR_DEAD: char = ' ';
const CHAR_ALIVE: char = '#';

pub struct Gol {
    w: usize,
    h: usize,
    area: Vec<bool>,
}

pub struct Pos {
    pub x: usize,
    pub y: usize
}

impl Gol {
    fn count_live_neighbours(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for i in y.saturating_sub(1)..y + 2 {
            for j in x.saturating_sub(1)..x + 2 {
                if i == y && j == x {
                    continue
                }
                if let Some(state) = self.area.get(i*self.w + j) {
                    if *state == true {
                        count += 1;
                        if count == 4 {
                            return count;
                        }
                    }
                }
            }
        }
        count
    }

    fn get_char_at(&self, x: usize, y: usize) -> char {
        match self.area[self.w * y + x] {
            true => CHAR_ALIVE,
            false => CHAR_DEAD,
        }
    }

    pub fn new(w: usize, h: usize, alive: &Vec<Pos>) -> Self {
        let mut area = vec![false; w*h];
        for pos in alive {
            area[w * pos.y + pos.x] = true;
        }
        Gol {w, h, area }
    }

    pub fn print(&self) {
        let stdout = io::stdout();
        let mut writer = BufWriter::new(stdout.lock());
        for y in 0..self.h {
            let mut line = String::with_capacity(self.w);
            for x in 0..self.w {
                line.push(self.get_char_at(x, y));
            }
            writeln!(writer, "{}", line).unwrap();
        }
        writer.flush().unwrap();
    }

    pub fn step(&mut self) {
        let mut new_area = vec![false; self.h * self.w];
        for y in 0..self.h {
            for x in 0..self.w {
                match self.area[y * self.w + x] {
                    true => {
                        // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                        // Any live cell with more than three live neighbours dies, as if by overpopulation.
                        // Any live cell with two or three live neighbours lives on to the next generation.
                        match self.count_live_neighbours(x, y) {
                            2 | 3 => new_area[self.w * y + x] = true,
                            _ => (),
                        }
                    },
                    false => {
                        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                        match self.count_live_neighbours(x, y) {
                            3 => new_area[self.w * y + x] = true,
                            _ => (),
                        }
                    },
                }
            }
        }
        self.area = new_area;
    }
}

pub struct Figures {
    pub figures: Vec<Pos>,
}

impl Figures {
    pub fn gospel_glider_gun(x: usize, y: usize) -> Self {
        Self{ figures: vec![

            Pos{ x: 1, y: 5 },
            Pos{ x: 1, y: 6 },
            Pos{ x: 2, y: 5 },
            Pos{ x: 2, y: 6 },

            Pos{ x: 11, y: 5 },
            Pos{ x: 11, y: 6 },
            Pos{ x: 11, y: 7 },

            Pos{ x: 12, y: 4 },
            Pos{ x: 12, y: 8 },

            Pos{ x: 13, y: 3 },
            Pos{ x: 13, y: 9 },

            Pos{ x: 14, y: 3 },
            Pos{ x: 14, y: 9 },

            Pos{ x: 15, y: 6 },

            Pos{ x: 16, y: 4 },
            Pos{ x: 16, y: 8 },

            Pos{ x: 17, y: 5 },
            Pos{ x: 17, y: 6 },
            Pos{ x: 17, y: 7 },

            Pos{ x: 18, y: 6 },

            Pos{ x: 21, y: 3 },
            Pos{ x: 21, y: 4 },
            Pos{ x: 21, y: 5 },

            Pos{ x: 22, y: 3 },
            Pos{ x: 22, y: 4 },
            Pos{ x: 22, y: 5 },

            Pos{ x: 23, y: 2 },
            Pos{ x: 23, y: 6 },

            Pos{ x: 25, y: 1 },
            Pos{ x: 25, y: 2 },
            Pos{ x: 25, y: 6 },
            Pos{ x: 25, y: 7 },

            Pos{ x: 36, y: 3 },
            Pos{ x: 36, y: 4 },
            Pos{ x: 37, y: 3 },
            Pos{ x: 37, y: 4 },



            // Pos{ x: x + 1, y: y + 1 },
            // Pos{ x: x, y: y + 1 },
            // Pos{ x: x + 1, y: y },
            // Pos{ x: x, y: y},
        ]}
    }

    pub fn square(x: usize, y: usize) -> Self {
        Self{ figures: vec![
            Pos{ x: x + 1, y: y + 1 },
            Pos{ x: x, y: y + 1 },
            Pos{ x: x + 1, y: y },
            Pos{ x: x, y: y},
        ]}
    }

    pub fn _stick(x: usize, y: usize) -> Self {
        Self{ figures: vec![
            Pos{ x: x + 1, y: y },
            Pos{ x: x - 1, y: y },
            Pos{ x: x, y: y},
        ]}
    }

    pub fn _bubble(x: usize, y: usize) -> Self {
        Self{ figures: vec![
            Pos{ x: x + 1, y: y },

            Pos{ x: x, y: y + 1 },
            Pos{ x: x + 2, y: y + 1 },

            Pos{ x: x, y: y + 2 },
            Pos{ x: x + 2, y: y + 2 },

            Pos{ x: x + 1, y: y + 3 },
        ]}
    }
}
