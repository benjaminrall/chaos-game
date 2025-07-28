use std::collections::VecDeque;
use chaos_game_macros::rule;
use crate::types::Vertex;

/// Rule that does not allow the same vertex to be chosen twice consecutively.
#[rule("no-repeats", history = 1)]
fn no_repeats(previous_points: &VecDeque<Vertex>, new_point: &Vertex) -> bool {
    if previous_points.len() == 0 {
        return true;
    }
    previous_points[0].index != new_point.index
}