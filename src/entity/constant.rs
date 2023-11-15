use crate::entity::status::ResourceType;

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

pub struct TextElement {
    pub position: Position,
    pub name: String,
    pub metadata: Metadata,
}

impl TextElement {
    pub fn new(position: Position, name: String, metadata: Metadata) -> TextElement {
        TextElement { position, name, metadata }
    }
}

#[derive(Debug)]
pub struct CursorElement {
    pub position: Position,
    pub name: String,
}

impl CursorElement {
    pub fn new(position: Position, name: String) -> CursorElement {
        CursorElement { position, name }
    }
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub resource: ResourceType,
    pub details: String
}

impl Metadata {
    pub fn new(resource: ResourceType, details: String) -> Metadata {
        Metadata { resource, details }
    }
}
