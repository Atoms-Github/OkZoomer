use std::path::Path;
use crate::com::Pat;

pub enum Instruction{
    NavTo(Pat),
    Delete(Pat),
    Copy(Pat, Pat),
    Move(Pat, Pat),
    Color(Pat, u8, u8, u8),
}
