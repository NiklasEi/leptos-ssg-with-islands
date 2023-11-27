use leptos_actix::generate_route_list_with_ssg;
use leptos_router::build_static_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use leptos::*;
    use test_leptos::app::*;

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let (routes, static_data_map) = generate_route_list_with_ssg(App);

    build_static_routes(&leptos_options, App, &routes, &static_data_map)
        .await
        .unwrap();

    Ok(())
}

#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}
