//! Advent of code 2015 day 25 part 1

fn row_column_to_index(row: u32, column: u32) -> u32 {
    // on any given diagonal, the sum of the row and column is constant
    let diagonal = row + column - 1;

    // the area of the triangle before the diagonal
    let area_before = diagonal * (diagonal - 1) / 2;

    // the position of the cell on the diagonal
    let position_on_diagonal = column;

    area_before + position_on_diagonal
}

fn code(row: u32, column: u32) -> u64 {
    // we could do modular exponentiation here, but the number of iterations is small enough that
    // we can just loop
    let mut value = 20_151_125;
    for _ in 1..row_column_to_index(row, column) {
        value = (value * 252_533) % 33_554_393;
    }

    value
}

fn main() {
    let row = 2978;
    let column = 3083;

    let value = code(row, column);
    println!("Code: {value}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_column_to_index() {
        assert_eq!(row_column_to_index(1, 1), 1);
        assert_eq!(row_column_to_index(2, 1), 2);
        assert_eq!(row_column_to_index(3, 1), 4);

        assert_eq!(row_column_to_index(1, 2), 3);
        assert_eq!(row_column_to_index(2, 2), 5);
        assert_eq!(row_column_to_index(3, 2), 8);

        assert_eq!(row_column_to_index(5, 2), 17);
        assert_eq!(row_column_to_index(2, 5), 20);
    }

    #[test]
    fn test_code() {
        assert_eq!(code(1, 1), 20151125);
        assert_eq!(code(2, 1), 31916031);
        assert_eq!(code(3, 1), 16080970);

        assert_eq!(code(1, 2), 18749137);
        assert_eq!(code(2, 2), 21629792);
        assert_eq!(code(3, 2), 8057251);

        assert_eq!(code(5, 2), 17552253);
        assert_eq!(code(2, 5), 15514188);
    }
}
