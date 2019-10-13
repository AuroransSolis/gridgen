use std::env;

fn main() {
    let size = env::args().skip(1).next().unwrap().parse::<i32>().unwrap();
    if size > 128 {
        panic!("Size must be less than or equal to 128.");
    }
    let grid = create_grid(size);
    print_grid(&grid, size as usize);
}

fn print_grid(grid: &[u8], side_len: usize) {
    for row in 0..side_len {
        println!("{:?}", &grid[row * side_len..(row + 1) * side_len]);
    }
}

#[inline]
fn magic_func(x: i32, y: i32, depth: i32) -> i32 {
    let mut val = 1;
    for i in 0..depth {
        val += (((x >> i) + (y >> i)) & 1) << i;
    }
    val
}

pub fn create_grid(size: i32) -> Vec<u8> {
    let capacity = (size * size) as usize;
    let mut grid = Vec::with_capacity(capacity);
    unsafe {
	grid.set_len(capacity);
    }

    let depth = (size-1).leading_zeros() + 1;

    let mut index = 0;
    for y in 0..size {
        for x in 0..size {
            grid[index] = magic_func(x, y, depth as i32) as u8;
            index += 1;
        }
    }
    grid
}
