use crate::prelude::*;

#[rustfmt::skip]
#[derive(Default)]
pub struct Item {
    weight       : Weight,
    renderable   : Renderable,
    id           : OptionId,
    item_classes : OptionClasses,
    inner_classes: OptionClasses,
    item_grow    : flex::ItemGrow,
    item_shrink  : flex::ItemShrink,
    item_size    : flex::ItemSize,
    item_offset  : flex::ItemOffset,
    item_align   : flex::ItemAlign,
    stuff        : ArcComponents,
}

impl_handle!(COMPONENT_BASE_FLEX_ITEM for Item);

impl ComponentTrait for Item {
    fn new() -> Self {
        Item::default()
            .with_item_classes(ClassesOp::Add, "pt-flex__item")
            .with_inner_classes(ClassesOp::Add, "pt-flex__item-inner")
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
        let order = match self.weight() {
            0 => None,
            _ => Some(concat_string!("order: ", self.weight().to_string(), ";")),
        };
        PrepareMarkup::With(html! {
            div id=[self.id()] class=[self.item_classes().get()] style=[order] {
                div class=[self.inner_classes().get()] {
                    (self.components().render(cx))
                }
            }
        })
    }
}

impl Item {
    // Item BUILDER.

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

    #[fn_builder]
    pub fn alter_item_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.item_classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_inner_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.inner_classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_grow(&mut self, grow: flex::ItemGrow) -> &mut Self {
        self.item_classes.alter_value(
            ClassesOp::Replace(self.item_grow.to_string()),
            grow.to_string(),
        );
        self.item_grow = grow;
        self
    }

    #[fn_builder]
    pub fn alter_shrink(&mut self, shrink: flex::ItemShrink) -> &mut Self {
        self.item_classes.alter_value(
            ClassesOp::Replace(self.item_shrink.to_string()),
            shrink.to_string(),
        );
        self.item_shrink = shrink;
        self
    }

    #[fn_builder]
    pub fn alter_size(&mut self, size: flex::ItemSize) -> &mut Self {
        self.item_classes.alter_value(
            ClassesOp::Replace(self.item_size.to_string()),
            size.to_string(),
        );
        self.item_size = size;
        self
    }

    #[fn_builder]
    pub fn alter_offset(&mut self, offset: flex::ItemOffset) -> &mut Self {
        self.item_classes.alter_value(
            ClassesOp::Replace(self.item_offset.to_string()),
            offset.to_string(),
        );
        self.item_offset = offset;
        self
    }

    #[fn_builder]
    pub fn alter_align(&mut self, align: flex::ItemAlign) -> &mut Self {
        self.item_classes.alter_value(
            ClassesOp::Replace(self.item_align.to_string()),
            align.to_string(),
        );
        self.item_align = align;
        self
    }

    pub fn add_component(mut self, component: impl ComponentTrait) -> Self {
        self.stuff.alter(ArcOp::Add(ArcComponent::with(component)));
        self
    }

    #[fn_builder]
    pub fn alter_components(&mut self, op: ArcOp) -> &mut Self {
        self.stuff.alter(op);
        self
    }

    // Item GETTERS.

    pub fn item_classes(&self) -> &OptionClasses {
        &self.item_classes
    }

    pub fn inner_classes(&self) -> &OptionClasses {
        &self.inner_classes
    }

    pub fn grow(&self) -> &flex::ItemGrow {
        &self.item_grow
    }

    pub fn shrink(&self) -> &flex::ItemShrink {
        &self.item_shrink
    }

    pub fn size(&self) -> &flex::ItemSize {
        &self.item_size
    }

    pub fn offset(&self) -> &flex::ItemOffset {
        &self.item_offset
    }

    pub fn align(&self) -> &flex::ItemAlign {
        &self.item_align
    }

    pub fn components(&self) -> &ArcComponents {
        &self.stuff
    }
}
