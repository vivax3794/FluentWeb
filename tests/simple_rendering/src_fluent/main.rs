use fluent_web_client::{component, render_component};

component!(App);

fn main() {
    render_component::<App>("mount");
}
