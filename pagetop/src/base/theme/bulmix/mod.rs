use crate::prelude::*;

pub const BULMIX_THEME: &str = "pagetop::theme::bulmix";

include!(concat!(env!("OUT_DIR"), "/bulmix.rs"));

pub struct Bulmix;

impl ThemeTrait for Bulmix {
    fn handler(&self) -> &'static str {
        BULMIX_THEME
    }

    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
        theme_static_files!(cfg, "/bulmix");
    }

    fn before_render_page(&self, page: &mut Page) {
        page.assets()
            .with_favicon(
                Favicon::new()
                    .with_icon("/theme/favicon.png")
            )
            .add_stylesheet(
                StyleSheet::source(
                    "/bulmix/css/bulma.min.css?ver=0.9.3"
                )
                .with_weight(-99)
            )
            .add_jquery();
    }

    fn before_render_component(
        &self,
        component: &mut dyn ComponentTrait,
        _assets: &mut Assets
    ) {
        match component.handler() {
            HEADING_COMPONENT => {
                let h = component_mut::<Heading>(component);
                h.alter_classes(
                    concat_string!("title ", match h.display() {
                        HeadingDisplay::XxLarge    => "is-1",
                        HeadingDisplay::Large      => "is-2",
                        HeadingDisplay::Normal     => "",
                        HeadingDisplay::Medium     => "is-3",
                        HeadingDisplay::Small      => "is-4",
                        HeadingDisplay::XxSmall    => "is-5",
                    }).as_str(),
                    ClassesOp::SetDefault
                );
            },
            PARAGRAPH_COMPONENT => {
                let p = component_mut::<Paragraph>(component);
                p.alter_classes(
                    match p.display() {
                        ParagraphDisplay::XxLarge    => "is-size-2",
                        ParagraphDisplay::Large      => "is-size-3",
                        ParagraphDisplay::MediumPlus => "is-size-4",
                        ParagraphDisplay::Normal     => "",
                        ParagraphDisplay::Medium     => "is-size-5",
                        ParagraphDisplay::Small      => "is-size-6",
                        ParagraphDisplay::XxSmall    => "is-size-7",
                    },
                    ClassesOp::SetDefault
                );
            },
            grid::ROW_COMPONENT => {
                let row = component_mut::<grid::Row>(component);
                row.alter_classes("columns", ClassesOp::SetDefault);
            },
            grid::COLUMN_COMPONENT => {
                let col = component_mut::<grid::Column>(component);
                col.alter_classes("column", ClassesOp::SetDefault);
            },
            _ => {},
        }
    }
}
