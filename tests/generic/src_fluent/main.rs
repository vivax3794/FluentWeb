use fluent_web_client::render_component;

mod App;
mod Sub1;
mod Sub2;
mod Sub3;

fn main() {
    render_component!(App, "mount");
}
