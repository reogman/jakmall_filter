pub trait Provider {
    fn get_min_sold(&self) -> usize;
    fn get_min_price(&self) -> usize;
    fn get_max_price(&self) -> usize;
    fn get_can_single_co(&self) -> bool;
}
