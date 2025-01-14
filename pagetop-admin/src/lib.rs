use pagetop::prelude::*;

static_locales!(LOCALES_ADMIN);

mod summary;

pub struct Admin;

impl_handle!(MODULE_ADMIN for Admin);

impl ModuleTrait for Admin {
    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALES_ADMIN)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALES_ADMIN)
    }

    fn actions(&self) -> Vec<Action> {
        actions![
            action::page::BeforePrepareBody::with(before_prepare_body),
            action::component::BeforePrepareComponent::<Menu>::with(before_prepare_menu)
                .filter_by_referer_id("admin-menu-test"),
        ]
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.service(
            service::web::scope("/admin").route("", service::web::get().to(summary::summary)),
        );
    }
}

fn before_prepare_body(page: &mut Page) {
    page.alter_body_classes(ClassesOp::Add, "test-admin");
}

fn before_prepare_menu(component: &mut Menu, _cx: &mut Context) {
    component.alter_id("admin-menu-test-altered");
}
