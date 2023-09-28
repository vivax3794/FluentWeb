use fluent_web_runtime::{forget, render_component};

mod App;

fn main() {
    forget(render_component!(App, "mount"));
}
