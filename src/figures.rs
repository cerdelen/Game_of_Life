use crate::gol::Pos;

pub struct Figures {
    pub figures: Vec<Pos>,
}

impl Figures {
    pub fn glider(x: usize, y: usize) -> Self {
        Self{ figures: vec![
            Pos{ x: x + 1, y: y + 2 },
            Pos{ x: x + 2, y: y + 3 },
            Pos{ x: x + 3, y: y + 3 },
            Pos{ x: x + 3, y: y + 2 },
            Pos{ x: x + 3, y: y + 1 },
        ]}
    }

    pub fn toad(x: usize, y: usize) -> Self {
        Self{ figures: vec![
            Pos{ x: x + 2, y: y + 1 },
            Pos{ x: x + 3, y: y + 1 },
            Pos{ x: x + 4, y: y + 1 },
            Pos{ x: x + 1, y: y + 2 },
            Pos{ x: x + 2, y: y + 2 },
            Pos{ x: x + 3, y: y + 2 },
        ]}
    }

    pub fn beacon(x: usize, y: usize) -> Self {
        let mut figure = vec![];
        figure.append(&mut Figures::square(x + 0, y + 0).figures);
        figure.append(&mut Figures::square(x + 2, y + 2).figures);

        Self{ figures: figure }
    }

    pub fn bee_hive(x: usize, y: usize) -> Self {
        Self{ figures: vec![
            Pos{ x: x + 0, y: y + 1 },
            Pos{ x: x + 1, y: y + 0 },
            Pos{ x: x + 1, y: y + 2 },
            Pos{ x: x + 2, y: y + 0 },
            Pos{ x: x + 3, y: y + 2 },
            Pos{ x: x + 3, y: y + 1 },
        ]}
    }

    pub fn tub(x: usize, y: usize) -> Self {
        Self{ figures: vec![
            Pos{ x: x + 1, y: y + 0 },
            Pos{ x: x + 0, y: y + 1 },
            Pos{ x: x + 1, y: y + 2 },
            Pos{ x: x + 2, y: y + 1 },
        ]}
    }

    pub fn mini_gun(x: usize, y: usize) -> Self {
        Self{ figures: vec![
            Pos{ x: x + 1, y: y + 6 },
            Pos{ x: x + 3, y: y + 6 },
            Pos{ x: x + 3, y: y + 5 },
            Pos{ x: x + 5, y: y + 4 },
            Pos{ x: x + 5, y: y + 3 },
            Pos{ x: x + 5, y: y + 2 },
            Pos{ x: x + 7, y: y + 1 },
            Pos{ x: x + 7, y: y + 2 },
            Pos{ x: x + 7, y: y + 3 },
            Pos{ x: x + 8, y: y + 2 },
        ]}
    }

    pub fn gospel_glider_gun(x: usize, y: usize) -> Self {
        Self{ figures: vec![
            Pos{ x: x + 1, y: y + 5 },
            Pos{ x: x + 1, y: y + 6 },
            Pos{ x: x + 2, y: y + 5 },
            Pos{ x: x + 2, y: y + 6 },

            Pos{ x: x + 11, y: y + 5 },
            Pos{ x: x + 11, y: y + 6 },
            Pos{ x: x + 11, y: y + 7 },

            Pos{ x: x + 12, y: y + 4 },
            Pos{ x: x + 12, y: y + 8 },

            Pos{ x: x + 13, y: y + 3 },
            Pos{ x: x + 13, y: y + 9 },

            Pos{ x: x + 14, y: y + 3 },
            Pos{ x: x + 14, y: y + 9 },

            Pos{ x: x + 15, y: y + 6 },

            Pos{ x: x + 16, y: y + 4 },
            Pos{ x: x + 16, y: y + 8 },

            Pos{ x: x + 17, y: y + 5 },
            Pos{ x: x + 17, y: y + 6 },
            Pos{ x: x + 17, y: y + 7 },

            Pos{ x: x + 18, y: y + 6 },

            Pos{ x: x + 21, y: y + 3 },
            Pos{ x: x + 21, y: y + 4 },
            Pos{ x: x + 21, y: y + 5 },

            Pos{ x: x + 22, y: y + 3 },
            Pos{ x: x + 22, y: y + 4 },
            Pos{ x: x + 22, y: y + 5 },

            Pos{ x: x + 23, y: y + 2 },
            Pos{ x: x + 23, y: y + 6 },

            Pos{ x: x + 25, y: y + 1 },
            Pos{ x: x + 25, y: y + 2 },
            Pos{ x: x + 25, y: y + 6 },
            Pos{ x: x + 25, y: y + 7 },

            Pos{ x: x + 36, y: y + 3 },
            Pos{ x: x + 36, y: y + 4 },
            Pos{ x: x + 37, y: y + 3 },
            Pos{ x: x + 37, y: y + 4 },
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
