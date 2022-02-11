use enum_map::Enum;

#[derive(Clone, Copy, Enum, Debug)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Dash,
    Attack,
}
