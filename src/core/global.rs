use crate::{Lazy, db};
use crate::core::theme::Theme;
use crate::core::module::Module;
use crate::core::response::page::PageContainer;
use crate::core::server;

use std::sync::RwLock;
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

// -----------------------------------------------------------------------------
// Temas registrados y tema por defecto.
// -----------------------------------------------------------------------------

pub static THEMES: Lazy<RwLock<Vec<&dyn Theme>>> = Lazy::new(|| {
    RwLock::new(Vec::new())
});

pub fn themes(cfg: &mut server::web::ServiceConfig) {
    cfg.service(actix_web_static_files::ResourceFiles::new(
        "/theme",
        assets()
    ));

    for t in THEMES.read().unwrap().iter() {
        t.configure_theme(cfg);
    }
}

// -----------------------------------------------------------------------------
// Módulos registrados.
// -----------------------------------------------------------------------------

pub static MODULES: Lazy<RwLock<Vec<&dyn Module>>> = Lazy::new(|| {
    RwLock::new(Vec::new())
});

pub fn modules(cfg: &mut server::web::ServiceConfig) {
    for m in MODULES.read().unwrap().iter() {
        m.configure_module(cfg);
    }
}

pub fn migrations(db_uri: db::Uri) {
    let mut conn = refinery::config::Config::try_from(db_uri).unwrap();
    for m in MODULES.read().unwrap().iter() {
        match m.configure_migrations() {
            Some(migrations) => {
                migrations.run(&mut conn).expect("Failed to run migrations");
            },
            _ => {}
        };
    }
}

// -----------------------------------------------------------------------------
// Componentes globales.
// -----------------------------------------------------------------------------

pub static COMPONENTS: Lazy<RwLock<HashMap<&str, PageContainer>>> = Lazy::new(
    || { RwLock::new(HashMap::new()) }
);
