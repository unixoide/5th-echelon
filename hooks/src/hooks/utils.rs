use std::collections::HashMap;

use super::datatypes::NetFiniteState;

pub fn state_ptr_to_name(state: *mut NetFiniteState) -> String {
    if state.is_null() {
        "NULL".to_owned()
    } else {
        let state_ref = unsafe { &mut *state };
        format!(
            "{}(inst={:?}, vtable={:?})",
            state_ref.get_state_name(),
            state,
            state_ref.vtable
        )
    }
}

const HASHES: &str = include_str!("../../maphashes.txt");

pub fn hashes() -> HashMap<usize, &'static str> {
    HASHES
        .split('\n')
        .filter_map(|line| line.split_once('\t'))
        .filter_map(|(txt, id)| Some((usize::from_str_radix(id.trim_end(), 16).ok()?, txt)))
        .collect()
}
