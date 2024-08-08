#[derive(Debug, PartialEq, Eq)]
pub struct LightGrid {
    lights: Vec<Vec<bool>>,
}

impl LightGrid {
    /// Creates a new `LightGrid` from the given lines.
    ///
    /// # Arguments
    ///
    /// * `lines` - An iterator over the lines of the grid.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `LightGrid` if the input is valid.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the input contains invalid characters (not '#' or '.').
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Result<Self, &'static str> {
        let lights = lines
            .map(|line| {
                line.as_ref()
                    .chars()
                    .map(|c| match c {
                        '#' => Some(true),
                        '.' => Some(false),
                        _ => None,
                    })
                    .collect::<Option<Vec<bool>>>()
            })
            .collect::<Option<Vec<Vec<bool>>>>();
        lights.map(|lights| Self { lights }).ok_or("Invalid input")
    }

    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.lights.get(y).and_then(|row| row.get(x)).copied()
    }

    fn get_neighbor(&self, x: usize, y: usize, offset_x: isize, offset_y: isize) -> Option<bool> {
        let x = isize::try_from(x).unwrap() + offset_x;
        let y = isize::try_from(y).unwrap() + offset_y;
        if x < 0 || y < 0 {
            return None;
        }
        self.get(usize::try_from(x).unwrap(), usize::try_from(y).unwrap())
    }

    fn count_on_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for offset_x in -1..=1 {
            for offset_y in -1..=1 {
                if offset_x == 0 && offset_y == 0 {
                    continue;
                }
                if self.get_neighbor(x, y, offset_x, offset_y) == Some(true) {
                    count += 1;
                }
            }
        }
        count
    }

    #[must_use]
    pub fn step(&self) -> Self {
        let mut lights = self.lights.clone();
        lights.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, light)| {
                let count = self.count_on_neighbors(x, y);
                *light = matches!((*light, count), (true, 2..=3) | (false, 3));
            });
        });

        Self { lights }
    }

    #[must_use]
    pub fn count_on(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().filter(|&&light| light).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
        .#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..
    "#;

    #[test]
    fn test_from_lines() {
        let grid =
            LightGrid::from_lines(INPUT.lines().map(str::trim).filter(|line| !line.is_empty()))
                .unwrap();

        assert_eq!(grid.get(0, 0), Some(false));
        assert_eq!(grid.get(1, 0), Some(true));
        assert_eq!(grid.get(0, 1), Some(false));
    }

    #[test]
    fn test_get_neighbor() {
        let grid =
            LightGrid::from_lines(INPUT.lines().map(str::trim).filter(|line| !line.is_empty()))
                .unwrap();
        assert_eq!(grid.get_neighbor(0, 0, 0, 0), Some(false));
        assert_eq!(grid.get_neighbor(0, 0, 1, 0), Some(true));
        assert_eq!(grid.get_neighbor(0, 0, 0, 1), Some(false));
        assert_eq!(grid.get_neighbor(0, 0, 1, 1), Some(false));
        assert_eq!(grid.get_neighbor(0, 0, -1, 0), None);
        assert_eq!(grid.get_neighbor(0, 0, 0, -1), None);
        assert_eq!(grid.get_neighbor(0, 0, -1, -1), None);
    }

    #[test]
    fn test_count_on_neighbors() {
        let grid =
            LightGrid::from_lines(INPUT.lines().map(str::trim).filter(|line| !line.is_empty()))
                .unwrap();
        assert_eq!(grid.count_on_neighbors(0, 0), 1);
        assert_eq!(grid.count_on_neighbors(1, 0), 0);
        assert_eq!(grid.count_on_neighbors(3, 4), 4);
    }

    #[test]
    fn test_step() {
        let grid =
            LightGrid::from_lines(INPUT.lines().map(str::trim).filter(|line| !line.is_empty()))
                .unwrap();

        let grid = grid.step();
        let expected = LightGrid::from_lines(
            r#"
                ..##..
                ..##.#
                ...##.
                ......
                #.....
                #.##..
            "#
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        )
        .unwrap();
        assert_eq!(grid, expected);

        let grid = grid.step();
        let expected = LightGrid::from_lines(
            r#"
                ..###.
                ......
                ..###.
                ......
                .#....
                .#....
            "#
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        )
        .unwrap();
        assert_eq!(grid, expected);

        let grid = grid.step();
        let expected = LightGrid::from_lines(
            r#"
                ...#..
                ......
                ...#..
                ..##..
                ......
                ......
            "#
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        )
        .unwrap();
        assert_eq!(grid, expected);

        let grid = grid.step();
        let expected = LightGrid::from_lines(
            r#"
                ......
                ......
                ..##..
                ..##..
                ......
                ......
            "#
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        )
        .unwrap();
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_count_on() {
        let grid = LightGrid::from_lines(
            r#"
                ......
                ......
                ..##..
                ..##..
                ......
                ......
            "#
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty()),
        )
        .unwrap();
        assert_eq!(grid.count_on(), 4);
    }

    #[test]
    fn test_fold_step() {
        let grid =
            LightGrid::from_lines(INPUT.lines().map(str::trim).filter(|line| !line.is_empty()))
                .unwrap();
        let grid = (0..4).fold(grid, |grid, _| grid.step());
        assert_eq!(grid.count_on(), 4);
    }
}
