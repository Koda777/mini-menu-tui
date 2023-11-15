#[derive(Clone, Copy, Debug)]
pub enum EventStatus {
    Init,
    ResetCursor,
    CursorUpward,
    CursorDownward,
    Back,
    Enter
}

#[derive(Clone, Copy, Debug)]
pub enum ResourceType {
    None,
    Directory,
    Workdir,
    Command
}
