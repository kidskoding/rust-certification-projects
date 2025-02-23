pub fn add(a: f64, b: f64) -> f64 {
    a + b
}
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0 as f64 {
        return Err("Cannot divide by 0!".to_string());
    }
    Ok(a / b)
}

pub fn factorial(a: isize) -> Option<isize> {
    if a < 0 {
        return None;
    } else if a == 0 {
        return Some(1);
    }
    Some(a * factorial(a - 1).unwrap())
}

pub fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    for k in 2..n {
        if n % k == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_add() {
        assert!((add(5f64, 5f64) - 10f64).abs() < EPSILON);
        assert!((add(2.2, 2.2) - 4.4).abs() < EPSILON);
    }
    #[test]
    fn test_subtract() {
        assert!((subtract(5f64, 5f64) - 0f64).abs() < EPSILON);
        assert!((subtract(2.2, 3.3) - (-1.1f64)).abs() < EPSILON);
    }
    #[test]
    fn test_multiply() {
        assert!((multiply(2f64, 2f64) - 4f64).abs() < EPSILON);
        assert!((multiply(2.2, 2.2) - 4.84).abs() < EPSILON);
    }
    #[test]
    fn test_divide() {
        assert!(divide(5f64, 5f64).is_ok());
        assert!((divide(5f64, 5f64).unwrap() - 1f64).abs() < EPSILON);

        assert!(divide(5f64, 0f64).is_err());
    }
    
    #[test]
    fn test_factorial() {
        assert!(factorial(-1).is_none());
        assert_eq!(factorial(0).unwrap(), 1);
        assert_eq!(factorial(5).unwrap(), 120);
        assert_eq!(factorial(6).unwrap(), 720);
    }
    
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(5, 10), 5);
        assert_eq!(gcd(1, 13), 1);
        assert_eq!(gcd(-4, -36), -4);
    }
    
    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert!(is_prime(2));
        assert_eq!(is_prime(46), false);
        assert!(is_prime(97));
        assert!(is_prime(13));
        assert_eq!(is_prime(26), false);
    }
}