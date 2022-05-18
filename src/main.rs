use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::u8;

use simple_error::bail;
use threadpool::ThreadPool;

fn main() {
    let mut boards: Vec<[[u8; 5]; 5]> = Vec::new();
    let mut draw_order = Vec::new();
    let mut prev_board = usize::MAX;

    if let Ok(lines) = read_lines("input.txt") {
        for (i, line) in lines.enumerate() {
            let line = line.unwrap();

            if i == 0 {
                draw_order = line
                    .split(',')
                    .map(|num| num.parse::<u8>().unwrap())
                    .collect();
            } else if (i - 1) % 6 != 0 {
                // only read lines with board on them
                let cur_row = ((i - 1) % 6) - 1;
                let cur_board = (i - 1) / 6;

                if cur_board != prev_board {
                    boards.push([[0; 5]; 5]);
                    prev_board = cur_board;
                }

                let raw_row = line
                    .split_whitespace()
                    .map(|num| num.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>();

                if raw_row.len() != 5 {
                    panic!("Row too long! {:?}", raw_row)
                } else {
                    for (i, num) in raw_row.iter().enumerate() {
                        boards[cur_board][cur_row][i] = *num;
                    }
                }
            }
        }
    }

    // sim games on threads for each boards
    let n_workers = num_cpus::get();
    let pool = ThreadPool::new(n_workers);
    let boards = boards;

    let (tx, rx) = channel();
    let draw_order = Arc::new(draw_order);

    for board in boards.iter() {
        let tx = tx.clone();
        let draw_order = draw_order.clone();
        let board = Arc::new(board.clone());

        pool.execute(move || {
            tx.send(simulate_game(&draw_order, *board).unwrap())
                .expect("channel will be there waiting for the pool");
        });
    }

    // wait for finish and close channel
    drop(tx);
    pool.join();

    // process results
    let mut results = Vec::new();

    for message in rx {
        results.push(message);
    }

    let &worst_result = results.iter().max_by_key(|(turns, _, _)| turns).unwrap();

    let score = score_board(worst_result.2, worst_result.1);

    println!("Score: {}", score);
}

fn simulate_game(
    draw_order: &Vec<u8>,
    board: [[u8; 5]; 5],
) -> simple_error::SimpleResult<(u32, u8, [[u8; 5]; 5])> {
    let mut board = board.clone();

    for (drawn, draw) in draw_order.iter().enumerate() {
        for i_row in 0..board.len() {
            for i_col in 0..board[i_row].len() {
                if board[i_row][i_col] == *draw {
                    board[i_row][i_col] = u8::MAX;

                    // check columns
                    let mut col_check_iter =
                        board.iter().flatten().skip(i_col).step_by(board[0].len());

                    if col_check_iter.all(|val| *val == u8::MAX)
                        || board[i_row].iter().all(|val| *val == u8::MAX)
                    {
                        return Ok((drawn as u32, *draw, board));
                    }

                    // check rows
                }
            }
        }
    }

    bail!("Game didn't end somehow!")
}

// 'marked' means u8::MAX val
fn score_board(board: [[u8; 5]; 5], final_draw: u8) -> u32 {
    let board_sum = board
        .iter()
        .flatten()
        .filter(|val| **val != u8::MAX)
        .map(|val| *val as u32)
        .sum::<u32>();

    board_sum * (final_draw as u32)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
