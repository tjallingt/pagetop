use crate::core::component::{AnyComponent, ComponentTrait, RenderContext};
use crate::html::{html, Markup};
use crate::{use_handle, Handle};

use_handle!(ERROR_404);

pub struct Error404;

impl ComponentTrait for Error404 {
    fn new() -> Self {
        Self
    }

    fn handle(&self) -> Handle {
        ERROR_404
    }

    fn default_render(&self, _rcx: &mut RenderContext) -> Markup {
        html! {
            div {
                h1 { ("RESOURCE NOT FOUND") }
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
