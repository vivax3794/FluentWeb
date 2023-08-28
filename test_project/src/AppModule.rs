use ::fluent_web_client::internal::web_sys::*;
#[derive(Clone)]
struct __Fluid_Data {
    value: ::std::rc::Rc<::std::cell::RefCell<i32>>,
}
#[derive(Clone)]
pub struct Component {
    root_name: ::std::string::String,
    data: __Fluid_Data,
}
impl Component {
    fn update_element___Fluent_UUID_fc16f909_4bb9_48ba_a61a_780a7ff5b40b(&self) {
        let __Fluid_Data { value } = &self.data;
        let value = value.borrow();
        let __Fluent_Element = ::fluent_web_client::internal::get_element(
            &self.root_name,
            "__Fluent_UUID_fc16f909_4bb9_48ba_a61a_780a7ff5b40b",
        );
        __Fluent_Element
            .set_text_content(
                ::std::option::Option::Some(
                    &::std::format!(
                        " {} * 2 = {}\n\n", ::fluent_web_client::internal::display(&
                        (value)), ::fluent_web_client::internal::display(& (* value * 2))
                    ),
                ),
            );
    }
    fn set_event___Fluent_UUID_0f875c0e_a8e0_4185_b514_a938f203055e(&self) {
        let __Fluent_Component = self.clone();
        use ::fluent_web_client::internal::wasm_bindgen::JsCast;
        let __Fluent_Element = ::fluent_web_client::internal::get_element(
            &self.root_name,
            "__Fluent_UUID_0f875c0e_a8e0_4185_b514_a938f203055e",
        );
        let __Fluent_Element: &::fluent_web_client::internal::web_sys::HtmlInputElement = __Fluent_Element
            .dyn_ref()
            .unwrap();
        let element = __Fluent_Element.clone();
        let __Fluent_Function = ::fluent_web_client::internal::wasm_bindgen::closure::Closure::<
            dyn Fn(_),
        >::new(move |event: ::fluent_web_client::internal::web_sys::Event| {
            let event = event
                .dyn_ref::<::fluent_web_client::internal::web_sys::KeyboardEvent>()
                .unwrap();
            {
                let __Fluid_Data { value } = &__Fluent_Component.data;
                let mut value = value.borrow_mut();
                {
                    if let Ok(input) = element.value().parse() {
                        *value = input;
                    }
                };
            }
            use ::fluent_web_client::internal::Component;
            __Fluent_Component.update_all();
        });
        __Fluent_Element
            .add_event_listener_with_callback(
                "keyup",
                __Fluent_Function.as_ref().unchecked_ref(),
            )
            .unwrap();
        __Fluent_Function.forget();
    }
}
impl ::fluent_web_client::internal::Component for Component {
    fn render_init(&self) -> ::std::string::String {
        "<input type=\"number\" id=\"__Fluent_UUID_0f875c0e_a8e0_4185_b514_a938f203055e\">\n\n    <br><span id=\"__Fluent_UUID_fc16f909_4bb9_48ba_a61a_780a7ff5b40b\"></span>"
            .into()
    }
    fn create(root_id: String) -> Self {
        Self {
            root_name: root_id,
            data: __Fluid_Data {
                value: ::std::rc::Rc::new(::std::cell::RefCell::new(0)),
            },
        }
    }
    fn setup_events(&self) {
        self.set_event___Fluent_UUID_0f875c0e_a8e0_4185_b514_a938f203055e();
    }
    fn update_all(&self) {
        self.update_element___Fluent_UUID_fc16f909_4bb9_48ba_a61a_780a7ff5b40b();
    }
    fn spawn_sub(&self) {}
}
