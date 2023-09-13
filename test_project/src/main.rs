use fluent_web_client::component;

component!(App);

fn main() {
    console_error_panic_hook::set_once();
    fluent_web_client::render_component::<App>("mount");
}
