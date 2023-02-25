use bevy::prelude::Component;
use enum_index::EnumIndex;
use enum_index_derive::EnumIndex;
extern crate enum_index;

#[derive(Component, EnumIndex)]
pub enum MenuButton {
    WithoutWall(u32),
    ExteriorWall(u32),
    VerticalWall(u32),
    HorizontalWall(u32),
    VerticalAndHorizontalWall(u32),
    Quit,
}