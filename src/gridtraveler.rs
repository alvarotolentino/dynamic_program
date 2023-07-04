use std::collections::HashMap;

pub fn grid_traveler(x_axis: i32, y_axis: i32, cache: &mut HashMap<String, i32>) -> i32 {
    let key = format!("{},{}", x_axis, y_axis);

    if x_axis == 1 && y_axis == 1 {
        return 1;
    }
    if x_axis == 0 || y_axis == 0 {
        return 0;
    }

    let y = grid_traveler(y_axis - 1, x_axis, cache);
    let x = grid_traveler(y_axis, x_axis - 1, cache);
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
        let memo = &mut HashMap::new();
        let result = grid_traveler(1, 1, memo);
        assert_eq!(result, 1)
    }
    #[test]
    fn grid_traveler_2_x_3() {
        let memo = &mut HashMap::new();
        let result = grid_traveler(2, 3, memo);
        assert_eq!(result, 3)
    }
}
