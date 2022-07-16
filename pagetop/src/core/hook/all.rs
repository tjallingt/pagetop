use crate::Lazy;
use super::{HookAction, HooksHolder};

use std::sync::RwLock;
use std::collections::HashMap;

// Registered actions.
static ACTIONS: Lazy<RwLock<HashMap<&str, HooksHolder>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

pub fn add_hook(hook: HookAction) {
    let mut hmap = ACTIONS.write().unwrap();
    let action_handler = hook.handler();
    if let Some(actions) = hmap.get_mut(action_handler) {
        actions.add(hook);
    } else {
        hmap.insert(action_handler, HooksHolder::new_with(hook));
    }
}

pub fn run_actions<B, F>(action_handler: &str, f: F) where F: FnMut(&HookAction) -> B {
    if let Some(actions) = ACTIONS.read().unwrap().get(action_handler) {
        actions.iter_map(f)
    }
}
