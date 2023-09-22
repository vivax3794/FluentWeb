use fluent_web_client::render_component;

mod App;

fn main() {
    render_component!(App, "mount");
}
