#![allow(warnings)]
use ::fluent_web_client::internal::web_sys::*;
#[derive(Clone)]
struct __Fluid_Data {
    name: ::fluent_web_client::internal::ChangeDetector<String>,
    sender: ::fluent_web_client::internal::ChangeDetector<String>,
}
#[derive(Default)]
struct __Fluid_Reactive_Functions {
    name: ::std::collections::HashSet<fn(&Component)>,
    sender: ::std::collections::HashSet<fn(&Component)>,
}
#[derive(Clone)]
pub struct Component {
    root_name: ::std::string::String,
    data: __Fluid_Data,
    updates: ::std::rc::Rc<::std::cell::RefCell<__Fluid_Reactive_Functions>>,
}
impl Component {
    fn update_element___Fluent_UUID_ef105942_cd97_404c_8b1d_014278bb71c0(&self) {
        let __Fluid_Data { name, sender } = self.data.clone();
        let name = name.borrow();
        let sender = sender.borrow();
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_ef105942_cd97_404c_8b1d_014278bb71c0",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            let __Fluent_Text = &::std::format!(
                "Hello {}", ::fluent_web_client::internal::display(& (name))
            );
            __Fluent_Element
                .set_text_content(::std::option::Option::Some(__Fluent_Text));
        }
        self.detect_reads(
            Component::update_element___Fluent_UUID_ef105942_cd97_404c_8b1d_014278bb71c0,
        );
    }
    fn update_element___Fluent_UUID_50854a71_1cfc_48c8_9127_6f15d7769544(&self) {
        let __Fluid_Data { name, sender } = self.data.clone();
        let name = name.borrow();
        let sender = sender.borrow();
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_50854a71_1cfc_48c8_9127_6f15d7769544",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            let __Fluent_Text = &::std::format!(
                "I am {} and I love {}", ::fluent_web_client::internal::display(&
                (sender)), ::fluent_web_client::internal::display(& (name))
            );
            __Fluent_Element
                .set_text_content(::std::option::Option::Some(__Fluent_Text));
        }
        self.detect_reads(
            Component::update_element___Fluent_UUID_50854a71_1cfc_48c8_9127_6f15d7769544,
        );
    }
    fn set_event___Fluent_UUID_e78a6dc6_8b41_4d8b_8204_6f0ef10ab125(&self) {
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_e78a6dc6_8b41_4d8b_8204_6f0ef10ab125",
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
                    let __Fluid_Data { name, sender } = __Fluent_Component.data.clone();
                    let mut name = name.borrow_mut();
                    let mut sender = sender.borrow_mut();
                    { *sender = element.value() };
                }
                use ::fluent_web_client::internal::Component;
                __Fluent_Component.update_changed_values();
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
    fn set_event___Fluent_UUID_0ff45232_68e4_4d46_95f6_22aefcae4c23(&self) {
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_0ff45232_68e4_4d46_95f6_22aefcae4c23",
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
                    let __Fluid_Data { name, sender } = __Fluent_Component.data.clone();
                    let mut name = name.borrow_mut();
                    let mut sender = sender.borrow_mut();
                    { *name = element.value() };
                }
                use ::fluent_web_client::internal::Component;
                __Fluent_Component.update_changed_values();
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
    fn detect_reads(&self, f: fn(&Component)) {
        let mut __Fluent_Updates = self.updates.borrow_mut();
        let __Fluid_Data { name, sender } = self.data.clone();
        if name.was_read() {
            __Fluent_Updates.name.insert(f);
        }
        name.clear();
        if sender.was_read() {
            __Fluent_Updates.sender.insert(f);
        }
        sender.clear();
    }
    fn update_changed_values(&self) {
        let mut __Fluent_Updates = self.updates.borrow_mut();
        let __Fluid_Data { name, sender } = self.data.clone();
        let mut __Fluent_Functions: ::std::collections::HashSet<fn(&Component)> = ::std::collections::HashSet::new();
        if name.was_written() {
            __Fluent_Functions.extend(__Fluent_Updates.name.iter());
        }
        name.clear();
        if sender.was_written() {
            __Fluent_Functions.extend(__Fluent_Updates.sender.iter());
        }
        sender.clear();
        ::std::mem::drop(__Fluent_Updates);
        for func in __Fluent_Functions.into_iter() {
            func(self);
        }
    }
}
impl ::fluent_web_client::internal::Component for Component {
    fn render_init(&self) -> ::std::string::String {
        let root = &self.root_name;
        format!(
            "<h1>Test Test</h1>\n    <h2><span class=\"__Fluent_UUID_ef105942_cd97_404c_8b1d_014278bb71c0\"></span></h2>\n    <p><span class=\"__Fluent_UUID_50854a71_1cfc_48c8_9127_6f15d7769544\"></span></p>\n\n    <input class=\" __Fluent_UUID_e78a6dc6_8b41_4d8b_8204_6f0ef10ab125\">\n    <input class=\" __Fluent_UUID_0ff45232_68e4_4d46_95f6_22aefcae4c23\">\n<style></style>"
        )
    }
    fn create(root_id: String) -> Self {
        Self {
            root_name: root_id,
            data: __Fluid_Data {
                name: ::fluent_web_client::internal::ChangeDetector::new("".into()),
                sender: ::fluent_web_client::internal::ChangeDetector::new("".into()),
            },
            updates: ::std::rc::Rc::new(
                ::std::cell::RefCell::new(__Fluid_Reactive_Functions::default()),
            ),
        }
    }
    fn setup_events(&self) {
        self.set_event___Fluent_UUID_e78a6dc6_8b41_4d8b_8204_6f0ef10ab125();
        self.set_event___Fluent_UUID_0ff45232_68e4_4d46_95f6_22aefcae4c23();
    }
    fn update_all(&self) {
        self.update_element___Fluent_UUID_ef105942_cd97_404c_8b1d_014278bb71c0();
        self.update_element___Fluent_UUID_50854a71_1cfc_48c8_9127_6f15d7769544();
    }
    fn spawn_sub(&self) {}
}
