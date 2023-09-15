#![allow(warnings)]
use ::fluent_web_client::internal::web_sys::*;
use ::fluent_web_client::internal::DomDisplay;
use ::fluent_web_client::internal::UseInEvent;
use std::str::FromStr;
#[derive(::fluent_web_client::internal::Derivative)]
#[derivative(Clone(bound = ""))]
struct __Fluid_Data<T> {
    _p: ::std::marker::PhantomData<(T)>,
}
#[derive(::fluent_web_client::internal::Derivative)]
#[derivative(Default(bound = ""))]
struct __Fluid_Reactive_Functions<T>
where
    T: FromStr + UseInEvent + 'static,
{
    _p: ::std::marker::PhantomData<(T)>,
}
#[derive(::fluent_web_client::internal::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Component<T>
where
    T: FromStr + UseInEvent + 'static,
{
    root_name: ::std::string::String,
    data: __Fluid_Data<T>,
    updates: ::std::rc::Rc<::std::cell::RefCell<__Fluid_Reactive_Functions<T>>>,
}
trait __Fluent_Event<
    T,
>: ::fluent_web_client::internal::serde::Serialize + for<'a> ::fluent_web_client::internal::serde::Deserialize<
        'a,
    > {
    const NAME: &'static str;
    type Wrapper: ::fluent_web_client::internal::serde::Serialize
        + for<'a> ::fluent_web_client::internal::serde::Deserialize<'a>;
    fn wrap(self) -> Self::Wrapper;
}
#[derive(
    ::fluent_web_client::internal::serde::Serialize,
    ::fluent_web_client::internal::serde::Deserialize
)]
#[serde(crate = "::fluent_web_client::internal::serde")]
pub struct input<T> {
    pub value: T,
}
impl<T> __Fluent_Event<T> for input<T>
where
    T: FromStr + UseInEvent + 'static,
{
    const NAME: &'static str = "input";
    type Wrapper = __Fluent_Events::input<T>;
    fn wrap(self) -> Self::Wrapper {
        __Fluent_Events::input(self, ::std::marker::PhantomData)
    }
}
pub mod __Fluent_Events {
    #[derive(
        ::fluent_web_client::internal::serde::Serialize,
        ::fluent_web_client::internal::serde::Deserialize
    )]
    #[serde(crate = "::fluent_web_client::internal::serde")]
    pub struct input<T>(pub super::input<T>, pub ::std::marker::PhantomData<(T)>);
}
impl<T> Component<T>
where
    T: FromStr + UseInEvent + 'static,
{
    pub type __Fluent_Event_input = __Fluent_Events::input<T>;
    fn set_event___Fluent_UUID_40f9d20b_e543_452e_98a2_9982349a4fe4_interal(
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
                .dyn_ref::<::fluent_web_client::internal::web_sys::InputEvent>()
                .unwrap();
            {
                let __Fluid_Data { .. } = self.data.clone();
                {
                    if let Ok(value) = element.value().parse() {
                        self.emit(input { value });
                    }
                };
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
    fn set_event___Fluent_UUID_40f9d20b_e543_452e_98a2_9982349a4fe4(&self) {
        let __Fluent_Elements = ::fluent_web_client::internal::get_elements(
            &self.root_name,
            ".__Fluent_UUID_40f9d20b_e543_452e_98a2_9982349a4fe4",
        );
        for __Fluent_Element in __Fluent_Elements.into_iter() {
            self.clone()
                .set_event___Fluent_UUID_40f9d20b_e543_452e_98a2_9982349a4fe4_interal(
                    __Fluent_Element,
                );
        }
    }
    fn detect_reads(&self, f: fn(&Component<T>)) {
        let mut __Fluent_Updates = self.updates.borrow_mut();
        let __Fluid_Data { .. } = self.data.clone();
    }
    fn update_changed_values(&self) {
        let mut __Fluent_Updates = self.updates.borrow_mut();
        let __Fluid_Data { .. } = self.data.clone();
        let mut __Fluent_Functions: ::std::collections::HashSet<fn(&Component<T>)> = ::std::collections::HashSet::new();
        ::std::mem::drop(__Fluent_Updates);
        for func in __Fluent_Functions.into_iter() {
            func(self);
        }
    }
    fn emit<__Fluent_E: __Fluent_Event<T>>(&self, event: __Fluent_E) {
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
impl<T> ::fluent_web_client::internal::Component for Component<T>
where
    T: FromStr + UseInEvent + 'static,
{
    fn render_init(&self) -> ::std::string::String {
        let root = &self.root_name;
        format!(
            "<input class=\" __Fluent_UUID_40f9d20b_e543_452e_98a2_9982349a4fe4\">\n<style></style>"
        )
    }
    fn create(root_id: String) -> Self {
        Self {
            root_name: root_id,
            data: __Fluid_Data {
                _p: std::marker::PhantomData,
            },
            updates: ::std::rc::Rc::new(
                ::std::cell::RefCell::new(__Fluid_Reactive_Functions::default()),
            ),
        }
    }
    fn setup_events(&self) {
        self.set_event___Fluent_UUID_40f9d20b_e543_452e_98a2_9982349a4fe4();
    }
    fn update_all(&self) {}
    fn spawn_sub(&self) {}
}
