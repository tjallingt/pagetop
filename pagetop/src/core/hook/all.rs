use super::{HookAction, ActionsHolder};
use crate::LazyStatic;
use crate::util::Handler;

use std::collections::HashMap;
use std::sync::RwLock;

// Registered actions.
static ACTIONS: LazyStatic<RwLock<HashMap<Handler, ActionsHolder>>> = LazyStatic::new(||
    RwLock::new(HashMap::new())
);

pub fn add_action(action: HookAction) {
    let mut actions = ACTIONS.write().unwrap();
    let action_handler = action.handler();
    if let Some(holder) = actions.get_mut(&action_handler) {
        holder.add(action);
    } else {
        actions.insert(action_handler, ActionsHolder::new_with(action));
    }
}

pub fn run_actions<B, F>(action_handler: Handler, f: F)
where
    F: FnMut(&HookAction) -> B,
{
    if let Some(holder) = ACTIONS.read().unwrap().get(&action_handler) {
        holder.iter_map(f)
    }
}
