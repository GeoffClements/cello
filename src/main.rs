mod grid;

fn main() {
    let width = 150;
    let mut row = grid::Row::new(width);
    row[width / 2] = true;
    println!("{row}");

    for _ in 0usize..width {
        row = row.step();
        println!("{row}");
    }
}
