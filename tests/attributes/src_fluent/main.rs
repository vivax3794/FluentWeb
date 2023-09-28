use fluent_web_runtime::{forget, render_component};

mod App;
mod Sub;

fn main() {
    forget(render_component!(App, "mount"));
}
