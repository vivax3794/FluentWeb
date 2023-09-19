#![feature(inherent_associated_types)]

use fluent_web_client::{component, render_component};

component!(App);
component!(Sub1);
component!(Sub2);
component!(Sub3);

fn main() {
    render_component::<App>("mount");
}
