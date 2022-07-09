#[derive(Copy, Clone)]
pub struct Pos<N> {
    pub y: N,
    pub x: N,
}

pub trait Adjacent {
    type N;
    fn get_adjacent(&self) -> Vec<Pos<Self::N>>;
}

impl Adjacent for Pos<usize> {
    type N = usize;

    fn get_adjacent(&self) -> Vec<Pos<usize>> {
        let mut adjacents = Vec::new();

        for y in [-1, 0, 1] {
            for x in [-1, 0, 1] {
                if !(y == 0 && x == 0) {
                    if !(y == -1 && self.y == 0) && !(x == -1 && self.x == 0) {
                        // omit up one row at top
                        adjacents.push(Pos {
                            y: (self.y as i16 + y) as usize,
                            x: (self.x as i16 + x) as usize,
                        })
                    }
                }
            }
        }

        adjacents
    }
}
