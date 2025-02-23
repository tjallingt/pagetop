use crate::prelude::*;

#[derive(Default)]
pub enum InputType {
    #[default]
    Textfield,
    Password,
    Search,
    Email,
    Telephone,
    Url,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct Input {
    weight      : Weight,
    renderable  : Renderable,
    classes     : OptionClasses,
    input_type  : InputType,
    name        : OptionName,
    value       : OptionString,
    label       : OptionTranslated,
    size        : Option<u16>,
    minlength   : Option<u16>,
    maxlength   : Option<u16>,
    placeholder : OptionString,
    autofocus   : OptionString,
    autocomplete: OptionString,
    disabled    : OptionString,
    readonly    : OptionString,
    required    : OptionString,
    help_text   : OptionTranslated,
    template    : String,
}

impl_handle!(COMPONENT_BASE_INPUT for Input);

impl ComponentTrait for Input {
    fn new() -> Self {
        Input::default()
            .with_classes(ClassesOp::Add, "form-item form-type-textfield")
            .with_size(Some(60))
            .with_maxlength(Some(128))
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    #[rustfmt::skip]
    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let type_input = match self.input_type() {
            InputType::Textfield => "text",
            InputType::Password  => "password",
            InputType::Search    => "search",
            InputType::Email     => "email",
            InputType::Telephone => "tel",
            InputType::Url       => "url",
        };
        let id = self.name().get().map(|name| concat_string!("edit-", name));
        PrepareMarkup::With(html! {
            div class=[self.classes().get()] {
                @if let Some(label) = self.label().using(cx.langid()) {
                    label class="form-label" for=[&id] {
                        (label) " "
                        @if self.required().get().is_some() {
                            span
                                class="form-required"
                                title="Este campo es obligatorio." { "*" } " "
                        }
                    }
                }
                input
                    type=(type_input)
                    id=[id]
                    class="form-control"
                    name=[self.name().get()]
                    value=[self.value().get()]
                    size=[self.size()]
                    minlength=[self.minlength()]
                    maxlength=[self.maxlength()]
                    placeholder=[self.placeholder().get()]
                    autofocus=[self.autofocus().get()]
                    autocomplete=[self.autocomplete().get()]
                    readonly=[self.readonly().get()]
                    required=[self.required().get()]
                    disabled=[self.disabled().get()] {}
                @if let Some(description) = self.help_text().using(cx.langid()) {
                    div class="form-text" { (description) }
                }
            }
        })
    }
}

impl Input {
    pub fn textfield() -> Self {
        Input::default()
    }

    pub fn password() -> Self {
        let mut input = Input::default().with_classes(
            ClassesOp::Replace("form-type-textfield".to_owned()),
            "form-type-password",
        );
        input.input_type = InputType::Password;
        input
    }

    pub fn search() -> Self {
        let mut input = Input::default().with_classes(
            ClassesOp::Replace("form-type-textfield".to_owned()),
            "form-type-search",
        );
        input.input_type = InputType::Search;
        input
    }

    pub fn email() -> Self {
        let mut input = Input::default().with_classes(
            ClassesOp::Replace("form-type-textfield".to_owned()),
            "form-type-email",
        );
        input.input_type = InputType::Email;
        input
    }

    pub fn telephone() -> Self {
        let mut input = Input::default().with_classes(
            ClassesOp::Replace("form-type-textfield".to_owned()),
            "form-type-telephone",
        );
        input.input_type = InputType::Telephone;
        input
    }

    pub fn url() -> Self {
        let mut input = Input::default().with_classes(
            ClassesOp::Replace("form-type-textfield".to_owned()),
            "form-type-url",
        );
        input.input_type = InputType::Url;
        input
    }

    // Input BUILDER.

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
        if let Some(previous) = self.name.get() {
            self.alter_classes(ClassesOp::Remove, concat_string!("form-item-", previous));
        }
        self.alter_classes(ClassesOp::Add, concat_string!("form-item-", name));
        self.name.alter_value(name);
        self
    }

    #[fn_builder]
    pub fn alter_value(&mut self, value: &str) -> &mut Self {
        self.value.alter_value(value);
        self
    }

    #[fn_builder]
    pub fn alter_label(&mut self, label: L10n) -> &mut Self {
        self.label.alter_value(label);
        self
    }

    #[fn_builder]
    pub fn alter_size(&mut self, size: Option<u16>) -> &mut Self {
        self.size = size;
        self
    }

    #[fn_builder]
    pub fn alter_minlength(&mut self, minlength: Option<u16>) -> &mut Self {
        self.minlength = minlength;
        self
    }

    #[fn_builder]
    pub fn alter_maxlength(&mut self, maxlength: Option<u16>) -> &mut Self {
        self.maxlength = maxlength;
        self
    }

    #[fn_builder]
    pub fn alter_placeholder(&mut self, placeholder: &str) -> &mut Self {
        self.placeholder.alter_value(placeholder);
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
    pub fn alter_autocomplete(&mut self, toggle: bool) -> &mut Self {
        self.autocomplete.alter_value(match toggle {
            true => "",
            false => "off",
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
    pub fn alter_readonly(&mut self, toggle: bool) -> &mut Self {
        self.readonly.alter_value(match toggle {
            true => "readonly",
            false => "",
        });
        self
    }

    #[fn_builder]
    pub fn alter_required(&mut self, toggle: bool) -> &mut Self {
        self.required.alter_value(match toggle {
            true => "required",
            false => "",
        });
        self
    }

    #[fn_builder]
    pub fn alter_help_text(&mut self, help_text: L10n) -> &mut Self {
        self.help_text.alter_value(help_text);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Input GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn input_type(&self) -> &InputType {
        &self.input_type
    }

    pub fn name(&self) -> &OptionName {
        &self.name
    }

    pub fn value(&self) -> &OptionString {
        &self.value
    }

    pub fn label(&self) -> &OptionTranslated {
        &self.label
    }

    pub fn size(&self) -> Option<u16> {
        self.size
    }

    pub fn minlength(&self) -> Option<u16> {
        self.minlength
    }

    pub fn maxlength(&self) -> Option<u16> {
        self.maxlength
    }

    pub fn placeholder(&self) -> &OptionString {
        &self.placeholder
    }

    pub fn autofocus(&self) -> &OptionString {
        &self.autofocus
    }

    pub fn autocomplete(&self) -> &OptionString {
        &self.autocomplete
    }

    pub fn disabled(&self) -> &OptionString {
        &self.disabled
    }

    pub fn readonly(&self) -> &OptionString {
        &self.readonly
    }

    pub fn required(&self) -> &OptionString {
        &self.required
    }

    pub fn help_text(&self) -> &OptionTranslated {
        &self.help_text
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
