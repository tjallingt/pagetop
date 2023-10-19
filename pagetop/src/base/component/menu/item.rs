use crate::prelude::*;

use super::{Megamenu, Submenu};

new_handle!(COMPONENT_MENU_ITEM);

type Label = TypedComponent<L10n>;
type Content = TypedComponent<Html>;
type SubmenuItems = TypedComponent<Submenu>;
type MegamenuGroups = TypedComponent<Megamenu>;
type Description = TypedComponent<L10n>;

#[derive(Default)]
pub enum ItemType {
    #[default]
    Void,
    Label(Label),
    Link(Label, FnContextualPath),
    LinkBlank(Label, FnContextualPath),
    Html(Content),
    Submenu(Label, SubmenuItems),
    Megamenu(Label, MegamenuGroups),
}

// Item.

#[rustfmt::skip]
#[derive(Default)]
pub struct Item {
    weight     : Weight,
    renderable : Renderable,
    item_type  : ItemType,
    description: Description,
}

impl ComponentTrait for Item {
    fn new() -> Self {
        Item::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_MENU_ITEM
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let description = self.description.get().into_string(cx);
        match self.item_type() {
            ItemType::Void => PrepareMarkup::None,
            ItemType::Label(label) => PrepareMarkup::With(html! {
                li class="pt-menu__label" {
                    span title=[description] {
                        (label.prepare(cx))
                    }
                }
            }),
            ItemType::Link(label, path) => PrepareMarkup::With(html! {
                li class="pt-menu__link" {
                    a href=(path(cx)) title=[description] {
                        (label.prepare(cx))
                    }
                }
            }),
            ItemType::LinkBlank(label, path) => PrepareMarkup::With(html! {
                li class="pt-menu__link" {
                    a href=(path(cx)) title=[description] target="_blank" {
                        (label.prepare(cx))
                    }
                }
            }),
            ItemType::Html(content) => PrepareMarkup::With(html! {
                li class="pt-menu__html" {
                    (content.prepare(cx))
                }
            }),
            ItemType::Submenu(label, submenu) => PrepareMarkup::With(html! {
                li class="pt-menu__children" {
                    a href="#" title=[description] {
                        (label.prepare(cx)) i class="pt-menu__icon bi-chevron-down" {}
                    }
                    div class="pt-menu__subs" {
                        (submenu.prepare(cx))
                    }
                }
            }),
            ItemType::Megamenu(label, megamenu) => PrepareMarkup::With(html! {
                li class="pt-menu__children" {
                    a href="#" title=[description] {
                        (label.prepare(cx)) i class="pt-menu__icon bi-chevron-down" {}
                    }
                    div class="pt-menu__subs pt-menu__mega" {
                        (megamenu.prepare(cx))
                    }
                }
            }),
        }
    }
}

impl Item {
    pub fn label(label: L10n) -> Self {
        Item {
            item_type: ItemType::Label(Label::with(label)),
            ..Default::default()
        }
    }

    pub fn link(label: L10n, path: FnContextualPath) -> Self {
        Item {
            item_type: ItemType::Link(Label::with(label), path),
            ..Default::default()
        }
    }

    pub fn link_blank(label: L10n, path: FnContextualPath) -> Self {
        Item {
            item_type: ItemType::LinkBlank(Label::with(label), path),
            ..Default::default()
        }
    }

    pub fn html(content: Html) -> Self {
        Item {
            item_type: ItemType::Html(Content::with(content)),
            ..Default::default()
        }
    }

    pub fn submenu(label: L10n, submenu: Submenu) -> Self {
        Item {
            item_type: ItemType::Submenu(Label::with(label), SubmenuItems::with(submenu)),
            ..Default::default()
        }
    }

    pub fn megamenu(label: L10n, megamenu: Megamenu) -> Self {
        Item {
            item_type: ItemType::Megamenu(Label::with(label), MegamenuGroups::with(megamenu)),
            ..Default::default()
        }
    }

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
    pub fn alter_description(&mut self, text: L10n) -> &mut Self {
        self.description.set(text);
        self
    }

    // Item GETTERS.

    pub fn item_type(&self) -> &ItemType {
        &self.item_type
    }

    pub fn description(&self) -> &Description {
        &self.description
    }
}