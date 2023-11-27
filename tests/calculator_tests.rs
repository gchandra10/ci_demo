// tests/calculator_tests.rs

//extern crate ci_demo; // Replace 'your_project' with the name of your crate
use ci_demo::calculator;

    #[test]
    fn test_add1() {
        assert!(calculator::add(20.0, 10.0) == 30f32);
    }

    #[test]
    fn test_add2() {
        assert!(calculator::add(10.0, 20.0) == 30f32);
    }

    #[test]
    fn test_add3() {
        assert!(calculator::add(-10.0, 20.0) == 10f32);
    }

    #[test]
    fn test_add4() {
        assert!(calculator::add(10.0, -20.0) == -10f32);
    }


    #[test]
    #[should_panic(expected = "cannot be less")]
    fn test_sub1() {
        assert!(calculator::sub(10.0, 20.0) == -10.0);
    }

    #[test]
    fn test_sub2() {
        assert!(calculator::sub(20.0, 10.0) == 10.0);
    }

    #[test]
    fn test_sub3() {
        assert!(calculator::sub(-20.0, -20.0) == 0.0);
    }

    #[test]
    fn test_mul1() {
        assert!(calculator::mul(20.0, 10.0) == 200.0);
    }

    #[test]
    fn test_mul2() {
        assert!(calculator::mul(20.0, 0.0) == 0.0);
    }

    #[test]
    fn test_mul3() {
        assert!(calculator::mul(0.0, 30.0) == 0.0);
    }

    #[test]
    fn test_mul4() {
        assert!(calculator::mul(-4.0, 2.0) == -8.0);
    }

    #[test]
    fn test_mul5() {
        assert!(calculator::mul(-4.0, -2.0) == 8.0);
    }

    #[test]
    fn test_div1() {
        assert!(calculator::div(20.0, 10.0) == 2.0);
    }

    #[test]
    fn test_div2() {
        assert!(calculator::div(0.0, 20.0) == 0.0);
    }

    #[test]
    #[should_panic(expected = "denominator cannot be zero")]
    fn test_div3() {
        assert!(calculator::div(20.0, 0.0) == 0.0);
    }

    #[test]
    fn test_div4() {
        assert!(calculator::div(-20.0, 2.0) == -10.0);
    }

    #[test]
    fn test_div5() {
        assert!(calculator::div(-20.0, -2.0) == 10.0);
    }
