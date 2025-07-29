use std::collections::VecDeque;
use chaos_game_macros::rule;
use crate::types::Vertex;

/// Rule that does not allow the same vertex to be chosen twice consecutively.
#[rule("star", history = 2)]
fn no_repeats(previous_points: &VecDeque<&Vertex>, new_point: &Vertex) -> bool {
    if previous_points.len() < 2 {
        return true;
    }

    if previous_points[0].index != previous_points[1].index {
        return true;
    }

    if new_point.index == (previous_points[0].index + 1) % new_point.sides {
        return false;
    }

    if new_point.index == (previous_points[0].index + new_point.sides - 1) % new_point.sides {
        return false;
    }

    return true;
}