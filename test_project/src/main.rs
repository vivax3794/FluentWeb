use fluent_web_client::component;

component!(App);
component!(Wrapper);
component!(Sub);

fn main() {
    console_error_panic_hook::set_once();
    fluent_web_client::render_component::<App>("mount");
}
