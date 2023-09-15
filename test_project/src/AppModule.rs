#![allow(warnings)]
use ::fluent_web_client::internal::web_sys::*;
use ::fluent_web_client::internal::DomDisplay;
use ::fluent_web_client::internal::UseInEvent;
#[derive(::fluent_web_client::internal::Derivative)]
#[derivative(Clone(bound = ""))]
struct __Fluid_Data {
    hide: ::fluent_web_client::internal::ChangeDetector<bool>,
    _p: ::std::marker::PhantomData<()>,
}
#[derive(::fluent_web_client::internal::Derivative)]
#[derivative(Default(bound = ""))]
struct __Fluid_Reactive_Functions {
    hide: ::std::collections::HashSet<fn(&Component)>,
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
    fn set_event___Fluent_UUID_67c1c2ed_e72f_40d7_a119_f4e99215dcfa_interal(
        self,
        __Fluent_Element: ::fluent_web_client::internal::web_sys::Element,
    ) {
        use ::fluent_web_client::internal::wasm_bindgen::JsCast;
        let __Fluent_Element: &::fluent_web_client::internal::web_sys::HtmlInputElement = __Fluent_Element
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
                let __Fluid_Data { hide, .. } = self.data.clone();
                let mut hide = hide.borrow_mut();
                { *hide = element.checked() };
            }
            use ::fluent_web_client::internal::Component;
            self.update_changed_values();
        });
        __Fluent_Element
            .add_event_listener_with_callback(
                "click",
                __Fluent_Function.as_ref().unchecked_ref(),
            )
            .unwrap();
        __Fluent_Function.forget();
    }
    fn set_event___Fluent_UUID_67c1c2ed_e72f_40d7_a119_f4e99215dcfa(&self) {
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_67c1c2ed_e72f_40d7_a119_f4e99215dcfa",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            self.clone()
                .set_event___Fluent_UUID_67c1c2ed_e72f_40d7_a119_f4e99215dcfa_interal(
                    __Fluent_Element,
                );
        }
    }
    fn detect_reads(&self, f: fn(&Component)) {
        let mut __Fluent_Updates = self.updates.borrow_mut();
        let __Fluid_Data { hide, .. } = self.data.clone();
        if hide.was_read() {
            __Fluent_Updates.hide.insert(f);
        }
        hide.clear();
    }
    fn update_changed_values(&self) {
        let mut __Fluent_Updates = self.updates.borrow_mut();
        let __Fluid_Data { hide, .. } = self.data.clone();
        let mut __Fluent_Functions: ::std::collections::HashSet<fn(&Component)> = ::std::collections::HashSet::new();
        if hide.was_written() {
            __Fluent_Functions.extend(__Fluent_Updates.hide.iter());
        }
        hide.clear();
        ::std::mem::drop(__Fluent_Updates);
        for func in __Fluent_Functions.into_iter() {
            func(self);
        }
    }
    fn emit<__Fluent_E: __Fluent_Event>(&self, event: __Fluent_E) {
        use ::fluent_web_client::internal::web_sys;
        let root_element = ::fluent_web_client::internal::get_by_id(&self.root_name);
        let data = ::fluent_web_client::internal::serde_wasm_bindgen::to_value(
                &event.wrap(),
            )
            .unwrap();
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
        format!(
            "<input type=\"checkbox\" checked=\"\" class=\" __Fluent_UUID_67c1c2ed_e72f_40d7_a119_f4e99215dcfa\">\n    <input =type=\"\n    if hide { &quot;password&quot; } else { &quot;text&quot; }\n    \" value=\"SecretPassword\">\n<style></style>"
        )
    }
    fn create(root_id: String) -> Self {
        Self {
            root_name: root_id,
            data: __Fluid_Data {
                hide: ::fluent_web_client::internal::ChangeDetector::new(true),
                _p: std::marker::PhantomData,
            },
            updates: ::std::rc::Rc::new(
                ::std::cell::RefCell::new(__Fluid_Reactive_Functions::default()),
            ),
        }
    }
    fn setup_events(&self) {
        self.set_event___Fluent_UUID_67c1c2ed_e72f_40d7_a119_f4e99215dcfa();
    }
    fn update_all(&self) {}
    fn spawn_sub(&self) {}
}
