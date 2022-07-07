use crate::prelude::*;

pub const CONTAINER_COMPONENT: &str = "pagetop::component::container";

pub enum ContainerType { Header, Footer, Main, Section, Wrapper }

pub struct Container {
    renderable    : fn() -> bool,
    weight        : isize,
    id            : IdentifierValue,
    classes       : Classes,
    inner_classes : Classes,
    container_type: ContainerType,
    components    : ComponentsBundle,
    template      : String,
}

impl ComponentTrait for Container {
    fn new() -> Self {
        Container {
            renderable    : render_always,
            weight        : 0,
            id            : IdentifierValue::new(),
            classes       : Classes::new_with_default("container"),
            inner_classes : Classes::new_with_default("container"),
            container_type: ContainerType::Wrapper,
            components    : ComponentsBundle::new(),
            template      : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        CONTAINER_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, context: &mut InContext) -> Markup {
        match self.container_type() {
            ContainerType::Header => html! {
                header id=[self.id().get()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().render(context))
                    }
                }
            },
            ContainerType::Footer => html! {
                footer id=[self.id().get()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().render(context))
                    }
                }
            },
            ContainerType::Main => html! {
                main id=[self.id().get()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().render(context))
                    }
                }
            },
            ContainerType::Section => html! {
                section id=[self.id().get()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().render(context))
                    }
                }
            },
            _ => html! {
                div id=[self.id().get()] class=[self.classes().get()] {
                    (self.components().render(context))
                }
            }
        }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Container {
    pub fn header() -> Self {
        let mut c = Container::new().with_classes("header", ClassesOp::SetDefault);
        c.container_type = ContainerType::Header;
        c
    }

    pub fn footer() -> Self {
        let mut c = Container::new().with_classes("footer", ClassesOp::SetDefault);
        c.container_type = ContainerType::Footer;
        c
    }

    pub fn main() -> Self {
        let mut c = Container::new().with_classes("main", ClassesOp::SetDefault);
        c.container_type = ContainerType::Main;
        c
    }

    pub fn section() -> Self {
        let mut c = Container::new().with_classes("section", ClassesOp::SetDefault);
        c.container_type = ContainerType::Section;
        c
    }

    // Container BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.alter_id(id);
        self
    }

    pub fn with_classes(mut self, classes: &str, op: ClassesOp) -> Self {
        self.alter_classes(classes, op);
        self
    }

    pub fn with_inner_classes(mut self, classes: &str, op: ClassesOp) -> Self {
        self.alter_inner_classes(classes, op);
        self
    }

    pub fn with_component(mut self, component: impl ComponentTrait) -> Self {
        self.alter_component(component);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Container ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.with_value(id);
        self
    }

    pub fn alter_classes(&mut self, classes: &str, op: ClassesOp) -> &mut Self {
        self.classes.alter(classes, op);
        self
    }

    pub fn alter_inner_classes(&mut self, classes: &str, op: ClassesOp) -> &mut Self {
        self.inner_classes.alter(classes, op);
        self
    }

    pub fn alter_component(&mut self, component: impl ComponentTrait) -> &mut Self {
        self.components.add(component);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Container GETTERS.

    pub fn id(&self) -> &IdentifierValue {
        &self.id
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn inner_classes(&self) -> &Classes {
        &self.inner_classes
    }

    pub fn container_type(&self) -> &ContainerType {
        &self.container_type
    }

    pub fn components(&self) -> &ComponentsBundle {
        &self.components
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
