use enum_map::Enum;

#[derive(Clone, PartialEq, Enum, Debug)]
pub enum DebugInfo {
    DebugModeNotice = 0, // Always goes first
    Fps,
    Player,
    Entities,
    Step,
    PapaTicker,
    MousePosition,
    AimPosition,
    Temp,
    Moving,
}
