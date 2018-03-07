fn main() {
    const ROWS:    usize = 20;
    const COLUMNS: usize = 20;
    let mut grid: [[u64; COLUMNS]; ROWS] = [[0; COLUMNS]; ROWS];

    for r in 0..ROWS {
        for c in 0..COLUMNS {
            if r == 0 || c == 0 { 
                grid[r][c] = 1;
            } else { 
                grid[r][c] += grid[r-1][c] + grid[r][c-1];
            }
        }
    }
    println!("Total paths = {}", grid[ROWS-1][COLUMNS-1]);
}
