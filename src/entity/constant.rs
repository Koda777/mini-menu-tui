#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub const DEFAULT_POSITION: Position = Position { x: 0, y: 0 };
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

#[derive(Debug)]
pub struct TextElement {
    pub position: Position,
    pub name: String
}

impl TextElement {
    pub fn new(position: Position, name: String) -> TextElement {
        TextElement { position, name }
    }
}

#[derive(Debug)]
pub struct CursorElement {
    pub position: Position,
    pub name: String
}

impl CursorElement {
    pub fn new(position: Position, name: String) -> CursorElement {
        CursorElement { position, name }
    }
}