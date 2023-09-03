#![allow(warnings)]
use ::fluent_web_client::internal::web_sys::*;
#[derive(Clone)]
struct __Fluid_Data {}
#[derive(Clone)]
pub struct Component {
    root_name: ::std::string::String,
    data: __Fluid_Data,
}
impl Component {
    fn spawn_component___Fluent_UUID_e869af1f_2cfb_415f_bfa2_9127bea9cafa(&self) {
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_e869af1f_2cfb_415f_bfa2_9127bea9cafa.__Fluent_Needs_Init",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            let __Fluent_Id = ::fluent_web_client::internal::uuid();
            __Fluent_Element.set_id(&__Fluent_Id);
            ::fluent_web_client::render_component::<super::Sub>(&__Fluent_Id);
        }
    }
}
impl ::fluent_web_client::internal::Component for Component {
    fn render_init(&self) -> ::std::string::String {
        "<p>Main</p>\n    <div class=\" __Fluent_UUID_e869af1f_2cfb_415f_bfa2_9127bea9cafa __Fluent_Needs_Init\"></div>\n<style>p{color:#00f}</style>"
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
        self.spawn_component___Fluent_UUID_e869af1f_2cfb_415f_bfa2_9127bea9cafa();
    }
}
