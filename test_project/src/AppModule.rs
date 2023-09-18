#![allow(warnings)]
use ::fluent_web_client::internal::web_sys::*;
use ::fluent_web_client::internal::DomDisplay;
use ::fluent_web_client::internal::UseInEvent;
use super::Sub;
#[derive(::fluent_web_client::internal::Derivative)]
#[derivative(Clone(bound = ""))]
struct __Fluid_Data {
    value: ::fluent_web_client::internal::ChangeDetector<u32>,
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
    fn spawn_component___Fluent_UUID_8cdb7b08_73af_47ff_9521_3cd5dfa17754(&self) {
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_8cdb7b08_73af_47ff_9521_3cd5dfa17754.__Fluent_Needs_Init",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            let __Fluent_Id = ::fluent_web_client::internal::uuid();
            __Fluent_Element.set_id(&__Fluent_Id);
            ::fluent_web_client::render_component::<Sub>(&__Fluent_Id);
        }
    }
    fn update_attribute___Fluent_UUID_936083bf_2042_4674_92cd_7e75ce98e341(&self) {
        let __Fluid_Data { value, .. } = self.data.clone();
        let value = value.borrow();
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_b330eccd_80c2_48f9_856c_3651bcf3e8ec",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            __Fluent_Element
                .set_attribute(
                    "value",
                    {
                        let __Fluent_Value = *value;
                        let __Fluent_Bytes = ::fluent_web_client::internal::bincode::serialize(
                                &__Fluent_Value,
                            )
                            .unwrap();
                        use ::fluent_web_client::internal::base64::engine::Engine;
                        &::fluent_web_client::internal::base64::engine::general_purpose::STANDARD_NO_PAD
                            .encode(__Fluent_Bytes)
                    },
                )
                .unwrap();
        }
        self.detect_reads(
            Component::update_attribute___Fluent_UUID_936083bf_2042_4674_92cd_7e75ce98e341,
        );
    }
    fn set_event___Fluent_UUID_e0b7d67b_f09e_43f1_beca_3be4dbd7da6d_interal(
        self,
        __Fluent_Element: ::fluent_web_client::internal::web_sys::Element,
    ) {
        use ::fluent_web_client::internal::wasm_bindgen::JsCast;
        let __Fluent_Element: &::fluent_web_client::internal::web_sys::HtmlButtonElement = __Fluent_Element
            .dyn_ref()
            .unwrap();
        let element = __Fluent_Element.clone();
        let __Fluent_Function = ::fluent_web_client::internal::wasm_bindgen::closure::Closure::<
            dyn Fn(_),
        >::new(move |event: ::fluent_web_client::internal::web_sys::Event| {
            let event = event
                .dyn_ref::<::fluent_web_client::internal::web_sys::Event>()
                .unwrap();
            {
                let __Fluid_Data { value, .. } = self.data.clone();
                let mut value = value.borrow_mut();
                { *value += 1 };
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
    fn set_event___Fluent_UUID_e0b7d67b_f09e_43f1_beca_3be4dbd7da6d(&self) {
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_f4317371_61ec_4a62_8db1_3381f42f2f9e",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            self.clone()
                .set_event___Fluent_UUID_e0b7d67b_f09e_43f1_beca_3be4dbd7da6d_interal(
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
            "<button class=\" __Fluent_UUID_f4317371_61ec_4a62_8db1_3381f42f2f9e\">+1</button>\n    <div class=\" __Fluent_UUID_8cdb7b08_73af_47ff_9521_3cd5dfa17754 __Fluent_Needs_Init __Fluent_UUID_b330eccd_80c2_48f9_856c_3651bcf3e8ec\"></div>\n<style></style>"
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
        self.set_event___Fluent_UUID_e0b7d67b_f09e_43f1_beca_3be4dbd7da6d();
        self.setup_watcher();
    }
    fn update_all(&self) {
        self.update_attribute___Fluent_UUID_936083bf_2042_4674_92cd_7e75ce98e341();
    }
    fn spawn_sub(&self) {
        self.spawn_component___Fluent_UUID_8cdb7b08_73af_47ff_9521_3cd5dfa17754();
    }
}
