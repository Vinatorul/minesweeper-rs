#[derive(PartialEq)]
pub enum ParamType {
    Width,
    Height,
    Mines
}

pub enum MoveDestination {
    Up,
    Down,
    Left,
    Right
}

pub enum GameEndState {
    Win,
    Lose
}
