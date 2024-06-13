pub fn add(a: f32, b: f32) -> f32 {
    a + b
}

pub fn sub(a: f32, b: f32) -> f32 {
    if a < b {
        panic!("first value cannot be less than second value");
    } else {
        a - b
    }
}

pub fn mul(a: f32, b: f32) -> f32 {
    a * b
}

pub fn div(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("denominator cannot be zero")
    } else {
        a / b
    }
}

// #[cfg(test)] // Only compiles when running tests
// mod tests {

//     use super::*;

//     #[test]
//     fn test_add1() {
//         assert!(add(20.0, 10.0) == 30f32);
//     }

//     #[test]
//     fn test_add2() {
//         assert!(add(10.0, 20.0) == 30f32);
//     }

//     #[test]
//     fn test_add3() {
//         assert!(add(-10.0, 20.0) == 10f32);
//     }

//     #[test]
//     fn test_add4() {
//         assert!(add(10.0, -20.0) == -10f32);
//     }


//     #[test]
//     #[should_panic(expected = "cannot be less")]
//     fn test_sub1() {
//         assert!(sub(10.0, 20.0) == -10.0);
//     }

//     #[test]
//     fn test_sub2() {
//         assert!(sub(20.0, 10.0) == 10.0);
//     }

//     #[test]
//     fn test_sub3() {
//         assert!(sub(-20.0, -20.0) == 0.0);
//     }

//     #[test]
//     fn test_mul1() {
//         assert!(mul(20.0, 10.0) == 200.0);
//     }

//     #[test]
//     fn test_mul2() {
//         assert!(mul(20.0, 0.0) == 0.0);
//     }

//     #[test]
//     fn test_mul3() {
//         assert!(mul(0.0, 30.0) == 0.0);
//     }

//     #[test]
//     fn test_mul4() {
//         assert!(mul(-4.0, 2.0) == -8.0);
//     }

//     #[test]
//     fn test_mul5() {
//         assert!(mul(-4.0, -2.0) == 8.0);
//     }

//     #[test]
//     fn test_div1() {
//         assert!(div(20.0, 10.0) == 2.0);
//     }

//     #[test]
//     fn test_div2() {
//         assert!(div(0.0, 20.0) == 0.0);
//     }

//     #[test]
//     #[should_panic(expected = "denominator cannot be zero")]
//     fn test_div3() {
//         assert!(div(20.0, 0.0) == 0.0);
//     }

//     #[test]
//     fn test_div4() {
//         assert!(div(-20.0, 2.0) == -10.0);
//     }

//     #[test]
//     fn test_div5() {
//         assert!(div(-20.0, -2.0) == 10.0);
//     }
// }

