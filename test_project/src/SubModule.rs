use ::fluent_web_client::internal::web_sys::*;
#[derive(Clone)]
struct __Fluid_Data {
    number: ::std::rc::Rc<::std::cell::RefCell<u32>>,
}
#[derive(Clone)]
pub struct Component {
    root_name: ::std::string::String,
    data: __Fluid_Data,
}
impl Component {
    fn update_element___Fluent_UUID_e008ad08_fdcc_4dbf_8058_16dd1716146f(&self) {
        let __Fluid_Data { number } = &self.data;
        let number = *number.borrow();
        let __Fluent_Element = ::fluent_web_client::internal::get_element(
            &self.root_name,
            "__Fluent_UUID_e008ad08_fdcc_4dbf_8058_16dd1716146f",
        );
        __Fluent_Element
            .set_text_content(
                ::std::option::Option::Some(
                    &::std::format!("{}", ::fluent_web_client::internal::display(number)),
                ),
            );
    }
    fn set_event___Fluent_UUID_a43c0b8a_6bab_4414_9868_57e339628350(&self) {
        let __Fluent_Component = self.clone();
        use ::fluent_web_client::internal::wasm_bindgen::JsCast;
        let __Fluent_Element = ::fluent_web_client::internal::get_element(
            &self.root_name,
            "__Fluent_UUID_a43c0b8a_6bab_4414_9868_57e339628350",
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
                let __Fluid_Data { number } = &__Fluent_Component.data;
                let mut number = number.borrow_mut();
                { *number += 1 };
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
        "<button id=\"__Fluent_UUID_a43c0b8a_6bab_4414_9868_57e339628350\"><span id=\"__Fluent_UUID_e008ad08_fdcc_4dbf_8058_16dd1716146f\"></span></button>\n"
            .into()
    }
    fn create(root_id: String) -> Self {
        Self {
            root_name: root_id,
            data: __Fluid_Data {
                number: ::std::rc::Rc::new(::std::cell::RefCell::new(0)),
            },
        }
    }
    fn setup_events(&self) {
        self.set_event___Fluent_UUID_a43c0b8a_6bab_4414_9868_57e339628350();
    }
    fn update_all(&self) {
        self.update_element___Fluent_UUID_e008ad08_fdcc_4dbf_8058_16dd1716146f();
    }
    fn spawn_sub(&self) {}
}
