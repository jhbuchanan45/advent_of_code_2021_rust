use crate::pos::{Adjacent, Pos};
use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

#[derive(Default)]
pub struct Cavern<N, C> {
    pub flash_count: u32,
    queue: VecDeque<Pos<N>>,
    pub charges: Vec<Vec<C>>,
}

pub trait Charge<N> {
    fn charge_pos(&mut self, pos: Pos<N>, ignore_zero: bool);
    fn charge_all(&mut self);
    fn process_queue(&mut self);
    fn flash_pos(&mut self, pos: Pos<N>);
}

impl Index<Pos<usize>> for Cavern<usize, u8> {
    type Output = u8;

    fn index(&self, pos: Pos<usize>) -> &Self::Output {
        &self.charges[pos.y][pos.x]
    }
}

impl IndexMut<Pos<usize>> for Cavern<usize, u8> {
    fn index_mut(&mut self, pos: Pos<usize>) -> &mut Self::Output {
        &mut self.charges[pos.y][pos.x]
    }
}

impl Charge<usize> for Cavern<usize, u8> {
    fn charge_pos(&mut self, pos: Pos<usize>, ignore_zero: bool) {
        if pos.y < self.charges.len() && pos.x < self.charges[0].len() {
            if !(!ignore_zero && self[pos] == 0) {
                self[pos] += 1;

                if self[pos] > 9 {
                    self.queue.push_back(pos)
                }
            }
        }
    }

    fn charge_all(&mut self) {
        for y in 0..self.charges.len() {
            for x in 0..self.charges[0].len() {
                self.charge_pos(Pos { y: y, x: x }, true);
            }
        }
    }

    // items will get added to q multiple times if charged multiple times over 9
    // recursion might be easier lmao
    fn process_queue(&mut self) {
        while self.queue.len() > 0 {
            let cur_pos = self.queue.pop_front().unwrap();
            let flashes = self[cur_pos] / 9;

            self[cur_pos] = 0;
            self.flash_count += flashes as u32;

            for pos in cur_pos.get_adjacent() {
                for _ in 0..flashes {
                    self.charge_pos(pos, false)
                }
            }
        }
    }

    fn flash_pos(&mut self, _pos: Pos<usize>) {
        todo!() // unnecessary??
    }
}
