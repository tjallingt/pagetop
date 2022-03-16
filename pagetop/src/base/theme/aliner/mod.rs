use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/aliner.rs"));

pub struct AlinerTheme;

impl ThemeTrait for AlinerTheme {
    fn name(&self) -> &'static str {
        "aliner"
    }

    fn fullname(&self) -> String {
        "Aliner".to_owned()
    }

    fn configure_theme(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.service(actix_web_static_files::ResourceFiles::new(
            "/aliner",
            assets()
        ));
    }

    fn before_render_page(&self, page: &mut Page) {
        page.assets()
            .add_stylesheet(
                StyleSheet::source(
                    "/aliner/css/styles.css"
                )
                .with_weight(-99)
            );
    }
}
