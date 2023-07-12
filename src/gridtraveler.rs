use std::collections::HashMap;

/**
 * Returns the number of ways to travel from the top left to the bottom right of a grid.
 */
pub fn grid_traveler(cache: &mut HashMap<String, i32>, x_axis: i32, y_axis: i32) -> i32 {
    let key = format!("{},{}", x_axis, y_axis);

    if x_axis == 1 && y_axis == 1 {
        return 1;
    }
    if x_axis == 0 || y_axis == 0 {
        return 0;
    }

    let y =  grid_traveler(cache, y_axis - 1, x_axis);
    let x = grid_traveler(cache, y_axis, x_axis - 1);
    cache.insert(key.to_string(), x + y);

    match cache.get(&key) {
        Some(x) => *x,
        None => Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::grid_traveler;
    use std::collections::HashMap;
    #[test]
    fn grid_traveler_1_x_1() {
        let cache = &mut HashMap::new();
        let result = grid_traveler(cache, 1, 1);
        assert_eq!(result, 1)
    }
    #[test]
    fn grid_traveler_2_x_3() {
        let cache = &mut HashMap::new();
        let result = grid_traveler(cache, 2, 3);
        assert_eq!(result, 3)
    }
}
