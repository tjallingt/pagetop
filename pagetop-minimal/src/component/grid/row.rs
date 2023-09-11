use pagetop::prelude::*;

use crate::component::grid;

new_handle!(COMPONENT_GRID_ROW);

actions_for_component!(Row);

#[rustfmt::skip]
#[derive(Default)]
pub struct Row {
    weight    : Weight,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    cols      : TypedComponents<grid::Column>,
    template  : String,
}

impl ComponentTrait for Row {
    fn new() -> Self {
        Row::default().with_classes(ClassesOp::SetDefault, &["row"])
    }

    fn handle(&self) -> Handle {
        COMPONENT_GRID_ROW
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

    fn before_prepare_component(&mut self, cx: &mut Context) {
        run_actions_before_prepare_row(self, cx);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div id=[self.id()] class=[self.classes().get()] {
                (self.columns().prepare(cx))
            }
        })
    }

    fn after_prepare_component(&mut self, cx: &mut Context) {
        run_actions_after_prepare_row(self, cx);
    }
}

impl Row {
    // Row BUILDER.

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
    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &[impl ToString]) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    pub fn with_column(mut self, column: grid::Column) -> Self {
        self.cols.alter(TypedOp::Add(TypedComponent::with(column)));
        self
    }

    #[fn_builder]
    pub fn alter_columns(&mut self, op: TypedOp<grid::Column>) -> &mut Self {
        self.cols.alter(op);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Row GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn columns(&self) -> &TypedComponents<grid::Column> {
        &self.cols
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
