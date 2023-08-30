#![allow(warnings)]
use ::fluent_web_client::internal::web_sys::*;
use super::*;
struct Thing;
#[derive(Clone)]
struct __Fluid_Data {}
#[derive(Clone)]
pub struct Component {
    root_name: ::std::string::String,
    data: __Fluid_Data,
}
impl Component {
    fn spawn_component___Fluent_UUID_bb57ba68_90fe_4f90_b179_a9fe71f02574(&self) {
        ::fluent_web_client::render_component::<
            Wrapper,
        >("__Fluent_UUID_bb57ba68_90fe_4f90_b179_a9fe71f02574");
    }
    fn spawn_component___Fluent_UUID_2d809d4b_4d72_4b66_a836_02f7356cc007(&self) {
        ::fluent_web_client::render_component::<
            Wrapper,
        >("__Fluent_UUID_2d809d4b_4d72_4b66_a836_02f7356cc007");
    }
    fn spawn_component___Fluent_UUID_a6b515a2_94f9_4c5e_95af_6fae0a669b07(&self) {
        ::fluent_web_client::render_component::<
            Wrapper,
        >("__Fluent_UUID_a6b515a2_94f9_4c5e_95af_6fae0a669b07");
    }
}
impl ::fluent_web_client::internal::Component for Component {
    fn render_init(&self) -> ::std::string::String {
        "<div id=\"__Fluent_UUID_bb57ba68_90fe_4f90_b179_a9fe71f02574\"></div>\n    <div id=\"__Fluent_UUID_2d809d4b_4d72_4b66_a836_02f7356cc007\"></div>\n    <div id=\"__Fluent_UUID_a6b515a2_94f9_4c5e_95af_6fae0a669b07\"></div>\n\n"
            .into()
    }
    fn create(root_id: String) -> Self {
        Self {
            root_name: root_id,
            data: __Fluid_Data {},
        }
    }
    fn setup_events(&self) {}
    fn update_all(&self) {}
    fn spawn_sub(&self) {
        self.spawn_component___Fluent_UUID_bb57ba68_90fe_4f90_b179_a9fe71f02574();
        self.spawn_component___Fluent_UUID_2d809d4b_4d72_4b66_a836_02f7356cc007();
        self.spawn_component___Fluent_UUID_a6b515a2_94f9_4c5e_95af_6fae0a669b07();
    }
}
