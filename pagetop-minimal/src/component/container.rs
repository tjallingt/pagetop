use pagetop::prelude::*;

define_handle!(COMPONENT_CONTAINER);

action_before_render_component!(ACTION_BEFORE_RENDER_CONTAINER for Container);

#[derive(Default)]
pub enum ContainerType {
    #[default]
    Wrapper,
    Header,
    Footer,
    Main,
    Section,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct Container {
    weight        : isize,
    renderable    : Renderable,
    id            : IdentifierValue,
    classes       : Classes,
    inner_classes : Classes,
    container_type: ContainerType,
    components    : ComponentsBundle,
    template      : String,
}

impl ComponentTrait for Container {
    fn new() -> Self {
        Container::default()
            .with_classes(ClassesOp::SetDefault, "container")
            .with_inner_classes(ClassesOp::SetDefault, "container")
    }

    fn handle(&self) -> Handle {
        COMPONENT_CONTAINER
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn before_render(&mut self, rcx: &mut RenderContext) {
        run_actions_before_render_component(self, rcx);
    }

    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        match self.container_type() {
            ContainerType::Header => html! {
                header id=[self.id()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().render(rcx))
                    }
                }
            },
            ContainerType::Footer => html! {
                footer id=[self.id()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().render(rcx))
                    }
                }
            },
            ContainerType::Main => html! {
                main id=[self.id()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().render(rcx))
                    }
                }
            },
            ContainerType::Section => html! {
                section id=[self.id()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().render(rcx))
                    }
                }
            },
            _ => html! {
                div id=[self.id()] class=[self.classes().get()] {
                    (self.components().render(rcx))
                }
            },
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
        let mut c = Container::new().with_classes(ClassesOp::SetDefault, "header");
        c.container_type = ContainerType::Header;
        c
    }

    pub fn footer() -> Self {
        let mut c = Container::new().with_classes(ClassesOp::SetDefault, "footer");
        c.container_type = ContainerType::Footer;
        c
    }

    pub fn main() -> Self {
        let mut c = Container::new().with_classes(ClassesOp::SetDefault, "main");
        c.container_type = ContainerType::Main;
        c
    }

    pub fn section() -> Self {
        let mut c = Container::new().with_classes(ClassesOp::SetDefault, "section");
        c.container_type = ContainerType::Section;
        c
    }

    // Container BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_inner_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.inner_classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_component(&mut self, component: impl ComponentTrait) -> &mut Self {
        self.components.add(component);
        self
    }

    #[fn_builder]
    pub fn alter_bundle(&mut self, op: BundleOp, component: impl ComponentTrait) -> &mut Self {
        self.components.alter_bundle(op, component);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Container GETTERS.

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
