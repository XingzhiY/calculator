fn mr_cyclic_one_level(m: i32, c3: f64, tolerance: f64, max_iter: usize) -> f64 {
    let mut mr_value = 0.5;  // Initial guess
    for _ in 0..max_iter {
        let new_mr_value = 1.0 - (1.0 - 1.0 / c3).powf(m as f64 * mr_value);
        if (new_mr_value - mr_value).abs() < tolerance {
            println!("< tolerance");
            return new_mr_value;
        }
        mr_value = new_mr_value;
    }

    mr_value  // Return the last computed value
}
fn mr_cyclic_two_level(m: i32, c2: f64, c3: f64, tolerance: f64, max_iter: usize) -> f64 {
    let mut mr_value = 0.5;  // Initial guess
    for _ in 0..max_iter {
        let new_mr_value = 1.0 - (1.0 - 1.0 / c3).powf(m as f64 * mr_value);
        if (new_mr_value - mr_value).abs() < tolerance {
            println!("< tolerance");
            return new_mr_value;
        }
        mr_value = new_mr_value;
    }

    mr_value  // Return the last computed value
}

fn mr_origin(m: i32, c2: i32, c3: f64, tolerance: f64, max_iter: usize) -> f64 {
    if m <= c2 {
        println!("m <= c2");
        return 0.0;  // Base case to avoid invalid computations
    }

    let mut mr_value = 0.5;  // Initial guess
    for _ in 0..max_iter {
        let mut sum = 0.0;
        for i in 1..=(m - c2) {
            sum += (2 * i - 1) as f64;
        }
        sum *= mr_value;
        let new_mr_value = 1.0 - (1.0 - 1.0 / c3).powf(1.0 / (m - c2) as f64 * sum);
        if (new_mr_value - mr_value).abs() < tolerance {
            println!("< tolerance");
            return new_mr_value;
        }
        mr_value = new_mr_value;
    }

    mr_value  // Return the last computed value
}

fn mr_first(m: i32, c2: i32, c3: f64, tolerance: f64, max_iter: usize) -> f64 {
    if m <= c2 {
        println!("m <= c2");
        return 0.0;  // Base case to avoid invalid computations
    }

    let mut mr_value = 0.5;  // Initial guess
    for _ in 0..max_iter {
        let mut sum = 0.0;
        for i in 1..=(m - c2) {
            sum += (2 * i - 1) as f64;
        }
        sum *= mr_value;
        let new_mr_value = 1.0 - (1.0 - 1.0 / c3).powf(1.0 / (m) as f64 * sum);
        if (new_mr_value - mr_value).abs() < tolerance {
            println!("< tolerance");
            return new_mr_value;
        }
        mr_value = new_mr_value;
    }

    mr_value  // Return the last computed value
}


fn mr_ancient(m: i32, c2: i32, c3: f64, tolerance: f64, max_iter: usize) -> f64 {
    if m <= c2 {
        println!("m <= c2");
        return 0.0;  // Base case to avoid invalid computations
    }

    let mut mr_value = 0.5;  // Initial guess
    for _ in 0..max_iter {
        let mut sum = 0.0;
        for i in 1..=(m) {
            sum += (2 * i - 1) as f64;
        }
        sum *= mr_value;
        let new_mr_value = 1.0 - (1.0 - 1.0 / c3).powf(1.0 / (m) as f64 * sum);
        if (new_mr_value - mr_value).abs() < tolerance {
            println!("< tolerance");
            return new_mr_value;
        }
        mr_value = new_mr_value;
    }

    mr_value  // Return the last computed value
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_mr_cyclic_one_level() {
        let m = 40;
        let c3 = 20.;
        let tolerance = 1e-6;
        let max_iter = 1000;

        let result = mr_cyclic_one_level(m, c3, tolerance, max_iter);
        println!("Test Result: {}", result);
        assert!(result >= 0.0);
    }
    #[test]
    fn test_mr_cyclic_two_level() {

        let m = 256*1024;
        let c2=1.0 * 1024.;
        let c3 = 96.*1024.;
        let tolerance = 1e-6;
        let max_iter = 1000;

        let result = mr_cyclic_two_level(m,c2, c3, tolerance, max_iter);
        println!("Test Result: {}", result);
        assert!(result >= 0.0);
    }
    #[test]
    fn test_mr_cyclic_two_level_8() {

        let m = 256*1024/8;
        let c2=1.0 * 1024./8.;
        let c3 = 96.*1024./8.;
        let tolerance = 1e-6;
        let max_iter = 1000;

        let result = mr_cyclic_two_level(m,c2, c3, tolerance, max_iter);
        println!("Test Result: {}", result);
        assert!(result >= 0.0);
    }

    #[test]
    fn test_mr_ancient() {
        let m = 128;
        let c2 = 1;
        let c3 = 96.;
        let tolerance = 1e-6;
        let max_iter = 1000;

        let result = mr_ancient(m, c2, c3, tolerance, max_iter);
        println!("Test Result: {}", result);
        assert!(result >= 0.0);
    }

    #[test]
    fn test_mr_origin() {
        let m = 128;
        let c2 = 1;
        let c3 = 96.;
        let tolerance = 1e-6;
        let max_iter = 1000;

        let result = mr_origin(m, c2, c3, tolerance, max_iter);
        println!("Test Result: {}", result);
        assert!(result >= 0.0);
    }

    #[test]
    fn test_mr_first() {
        let m = 4;
        let c2 = 1;
        let c3 = 3.;
        let tolerance = 1e-10;
        let max_iter = 1000;

        let result = mr_first(m, c2, c3, tolerance, max_iter);
        println!("Test Result: {}", result);
        assert!(result >= 0.0);
    }
}
