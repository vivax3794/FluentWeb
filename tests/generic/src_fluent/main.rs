use fluent_web_runtime::{forget, render_component};

mod App;
mod Sub1;
mod Sub2;
mod Sub3;

fn main() {
    forget(render_component!(App, "mount"));
}
