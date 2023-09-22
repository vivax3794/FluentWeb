use fluent_web_client::render_component;

mod App;
mod Sub;

fn main() {
    render_component!(App, "mount");
}
