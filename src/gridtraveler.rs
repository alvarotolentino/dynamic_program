use std::collections::HashMap;

/**
 * Returns the number of ways to travel from the top left to the bottom right of a grid using memoization.
 */
pub fn grid_traveler(cache: &mut HashMap<String, usize>, x_axis: i32, y_axis: i32) -> usize {
    let key = format!("{},{}", x_axis, y_axis);

    if x_axis == 1 && y_axis == 1 {
        return 1;
    }
    if x_axis == 0 || y_axis == 0 {
        return 0;
    }

    let y = grid_traveler(cache, y_axis - 1, x_axis);
    let x = grid_traveler(cache, y_axis, x_axis - 1);
    cache.insert(key.to_string(), x + y);

    match cache.get(&key) {
        Some(x) => *x,
        None => Default::default(),
    }
}

/**
 * Returns the number of ways to travel from the top left to the bottom right of a grid using tabulation.
 */
pub fn grid_traveler_tab(y_axis: usize, x_axis: usize) -> usize {
    let mut table: Vec<Vec<usize>> = vec![vec![0; x_axis + 1]; y_axis + 1];
    table[1][1] = 1;
    for i in 0..=y_axis {
        for j in 0..=x_axis {
            let current = table[i][j];
            if j < x_axis {
                table[i][j + 1] += current;
            }
            if i < y_axis {
                table[i + 1][j] += current;
            }
        }
    }
    table[y_axis][x_axis]
}

#[cfg(test)]
mod tests {
    use crate::gridtraveler::grid_traveler_tab;

    use super::grid_traveler;
    use std::collections::HashMap;
    #[test]
    fn grid_traveler_1_x_1() {
        let cache = &mut HashMap::new();
        let result = grid_traveler(cache, 1, 1);
        assert_eq!(result, 1, "Result: {}", result)
    }
    #[test]
    fn grid_traveler_2_x_3() {
        let cache = &mut HashMap::new();
        let result = grid_traveler(cache, 2, 3);
        assert_eq!(result, 3, "Result: {}", result)
    }
    #[test]
    fn grid_traveler_3_x_3() {
        let cache = &mut HashMap::new();
        let result = grid_traveler(cache, 3, 3);
        assert_eq!(result, 6, "Result: {}", result)
    }

    #[test]
    fn grid_traveler_tabulation_3_x_3() {
        let result = grid_traveler_tab(3, 3);
        assert_eq!(result, 6, "Result: {}", result)
    }
    #[test]
    fn grid_traveler_tabulation_18_x_18() {
        let result = grid_traveler_tab(18, 18);
        assert_eq!(result, 2333606220, "Result: {}", result)
    }
}
