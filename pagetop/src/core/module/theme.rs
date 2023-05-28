use super::ModuleTrait;

use crate::config;
use crate::core::component::{ComponentTrait, RenderContext};
use crate::html::{html, Favicon, Markup};
use crate::response::page::Page;

pub type ThemeStaticRef = &'static dyn ThemeTrait;

/// Los temas deben implementar este "trait".
pub trait ThemeTrait: ModuleTrait + Send + Sync {
    #[allow(unused_variables)]
    fn before_render_page(&self, page: &mut Page) {
        if page.favicon().is_none() {
            page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")));
        }
    }

    fn render_page_head(&self, page: &mut Page) -> Markup {
        let title = page.title();
        let description = page.description();
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        html! {
            head {
                meta charset="utf-8";

                @if !title.is_empty() {
                    title { (config::SETTINGS.app.name) (" | ") (title) }
                } @else {
                    title { (config::SETTINGS.app.name) }
                }

                @if !description.is_empty() {
                    meta name="description" content=(description);
                }

                meta name="viewport" content=(viewport);
                @for (name, content) in page.metadata() {
                    meta name=(name) content=(content) {}
                }

                meta http-equiv="X-UA-Compatible" content="IE=edge";
                @for (property, content) in page.properties() {
                    meta property=(property) content=(content) {}
                }

                @if let Some(f) = page.favicon() {
                    (f.render())
                }

                (page.context().render())
            }
        }
    }

    fn render_page_body(&self, page: &mut Page) -> Markup {
        html! {
            body class=[page.body_classes().get()] {
                @match page.template() {
                    "admin" => {
                        @for region in &["top-menu", "side-menu", "region-content"] {
                            @if let Some(content) = page.render_region(region) {
                                #(region) { (content) }
                            }
                        }
                    },
                    _ => {
                        @for region in &["region-content"] {
                            @if let Some(content) = page.render_region(region) {
                                #(region) { (content) }
                            }
                        }
                    }
                }
            }
        }
    }

    #[rustfmt::skip]
    #[allow(unused_variables)]
    fn before_render_component(
        &self,
        component: &mut dyn ComponentTrait,
        rcx: &mut RenderContext,
    ) {
        /*
            Cómo usarlo:

            match component.handle() {
                BLOCK_COMPONENT => {
                    let block = component_mut::<Block>(component);
                    block.alter_title("New title");
                },
                _ => {},
            }
        */
    }

    #[rustfmt::skip]
    #[allow(unused_variables)]
    fn render_component(
        &self,
        component: &dyn ComponentTrait,
        rcx: &mut RenderContext,
    ) -> Option<Markup> {
        None
        /*
            Cómo usarlo:

            match component.handle() {
                BLOCK_COMPONENT => {
                    let block = component_ref::<Block>(component);
                    match block.template() {
                        "default" => Some(block_default(block)),
                        _ => None,
                    }
                },
                _ => None,
            }
        */
    }
}