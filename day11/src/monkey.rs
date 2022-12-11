pub struct Monkey {
    pub(crate) operation: Box<dyn Fn(u64) -> u64>,
    pub(crate) divisible_by: u8,
    pub(crate) true_destination: u8,
    pub(crate) false_destination: u8,
}
