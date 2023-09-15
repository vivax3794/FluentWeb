#![feature(inherent_associated_types)]
#[allow(incomplete_features)]
use fluent_web_client::component;

component!(App);
component!(Input);

fn main() {
    console_error_panic_hook::set_once();
    fluent_web_client::render_component::<App>("mount");
}
