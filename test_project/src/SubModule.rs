#![allow(warnings)]
use ::fluent_web_client::internal::web_sys::*;
#[derive(Clone)]
struct __Fluid_Data {
    counter: ::std::rc::Rc<::std::cell::RefCell<u32>>,
}
#[derive(Clone)]
pub struct Component {
    root_name: ::std::string::String,
    data: __Fluid_Data,
}
impl Component {
    fn update_element___Fluent_UUID_0a1952d6_62cb_4aca_893c_2668233c7b18(&self) {
        let __Fluid_Data { counter } = &self.data;
        let counter = counter.borrow();
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            "__Fluent_UUID_0a1952d6_62cb_4aca_893c_2668233c7b18",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            let __Fluent_Text = &::std::format!(
                "{}", ::fluent_web_client::internal::display(& (counter))
            );
            __Fluent_Element
                .set_text_content(::std::option::Option::Some(__Fluent_Text));
        }
    }
    fn set_event___Fluent_UUID_91aa6be5_9e36_4500_8e70_00a238792506(&self) {
        let __Fluent_Component = self.clone();
        use ::fluent_web_client::internal::wasm_bindgen::JsCast;
        let __Fluent_Element = ::fluent_web_client::internal::get_element(
            &self.root_name,
            "__Fluent_UUID_91aa6be5_9e36_4500_8e70_00a238792506",
        );
        let __Fluent_Element: &::fluent_web_client::internal::web_sys::HtmlButtonElement = __Fluent_Element
            .dyn_ref()
            .unwrap();
        let element = __Fluent_Element.clone();
        let __Fluent_Function = ::fluent_web_client::internal::wasm_bindgen::closure::Closure::<
            dyn Fn(_),
        >::new(move |event: ::fluent_web_client::internal::web_sys::Event| {
            let event = event
                .dyn_ref::<::fluent_web_client::internal::web_sys::MouseEvent>()
                .unwrap();
            {
                let __Fluid_Data { counter } = &__Fluent_Component.data;
                let mut counter = counter.borrow_mut();
                { *counter += 1 };
            }
            use ::fluent_web_client::internal::Component;
            __Fluent_Component.update_all();
        });
        __Fluent_Element
            .add_event_listener_with_callback(
                "click",
                __Fluent_Function.as_ref().unchecked_ref(),
            )
            .unwrap();
        __Fluent_Function.forget();
    }
}
impl ::fluent_web_client::internal::Component for Component {
    fn render_init(&self) -> ::std::string::String {
        "<button id=\"__Fluent_UUID_91aa6be5_9e36_4500_8e70_00a238792506\"><span class=\"__Fluent_UUID_0a1952d6_62cb_4aca_893c_2668233c7b18\"></span></button>\n"
            .into()
    }
    fn create(root_id: String) -> Self {
        Self {
            root_name: root_id,
            data: __Fluid_Data {
                counter: ::std::rc::Rc::new(::std::cell::RefCell::new(0)),
            },
        }
    }
    fn setup_events(&self) {
        self.set_event___Fluent_UUID_91aa6be5_9e36_4500_8e70_00a238792506();
    }
    fn update_all(&self) {
        self.update_element___Fluent_UUID_0a1952d6_62cb_4aca_893c_2668233c7b18();
    }
    fn spawn_sub(&self) {}
}
