#![allow(warnings)]
use ::fluent_web_client::internal::web_sys::*;
#[derive(Clone)]
struct __Fluid_Data {
    name: ::std::rc::Rc<::std::cell::RefCell<String>>,
    sender: ::std::rc::Rc<::std::cell::RefCell<String>>,
}
#[derive(Clone)]
pub struct Component {
    root_name: ::std::string::String,
    data: __Fluid_Data,
}
impl Component {
    fn update_element___Fluent_UUID_ec274cb4_8f87_4467_9891_fbfbf2f0ebf5(&self) {
        ::fluent_web_client::internal::log(
            "__Fluent_UUID_ec274cb4_8f87_4467_9891_fbfbf2f0ebf5",
        );
        let __Fluid_Data { name, sender } = &self.data;
        let name = ::fluent_web_client::internal::ReadDetector::new(name.borrow());
        let sender = ::fluent_web_client::internal::ReadDetector::new(sender.borrow());
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_ec274cb4_8f87_4467_9891_fbfbf2f0ebf5",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            let __Fluent_Text = &::std::format!(
                "Hello {}", ::fluent_web_client::internal::display(& (name))
            );
            __Fluent_Element
                .set_text_content(::std::option::Option::Some(__Fluent_Text));
        }
    }
    fn update_element___Fluent_UUID_35af7d43_d1b1_4c0c_ba41_7ff37df466f3(&self) {
        ::fluent_web_client::internal::log(
            "__Fluent_UUID_35af7d43_d1b1_4c0c_ba41_7ff37df466f3",
        );
        let __Fluid_Data { name, sender } = &self.data;
        let name = ::fluent_web_client::internal::ReadDetector::new(name.borrow());
        let sender = ::fluent_web_client::internal::ReadDetector::new(sender.borrow());
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_35af7d43_d1b1_4c0c_ba41_7ff37df466f3",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            let __Fluent_Text = &::std::format!(
                "I am {} :D", ::fluent_web_client::internal::display(& (sender))
            );
            __Fluent_Element
                .set_text_content(::std::option::Option::Some(__Fluent_Text));
        }
    }
    fn set_event___Fluent_UUID_9d965ea4_d680_4f77_bb70_e17b7f385d09(&self) {
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_9d965ea4_d680_4f77_bb70_e17b7f385d09",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            use ::fluent_web_client::internal::wasm_bindgen::JsCast;
            let __Fluent_Component = self.clone();
            let __Fluent_Element: &::fluent_web_client::internal::web_sys::HtmlInputElement = __Fluent_Element
                .dyn_ref()
                .unwrap();
            let element = __Fluent_Element.clone();
            let __Fluent_Function = ::fluent_web_client::internal::wasm_bindgen::closure::Closure::<
                dyn Fn(_),
            >::new(move |event: ::fluent_web_client::internal::web_sys::Event| {
                let event = event
                    .dyn_ref::<::fluent_web_client::internal::web_sys::InputEvent>()
                    .unwrap();
                {
                    let __Fluid_Data { name, sender } = &__Fluent_Component.data;
                    let mut name = ::fluent_web_client::internal::WriteDetector::new(
                        name.borrow_mut(),
                    );
                    let mut sender = ::fluent_web_client::internal::WriteDetector::new(
                        sender.borrow_mut(),
                    );
                    { *sender = element.value() };
                }
                use ::fluent_web_client::internal::Component;
                __Fluent_Component.update_all();
            });
            __Fluent_Element
                .add_event_listener_with_callback(
                    "input",
                    __Fluent_Function.as_ref().unchecked_ref(),
                )
                .unwrap();
            __Fluent_Function.forget();
        }
    }
}
impl ::fluent_web_client::internal::Component for Component {
    fn render_init(&self) -> ::std::string::String {
        let root = &self.root_name;
        format!(
            "<h1>Test Test</h1>\n    <h2><span class=\"__Fluent_UUID_ec274cb4_8f87_4467_9891_fbfbf2f0ebf5\"></span></h2>\n    <p><span class=\"__Fluent_UUID_35af7d43_d1b1_4c0c_ba41_7ff37df466f3\"></span></p>\n\n    <input class=\" __Fluent_UUID_9d965ea4_d680_4f77_bb70_e17b7f385d09\">\n<style></style>"
        )
    }
    fn create(root_id: String) -> Self {
        Self {
            root_name: root_id,
            data: __Fluid_Data {
                name: ::std::rc::Rc::new(::std::cell::RefCell::new("World".into())),
                sender: ::std::rc::Rc::new(::std::cell::RefCell::new("".into())),
            },
        }
    }
    fn setup_events(&self) {
        self.set_event___Fluent_UUID_9d965ea4_d680_4f77_bb70_e17b7f385d09();
    }
    fn update_all(&self) {
        self.update_element___Fluent_UUID_ec274cb4_8f87_4467_9891_fbfbf2f0ebf5();
        self.update_element___Fluent_UUID_35af7d43_d1b1_4c0c_ba41_7ff37df466f3();
    }
    fn spawn_sub(&self) {}
}
