use crate::prelude::*;

#[derive(Default)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct Button {
    weight     : Weight,
    renderable : Renderable,
    classes    : OptionClasses,
    button_type: ButtonType,
    name       : OptionString,
    value      : OptionTranslated,
    autofocus  : OptionString,
    disabled   : OptionString,
    template   : String,
}

impl_handle!(COMPONENT_BASE_BUTTON for Button);

impl ComponentTrait for Button {
    fn new() -> Self {
        Button::default().with_classes(ClassesOp::Add, "btn btn-primary form-button")
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let button_type = match self.button_type() {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
        };
        let id = self.name().get().map(|name| concat_string!("edit-", name));
        PrepareMarkup::With(html! {
            button
                type=(button_type)
                id=[id]
                class=[self.classes().get()]
                name=[self.name().get()]
                value=[self.value().using(cx.langid())]
                autofocus=[self.autofocus().get()]
                disabled=[self.disabled().get()]
            {
                (self.value().escaped(cx.langid()))
            }
        })
    }
}

impl Button {
    pub fn with(value: L10n) -> Self {
        Button::default().with_value(value)
    }

    pub fn submit(value: L10n) -> Self {
        let mut button = Button::default()
            .with_classes(ClassesOp::Replace("form-button".to_owned()), "form-submit")
            .with_value(value);
        button.button_type = ButtonType::Submit;
        button
    }

    pub fn reset(value: L10n) -> Self {
        let mut button = Button::default()
            .with_classes(ClassesOp::Replace("form-button".to_owned()), "form-reset")
            .with_value(value);
        button.button_type = ButtonType::Reset;
        button
    }

    // Button BUILDER.

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
    pub fn alter_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_name(&mut self, name: &str) -> &mut Self {
        self.name.alter_value(name);
        self
    }

    #[fn_builder]
    pub fn alter_value(&mut self, value: L10n) -> &mut Self {
        self.value.alter_value(value);
        self
    }

    #[fn_builder]
    pub fn alter_autofocus(&mut self, toggle: bool) -> &mut Self {
        self.autofocus.alter_value(match toggle {
            true => "autofocus",
            false => "",
        });
        self
    }

    #[fn_builder]
    pub fn alter_disabled(&mut self, toggle: bool) -> &mut Self {
        self.disabled.alter_value(match toggle {
            true => "disabled",
            false => "",
        });
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Button GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn button_type(&self) -> &ButtonType {
        &self.button_type
    }

    pub fn name(&self) -> &OptionString {
        &self.name
    }

    pub fn value(&self) -> &OptionTranslated {
        &self.value
    }

    pub fn autofocus(&self) -> &OptionString {
        &self.autofocus
    }

    pub fn disabled(&self) -> &OptionString {
        &self.disabled
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
