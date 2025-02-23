use pagetop::prelude::*;

struct HelloName;

impl_handle!(APP_HELLO_NAME for HelloName);

impl ModuleTrait for HelloName {
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.service(hello_name);
    }
}

#[service::get("/hello/{name}")]
async fn hello_name(
    request: service::HttpRequest,
    path: service::web::Path<String>,
) -> ResultPage<Markup, FatalError> {
    let name = path.into_inner();
    Page::new(request)
        .with_in("content", Html::with(html! { h1 { "Hello " (name) "!" } }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloName).unwrap().run()?.await
}
