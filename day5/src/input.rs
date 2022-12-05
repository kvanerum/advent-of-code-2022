pub(crate) struct Input {
    pub(crate) stacks: Vec<Vec<char>>,
    pub(crate) moves: Vec<Move>,
}

pub(crate) struct Move {
    pub(crate) amount: u8,
    pub(crate) from: u8,
    pub(crate) to: u8,
}
