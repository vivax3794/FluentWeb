#![allow(warnings)]
use ::fluent_web_client::internal::web_sys::*;
use super::Sub;
#[derive(Clone)]
struct __Fluid_Data {}
#[derive(Clone)]
pub struct Component {
    root_name: ::std::string::String,
    data: __Fluid_Data,
}
impl Component {
    fn spawn_component___Fluent_UUID_02f9dc98_715e_44a7_b1cd_69467b01a5f5(&self) {
        ::fluent_web_client::render_component::<
            Sub,
        >("__Fluent_UUID_02f9dc98_715e_44a7_b1cd_69467b01a5f5");
    }
}
impl ::fluent_web_client::internal::Component for Component {
    fn render_init(&self) -> ::std::string::String {
        "<div id=\"__Fluent_UUID_02f9dc98_715e_44a7_b1cd_69467b01a5f5\"></div>\n".into()
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
        self.spawn_component___Fluent_UUID_02f9dc98_715e_44a7_b1cd_69467b01a5f5();
    }
}
