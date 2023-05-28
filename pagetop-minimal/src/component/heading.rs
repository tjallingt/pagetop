use pagetop::prelude::*;

define_handle!(COMPONENT_HEADING);

#[derive(Default)]
pub enum HeadingType {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Default)]
pub enum HeadingDisplay {
    #[default]
    Normal,
    XxLarge,
    Large,
    Medium,
    Small,
    XxSmall,
    Subtitle,
}

type HeadingText = OneComponent<L10n>;

#[rustfmt::skip]
#[derive(Default)]
pub struct Heading {
    weight      : isize,
    renderable  : Renderable,
    id          : IdentifierValue,
    classes     : Classes,
    heading_type: HeadingType,
    text        : HeadingText,
    display     : HeadingDisplay,
    template    : String,
}

impl ComponentTrait for Heading {
    fn new() -> Self {
        Heading::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_HEADING
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        let id = self.id().get();
        let classes = self.classes().get();
        html! { @match &self.heading_type() {
            HeadingType::H1 => h1 id=[id] class=[classes] { (self.text().render(rcx)) },
            HeadingType::H2 => h2 id=[id] class=[classes] { (self.text().render(rcx)) },
            HeadingType::H3 => h3 id=[id] class=[classes] { (self.text().render(rcx)) },
            HeadingType::H4 => h4 id=[id] class=[classes] { (self.text().render(rcx)) },
            HeadingType::H5 => h5 id=[id] class=[classes] { (self.text().render(rcx)) },
            HeadingType::H6 => h6 id=[id] class=[classes] { (self.text().render(rcx)) },
        }}
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Heading {
    pub fn h1(text: L10n) -> Self {
        Heading::new()
            .with_heading_type(HeadingType::H1)
            .with_text(text)
    }

    pub fn h2(text: L10n) -> Self {
        Heading::new()
            .with_heading_type(HeadingType::H2)
            .with_text(text)
    }

    pub fn h3(text: L10n) -> Self {
        Heading::new()
            .with_heading_type(HeadingType::H3)
            .with_text(text)
    }

    pub fn h4(text: L10n) -> Self {
        Heading::new()
            .with_heading_type(HeadingType::H4)
            .with_text(text)
    }

    pub fn h5(text: L10n) -> Self {
        Heading::new()
            .with_heading_type(HeadingType::H5)
            .with_text(text)
    }

    pub fn h6(text: L10n) -> Self {
        Heading::new()
            .with_heading_type(HeadingType::H6)
            .with_text(text)
    }

    // Heading BUILDER.

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
    pub fn alter_heading_type(&mut self, heading_type: HeadingType) -> &mut Self {
        self.heading_type = heading_type;
        self
    }

    #[fn_builder]
    pub fn alter_text(&mut self, text: L10n) -> &mut Self {
        self.text.set(text);
        self
    }

    #[rustfmt::skip]
    #[fn_builder]
    pub fn alter_display(&mut self, display: HeadingDisplay) -> &mut Self {
        self.display = display;
        self.classes.alter_value(
            ClassesOp::SetDefault,
            match &self.display() {
                HeadingDisplay::XxLarge  => "display-2",
                HeadingDisplay::Large    => "display-3",
                HeadingDisplay::Medium   => "display-4",
                HeadingDisplay::Small    => "display-5",
                HeadingDisplay::XxSmall  => "display-6",
                HeadingDisplay::Normal   => "",
                HeadingDisplay::Subtitle => "",
            },
        );
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Paragraph GETTERS.

    pub fn id(&self) -> &IdentifierValue {
        &self.id
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn heading_type(&self) -> &HeadingType {
        &self.heading_type
    }

    pub fn text(&self) -> &HeadingText {
        &self.text
    }

    pub fn display(&self) -> &HeadingDisplay {
        &self.display
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}