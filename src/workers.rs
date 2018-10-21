pub use self::threaded::Workers;

mod threaded {
    use block::Matrix;

    /// Holds the number of lanes.
    pub struct Workers(u32);

    impl Workers {
        #[inline(always)]
        pub fn new(lanes: u32) -> Workers {
            Workers(lanes)
        }

        #[inline(always)]
        pub fn map<F>(&mut self, blocks: &mut Matrix, fill_slice: &F)
        where
            F: Fn(&mut Matrix, u32) + Sync,
        {
            for lane in 0..self.0 {
                fill_slice(blocks, lane);
            }
        }
    }
}
