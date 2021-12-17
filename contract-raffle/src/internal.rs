use crate::*;

pub(crate) fn assert_initialized() {
    assert!(!env::state_exists(), "Already initialized");
}