use crate::core::action::{action_ref, run_actions, ActionTrait};
use crate::response::page::action::ActionPage;
use crate::response::page::Page;
use crate::{create_handle, Handle, Weight};

create_handle!(ACTION_AFTER_PREPARE_BODY for Action);

pub struct ActionAfterPrepareBody {
    action: Option<ActionPage>,
    weight: Weight,
}

impl ActionTrait for ActionAfterPrepareBody {
    fn new() -> Self {
        ActionAfterPrepareBody {
            action: None,
            weight: 0,
        }
    }

    fn handle(&self) -> Handle {
        ACTION_AFTER_PREPARE_BODY
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl ActionAfterPrepareBody {
    pub fn with_action(mut self, action: ActionPage) -> Self {
        self.action = Some(action);
        self
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    pub(crate) fn run(&self, page: &mut Page) {
        if let Some(action) = self.action {
            action(page)
        }
    }
}

#[inline(always)]
pub(crate) fn run_actions_after_prepare_body(page: &mut Page) {
    run_actions(ACTION_AFTER_PREPARE_BODY, |action| {
        action_ref::<ActionAfterPrepareBody>(&**action).run(page)
    });
}