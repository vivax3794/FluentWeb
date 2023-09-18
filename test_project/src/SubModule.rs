#![allow(warnings)]
use ::fluent_web_client::internal::web_sys::*;
use ::fluent_web_client::internal::DomDisplay;
use ::fluent_web_client::internal::UseInEvent;
#[derive(::fluent_web_client::internal::Derivative)]
#[derivative(Clone(bound = ""))]
struct __Fluid_Data {
    value: ::fluent_web_client::internal::ChangeDetector<i32>,
    _p: ::std::marker::PhantomData<()>,
}
#[derive(::fluent_web_client::internal::Derivative)]
#[derivative(Default(bound = ""))]
struct __Fluid_Reactive_Functions {
    value: ::std::collections::HashSet<fn(&Component)>,
    _p: ::std::marker::PhantomData<()>,
}
#[derive(::fluent_web_client::internal::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Component {
    root_name: ::std::string::String,
    data: __Fluid_Data,
    updates: ::std::rc::Rc<::std::cell::RefCell<__Fluid_Reactive_Functions>>,
}
trait __Fluent_Event: ::fluent_web_client::internal::serde::Serialize + for<'a> ::fluent_web_client::internal::serde::Deserialize<
        'a,
    > {
    const NAME: &'static str;
    type Wrapper: ::fluent_web_client::internal::serde::Serialize
        + for<'a> ::fluent_web_client::internal::serde::Deserialize<'a>;
    fn wrap(self) -> Self::Wrapper;
}
pub mod __Fluent_Events {}
impl Component {
    fn update_element___Fluent_UUID_f7dc3383_d7c6_4ee3_b792_07d985e209a3(&self) {
        let __Fluid_Data { value, .. } = self.data.clone();
        let value = value.borrow();
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_f7dc3383_d7c6_4ee3_b792_07d985e209a3",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            let __Fluent_Text = &::std::format!(
                "{}", ::fluent_web_client::internal::display(& (value))
            );
            __Fluent_Element
                .set_text_content(::std::option::Option::Some(__Fluent_Text));
        }
        self.detect_reads(
            Component::update_element___Fluent_UUID_f7dc3383_d7c6_4ee3_b792_07d985e209a3,
        );
    }
    fn detect_reads(&self, f: fn(&Component)) {
        let mut __Fluent_Updates = self.updates.borrow_mut();
        let __Fluid_Data { value, .. } = self.data.clone();
        if value.was_read() {
            __Fluent_Updates.value.insert(f);
        }
        value.clear();
    }
    fn update_changed_values(&self) {
        let mut __Fluent_Updates = self.updates.borrow_mut();
        let __Fluid_Data { value, .. } = self.data.clone();
        let mut __Fluent_Functions: ::std::collections::HashSet<fn(&Component)> = ::std::collections::HashSet::new();
        if value.was_written() {
            __Fluent_Functions.extend(__Fluent_Updates.value.iter());
        }
        value.clear();
        ::std::mem::drop(__Fluent_Updates);
        for func in __Fluent_Functions.into_iter() {
            func(self);
        }
    }
    fn setup_watcher(&self) {
        use ::fluent_web_client::internal::wasm_bindgen::JsCast;
        let component = self.clone();
        let function = move || component.update_props();
        let function = ::fluent_web_client::internal::wasm_bindgen::closure::Closure::<
            dyn Fn(),
        >::new(function);
        let js_function = function.as_ref().unchecked_ref();
        let observer = ::fluent_web_client::internal::web_sys::MutationObserver::new(
                js_function,
            )
            .unwrap();
        function.forget();
        let element = ::fluent_web_client::internal::get_by_id(&self.root_name);
        let mut options = ::fluent_web_client::internal::web_sys::MutationObserverInit::new();
        options.attributes(true);
        observer.observe_with_options(&element, &options);
    }
    fn update_props(&self) {
        let element = ::fluent_web_client::internal::get_by_id(&self.root_name);
        if let Some(value) = element.get_attribute("value") {
            use ::fluent_web_client::internal::base64::engine::Engine;
            let decoded = ::fluent_web_client::internal::base64::engine::general_purpose::STANDARD_NO_PAD
                .decode(value)
                .unwrap();
            let deserialized = ::fluent_web_client::internal::bincode::deserialize(
                    &decoded,
                )
                .unwrap();
            *self.data.value.borrow_mut() = deserialized;
        }
        self.update_changed_values();
    }
    fn emit<__Fluent_E: __Fluent_Event>(&self, event: __Fluent_E) {
        use ::fluent_web_client::internal::web_sys;
        let root_element = ::fluent_web_client::internal::get_by_id(&self.root_name);
        let data = ::fluent_web_client::internal::bincode::serialize(&event.wrap())
            .unwrap();
        let data = ::fluent_web_client::internal::js_sys::Uint8Array::from(
            data.as_slice(),
        );
        let event = web_sys::CustomEvent::new_with_event_init_dict(
                __Fluent_E::NAME,
                &web_sys::CustomEventInit::new().detail(&data),
            )
            .unwrap();
        root_element.dispatch_event(&event).unwrap();
    }
}
impl ::fluent_web_client::internal::Component for Component {
    fn render_init(&self) -> ::std::string::String {
        let root = &self.root_name;
        ::std::format!(
            "<h1><span class=\"__Fluent_UUID_f7dc3383_d7c6_4ee3_b792_07d985e209a3\"></span></h1>\n<style></style>"
        )
    }
    fn create(root_id: String) -> Self {
        Self {
            root_name: root_id,
            data: __Fluid_Data {
                value: ::fluent_web_client::internal::ChangeDetector::new(0),
                _p: std::marker::PhantomData,
            },
            updates: ::std::rc::Rc::new(
                ::std::cell::RefCell::new(__Fluid_Reactive_Functions::default()),
            ),
        }
    }
    fn setup_events(&self) {
        self.setup_watcher();
    }
    fn update_all(&self) {
        self.update_element___Fluent_UUID_f7dc3383_d7c6_4ee3_b792_07d985e209a3();
    }
    fn spawn_sub(&self) {}
}
