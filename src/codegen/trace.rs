
thread_local! {
    pub(crate) static TRACE_SEED: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
}

pub(crate) fn get_and_increment_seed() -> u64 {
    TRACE_SEED.with_borrow_mut(|seed| {
        let tmp = *seed;
        *seed += 1;
        tmp
    })
}
