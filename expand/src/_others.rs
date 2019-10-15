// example which fails to compile (on type checking)
//    #[parameterized(v = { 'a', 'z' })]
//    fn my_test(v: u8) {}

// example with multiple inputs
//    #[parameterized(b1 = {
//        true, false
//    }, b2 = {
//        false, true
//    })]
//    fn my_bool_test(b1: bool, b2: bool) {
//        assert_ne!(b1, b2);
//    }
