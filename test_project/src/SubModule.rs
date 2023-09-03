#![allow(warnings)]
use ::fluent_web_client::internal::web_sys::*;
#[derive(Clone)]
struct __Fluid_Data {}
#[derive(Clone)]
pub struct Component {
    root_name: ::std::string::String,
    data: __Fluid_Data,
}
impl Component {}
impl ::fluent_web_client::internal::Component for Component {
    fn render_init(&self) -> ::std::string::String {
        "<p>Sub</p>\n<style>p{background-color:red}</style>".into()
    }
    fn create(root_id: String) -> Self {
        Self {
            root_name: root_id,
            data: __Fluid_Data {},
        }
    }
    fn setup_events(&self) {}
    fn update_all(&self) {}
    fn spawn_sub(&self) {}
}
