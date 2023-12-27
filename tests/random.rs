mod random {
    pub fn init() {
        unsafe { srand(time(std::ptr::null()) as u32) };
    }

    pub fn gen() -> i32 {
        unsafe { rand() }
    }

    #[link(name = "c")]
    extern "C" {
        fn time(timer: *const u64) -> u64;
        fn srand(seed: u32);
        fn rand() -> i32;
    }
}

#[test]
fn test_random() {
    random::init();
    assert_ne!(random::gen(), random::gen());
}
