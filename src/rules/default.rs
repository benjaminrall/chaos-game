use std::collections::VecDeque;
use chaos_game_macros::rule;
use crate::types::Vertex;

/// Default rule that treats every vertex as valid.
#[rule("default")]
fn default(_previous_points: &VecDeque<&Vertex>, _new_point: &Vertex) -> bool {
    true
}