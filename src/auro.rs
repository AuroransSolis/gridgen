use std::env;

fn main() {
    let size = env::args().skip(1).next().unwrap().parse::<usize>().unwrap();
    if size > 128 {
        panic!("Size must be less than or equal to 128.");
    }
    let grid = gen_grid_bit_magic(size);
    print_grid(&grid, size);
}

fn print_grid(grid: &[u8], side_len: usize) {
    for row in 0..side_len {
        println!("{:?}", &grid[row * side_len..(row + 1) * side_len]);
    }
}

pub fn gen_grid_bit_magic(size: usize) -> Vec<u8> {
    let mut grid = Vec::with_capacity(size * size);
    unsafe {
	grid.set_len(size * size);
    }
    grid[0] = 1;
    let mut markers = vec![0; size * 2];
    markers[0] = 1;
    markers[size] = 1;
    for len in  1..size {
        let new_tr = get_mark_smallest(&mut markers, size, 0, len);
        let new_bl = get_mark_smallest(&mut markers, size, len, 0);
        grid[len] = new_tr;
        grid[len * size] = new_bl;
        for n in 1..len {
            let new_r = get_mark_smallest(&mut markers, size, n, len);
            let new_b = get_mark_smallest(&mut markers, size, len, n);
            grid[n * size + len] = new_r;
            grid[len * size + n] = new_b;
        }
	let new_br = get_mark_smallest(&mut markers, size, len, len);
        grid[len * size + len] = new_br;
    }
    grid
}

#[inline]
fn get_mark_smallest(markers: &mut [u128], size: usize, i: usize, j: usize) -> u8 {
    let new = (!(markers[j] | markers[size + i])).trailing_zeros() as u8;
    markers[j] |= 1 << new as u128;
    markers[size + i] |= 1 << new as u128;
    new + 1
}
