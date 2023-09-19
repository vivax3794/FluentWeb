use fluent_web_client::{component, render_component};

component!(App);
component!(Sub);

fn main() {
    render_component::<App>("mount");
}
