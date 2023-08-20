//! Tipos y funciones para operar con el servidor web ([actix-web](https://docs.rs/actix-web)).

pub use actix_session::Session;
pub use actix_web::{
    cookie, get, http, rt, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};

pub use actix_web_files::Files as ActixFiles;
pub use actix_web_static_files::ResourceFiles;

#[macro_export]
macro_rules! static_files {
    ( $bundle:ident ) => {
        $crate::paste! {
            mod [<static_files_ $bundle>] {
                include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
            }
        }
    };
    ( $bundle:ident => $STATIC:ident ) => {
        $crate::paste! {
            mod [<static_files_ $bundle>] {
                include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
            }
            static $STATIC: LazyStatic<HashMapResources> = LazyStatic::new([
                <static_files_ $bundle>]::$bundle
            );
        }
    };
}

#[macro_export]
macro_rules! static_files_service {
    ( $scfg:ident, $path:expr, $bundle:ident ) => {{
        $crate::paste! {
            let static_files = &$crate::config::SETTINGS.dev.static_files;
            if static_files.is_empty() {
                $scfg.service($crate::service::ResourceFiles::new(
                    $path,
                    [<static_files_ $bundle>]::$bundle(),
                ));
            } else {
                $scfg.service(
                    $crate::service::ActixFiles::new(
                        $path,
                        $crate::concat_string!(static_files, $path),
                    )
                    .show_files_listing(),
                );
            }
        }
    }};
}
