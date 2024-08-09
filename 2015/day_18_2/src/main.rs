//! Advent of code 2015 day 18 part 2

use std::io::BufRead;

use day_18_1::LightGrid;

fn set_corners_on(grid: &mut LightGrid) {
    let size = grid.size();
    grid.set(0, 0, true);
    grid.set(size - 1, 0, true);
    grid.set(0, size - 1, true);
    grid.set(size - 1, size - 1, true);
}

fn main() {
    let file = std::fs::File::open("../day_18_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut grid = LightGrid::from_lines(reader.lines().map(Result::unwrap)).unwrap();

    set_corners_on(&mut grid);
    for _ in 0..100 {
        grid = grid.step();
        set_corners_on(&mut grid);
    }
    println!("Lights on: {}", grid.count_on());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let mut grid = LightGrid::from_lines(
            r#"
                ##.#.#
                ...##.
                #....#
                ..#...
                #.#..#
                ####.#
            "#
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        )
        .unwrap();
        set_corners_on(&mut grid);

        let mut grid = grid.step();
        set_corners_on(&mut grid);
        let expected = LightGrid::from_lines(
            r#"
                #.##.#
                ####.#
                ...##.
                ......
                #...#.
                #.####
            "#
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        )
        .unwrap();
        assert_eq!(grid, expected);

        let mut grid = grid.step();
        set_corners_on(&mut grid);
        let expected = LightGrid::from_lines(
            r#"
                #..#.#
                #....#
                .#.##.
                ...##.
                .#..##
                ##.###
            "#
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        )
        .unwrap();
        assert_eq!(grid, expected);

        let mut grid = grid.step();
        set_corners_on(&mut grid);
        let expected = LightGrid::from_lines(
            r#"
                #...##
                ####.#
                ..##.#
                ......
                ##....
                ####.#
            "#
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        )
        .unwrap();
        assert_eq!(grid, expected);

        let mut grid = grid.step();
        set_corners_on(&mut grid);
        let expected = LightGrid::from_lines(
            r#"
                #.####
                #....#
                ...#..
                .##...
                #.....
                #.#..#
            "#
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        )
        .unwrap();
        assert_eq!(grid, expected);

        let mut grid = grid.step();
        set_corners_on(&mut grid);
        let expected = LightGrid::from_lines(
            r#"
                ##.###
                .##..#
                .##...
                .##...
                #.#...
                ##...#
            "#
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        )
        .unwrap();
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_loop_step() {
        let mut grid = LightGrid::from_lines(
            r#"
                ##.#.#
                ...##.
                #....#
                ..#...
                #.#..#
                ####.#
            "#
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        )
        .unwrap();

        set_corners_on(&mut grid);
        for _ in 0..5 {
            grid = grid.step();
            set_corners_on(&mut grid);
        }
        assert_eq!(grid.count_on(), 17);
    }
}
