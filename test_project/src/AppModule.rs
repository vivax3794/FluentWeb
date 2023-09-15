#![allow(warnings)]
use ::fluent_web_client::internal::web_sys::*;
use ::fluent_web_client::internal::DomDisplay;
use ::fluent_web_client::internal::UseInEvent;
use super::Input;
#[derive(::fluent_web_client::internal::Derivative)]
#[derivative(Clone(bound = ""))]
struct __Fluid_Data {
    value: ::fluent_web_client::internal::ChangeDetector<f32>,
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
    fn spawn_component___Fluent_UUID_3b25e3d4_12e5_42ec_b718_a3f2cdce3fa2(&self) {
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_3b25e3d4_12e5_42ec_b718_a3f2cdce3fa2.__Fluent_Needs_Init",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            let __Fluent_Id = ::fluent_web_client::internal::uuid();
            __Fluent_Element.set_id(&__Fluent_Id);
            ::fluent_web_client::render_component::<Input<f32>>(&__Fluent_Id);
        }
    }
    fn update_element___Fluent_UUID_45512210_4983_406a_a7ce_54f9447cf3d2(&self) {
        let __Fluid_Data { value, .. } = self.data.clone();
        let value = value.borrow();
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_45512210_4983_406a_a7ce_54f9447cf3d2",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            let __Fluent_Text = &::std::format!(
                "\n    Double: {}\n", ::fluent_web_client::internal::display(& (* value *
                2.0))
            );
            __Fluent_Element
                .set_text_content(::std::option::Option::Some(__Fluent_Text));
        }
        self.detect_reads(
            Component::update_element___Fluent_UUID_45512210_4983_406a_a7ce_54f9447cf3d2,
        );
    }
    fn set_event___Fluent_UUID_42392cd6_3dcf_4b1a_a6ed_0dbfae510c11_interal(
        self,
        __Fluent_Element: ::fluent_web_client::internal::web_sys::Element,
    ) {
        use ::fluent_web_client::internal::wasm_bindgen::JsCast;
        let __Fluent_Element: &::fluent_web_client::internal::web_sys::HtmlDivElement = __Fluent_Element
            .dyn_ref()
            .unwrap();
        let element = __Fluent_Element.clone();
        let __Fluent_Function = ::fluent_web_client::internal::wasm_bindgen::closure::Closure::<
            dyn Fn(_),
        >::new(move |event: ::fluent_web_client::internal::web_sys::Event| {
            let __Fluent_Custom_Event = event
                .dyn_ref::<::fluent_web_client::internal::web_sys::CustomEvent>()
                .unwrap();
            let __Fluent_Details = __Fluent_Custom_Event.detail();
            let event: Input<f32>::__Fluent_Event_input = ::fluent_web_client::internal::serde_wasm_bindgen::from_value(
                    __Fluent_Details,
                )
                .unwrap();
            let event = event.0;
            {
                let __Fluid_Data { value, .. } = self.data.clone();
                let mut value = value.borrow_mut();
                { *value = event.value };
            }
            use ::fluent_web_client::internal::Component;
            self.update_changed_values();
        });
        __Fluent_Element
            .add_event_listener_with_callback(
                "input",
                __Fluent_Function.as_ref().unchecked_ref(),
            )
            .unwrap();
        __Fluent_Function.forget();
    }
    fn set_event___Fluent_UUID_42392cd6_3dcf_4b1a_a6ed_0dbfae510c11(&self) {
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_42392cd6_3dcf_4b1a_a6ed_0dbfae510c11",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            self.clone()
                .set_event___Fluent_UUID_42392cd6_3dcf_4b1a_a6ed_0dbfae510c11_interal(
                    __Fluent_Element,
                );
        }
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
            "<div class=\" __Fluent_UUID_42392cd6_3dcf_4b1a_a6ed_0dbfae510c11 __Fluent_UUID_3b25e3d4_12e5_42ec_b718_a3f2cdce3fa2 __Fluent_Needs_Init\" @input=\"*value = event.value\"></div><span class=\"__Fluent_UUID_45512210_4983_406a_a7ce_54f9447cf3d2\"></span><style></style>"
        )
    }
    fn create(root_id: String) -> Self {
        Self {
            root_name: root_id,
            data: __Fluid_Data {
                value: ::fluent_web_client::internal::ChangeDetector::new(0.0),
                _p: std::marker::PhantomData,
            },
            updates: ::std::rc::Rc::new(
                ::std::cell::RefCell::new(__Fluid_Reactive_Functions::default()),
            ),
        }
    }
    fn setup_events(&self) {
        self.set_event___Fluent_UUID_42392cd6_3dcf_4b1a_a6ed_0dbfae510c11();
    }
    fn update_all(&self) {
        self.update_element___Fluent_UUID_45512210_4983_406a_a7ce_54f9447cf3d2();
    }
    fn spawn_sub(&self) {
        self.spawn_component___Fluent_UUID_3b25e3d4_12e5_42ec_b718_a3f2cdce3fa2();
    }
}
