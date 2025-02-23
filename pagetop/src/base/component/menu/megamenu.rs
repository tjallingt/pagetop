use crate::prelude::*;

use super::Group;

type Groups = TypedComponents<Group>;

#[rustfmt::skip]
#[derive(Default)]
pub struct Megamenu {
    weight    : Weight,
    renderable: Renderable,
    id        : OptionId,
    groups    : Groups,
}

impl_handle!(COMPONENT_BASE_MENU_MEGAMENU for Megamenu);

impl ComponentTrait for Megamenu {
    fn new() -> Self {
        Megamenu::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div id=[self.id()] class="pt-menu__groups" {
                (self.groups().render(cx))
            }
        })
    }
}

impl Megamenu {
    // Megamenu BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    pub fn add_group(mut self, group: Group) -> Self {
        self.groups.alter(TypedOp::Add(TypedComponent::with(group)));
        self
    }

    #[fn_builder]
    pub fn alter_groups(&mut self, op: TypedOp<Group>) -> &mut Self {
        self.groups.alter(op);
        self
    }

    // Megamenu GETTERS.

    pub fn groups(&self) -> &Groups {
        &self.groups
    }
}
