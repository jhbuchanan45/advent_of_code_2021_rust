use std::ops::RangeInclusive;

pub fn draw_line(grid: &mut Vec<Vec<i32>>, point1: (u32, u32), point2: (u32, u32)) {
    if point1.0 == point2.0 || point1.1 == point2.1 {
        draw_straight_line(grid, point1, point2);
    } else {
        draw_diagonal_line(grid, point1, point2);
    }
}

fn draw_diagonal_line(grid: &mut Vec<Vec<i32>>, mut point1: (u32, u32), mut point2: (u32, u32)) {
    // sort points to prefer drawing from left to right
    if point2.0 < point1.0 {
        (point1, point2) = (point2, point1)
    }

    let (point1, point2) = (point1, point2); // make immutable

    // bresenham's line drawing algorithm
    let dx2: i32 = (point2.0 as i32 - point1.0 as i32) * 2;
    let dy2: i32 = (point2.1 as i32 - point1.1 as i32) * 2;
    let mut diff: i32 = 0;
    let mut y = point1.1;

    for x in point1.0..=point2.0 {
        grid[y as usize][x as usize] += 1;

        if x < point2.0 { // avoid overflow error on edge cases
            diff += dy2;

            if dy2 >= 0 && diff > 0 {
                diff -= dx2;
                y += 1;
            } else if dy2 < 0 && diff < 0 {
                diff -= dx2;
                y -= 1;
            }
        }
    }
}

fn draw_straight_line(grid: &mut Vec<Vec<i32>>, point1: (u32, u32), point2: (u32, u32)) {
    if point1.0 == point2.0 {
        // line along x
        for y in min_to_max_iter(point1.1, point2.1) {
            grid[y as usize][point1.0 as usize] += 1;
        }
    } else if point1.1 == point2.1 {
        // line along y
        for x in min_to_max_iter(point1.0, point2.0) {
            grid[point1.1 as usize][x as usize] += 1;
        }
    }
}

fn min_to_max_iter<T>(a: T, b: T) -> RangeInclusive<T>
where
    T: Ord,
{
    if a < b {
        return a..=b;
    } else {
        return b..=a;
    }
}
