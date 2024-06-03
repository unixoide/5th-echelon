use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;

use super::datatypes::NetFiniteState;

pub fn state_ptr_to_name(state: *mut NetFiniteState) -> String {
    if state.is_null() {
        "NULL".to_owned()
    } else {
        let state_ref = unsafe { &mut *state };
        format!(
            "{}(inst={:?}, vtable={:?}, id={:x})",
            state_ref.get_state_name(),
            state,
            state_ref.vtable,
            state_ref.get_state_id(),
        )
    }
}

const HASHES: &str = include_str!("../../maphashes.txt");
static PARSED_HASHES: OnceLock<Arc<HashMap<usize, &'static str>>> = OnceLock::new();

pub fn hashes() -> Arc<HashMap<usize, &'static str>> {
    let hashes = PARSED_HASHES.get_or_init(|| {
        Arc::new(
            HASHES
                .split('\n')
                .filter_map(|line| line.split_once('\t'))
                .filter_map(|(txt, id)| Some((usize::from_str_radix(id.trim_end(), 16).ok()?, txt)))
                .collect(),
        )
    });
    Arc::clone(hashes)
}

pub fn id_to_name(id: usize) -> String {
    if let Some(name) = hashes().get(&id) {
        format!("{name}(id={id:x})")
    } else {
        format!("{id:x}")
    }
}

pub struct SomeOrQuestionmark<T>(pub Option<T>);

impl<T: std::fmt::Debug> std::fmt::Debug for SomeOrQuestionmark<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(t) = &self.0 {
            write!(f, "{t:?}")
        } else {
            write!(f, "???")
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for SomeOrQuestionmark<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(t) = &self.0 {
            write!(f, "{t}")
        } else {
            write!(f, "???")
        }
    }
}
