#![feature(specialization)]
#![allow(incomplete_features)]

pub mod internal;

pub fn render_component<C: internal::Component>(mount_point: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.get_element_by_id(mount_point).unwrap();

    body.set_class_name("__Fluent_Component");

    let component = C::create(mount_point.to_owned());
    body.set_inner_html(&component.render_init());
    component.setup_events();
    component.spawn_sub();
    component.update_all();
}

pub use paste::paste;

#[macro_export]
macro_rules! component {
    ($vis:vis $name:ident) => {
        ::fluent_web_client::paste! {
            mod [<$name Module>];
            $vis use [<$name Module>]::Component as $name;
        }
    };
}
