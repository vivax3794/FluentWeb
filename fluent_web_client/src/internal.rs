// This code is called by generated code and should always be sound.
#![allow(clippy::unwrap_used)]

use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub use base64;
pub use bincode;
pub use derivative::Derivative;
pub use js_sys;
pub use serde;
pub use wasm_bindgen;
pub use web_sys;

use wasm_bindgen::JsCast;

pub trait Component: Clone {
    fn render_init(&self) -> String;
    fn create(root_id: String) -> Self;

    fn setup_onetime(&self, root: Option<String>);
    fn update_all(&self, root: Option<String>);
    fn update_props(&self);

    fn setup_watcher(component: Self, root_name: &str)
    where
        Self: 'static,
    {
        let function = move || component.update_props();
        let function =
            wasm_bindgen::closure::Closure::<dyn Fn()>::new(function);
        let js_function = function.as_ref().unchecked_ref();
        let observer =
            web_sys::MutationObserver::new(js_function).unwrap();
        function.forget();

        let element = get_by_id(root_name);

        let mut options = web_sys::MutationObserverInit::new();
        options.attributes(true);
        observer.observe_with_options(&element, &options).unwrap();
    }
}

pub fn render_component<C: Component + 'static>(mount_point: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.get_element_by_id(mount_point).unwrap();

    element.class_list().add_1("__Fluent_Component").unwrap();
    element
        .class_list()
        .remove_1("__Fluent_Needs_Init")
        .unwrap();

    let component = C::create(mount_point.to_owned());
    element.set_inner_html(&component.render_init());
    C::setup_watcher(component.clone(), mount_point);
    component.setup_onetime(None);
    component.update_all(None);
}

#[must_use]
pub fn uuid() -> String {
    let id = uuid::Uuid::new_v4().to_string();
    format!("__Fluent_UUID_{id}")
}

#[must_use]
pub fn get_by_id(id: &str) -> web_sys::Element {
    let selector = ::std::format!("#{id}");
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document.query_selector(&selector).unwrap().unwrap()
}

#[must_use]
pub fn get_element(
    component_id: &str,
    element_id: &str,
) -> web_sys::Element {
    let selector = ::std::format!(
        "#{component_id} #{element_id}:not(#{component_id} .__Fluent_Component *)"
    );
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document.query_selector(&selector).unwrap().unwrap()
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn get_elements(
    component_id: &str,
    selector: &str,
    root_selector: Option<String>,
) -> Vec<web_sys::Element> {
    let selector = ::std::format!(
        "#{} {} {}:not(#{} .__Fluent_Component *)",
        component_id,
        root_selector.unwrap_or_default(),
        selector,
        component_id
    );
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let node_list = document.query_selector_all(&selector).unwrap();

    let length = node_list.length() as usize;
    let mut elements = Vec::with_capacity(length);
    for i in 0..length {
        elements.push(
            node_list
                .item(i as u32)
                .unwrap()
                .dyn_into::<web_sys::Element>()
                .unwrap(),
        );
    }
    elements
}

pub trait DomDisplay {
    fn dom_display(&self) -> String;
}

#[cfg(feature = "nightly")]
impl<T> DomDisplay for T
where
    T: Debug,
{
    #[inline(always)]
    default fn dom_display(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T> DomDisplay for T
where
    T: Debug + Display,
{
    fn dom_display(&self) -> String {
        format!("{self}")
    }
}

pub fn display<T: DomDisplay>(value: &T) -> String {
    value.dom_display()
}

pub fn log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}

#[derive(Derivative, Debug)]
#[derivative(Clone(bound = ""))]
pub struct ChangeDetector<T> {
    value: Rc<RefCell<T>>,
    read: Rc<RefCell<bool>>,
    write: Rc<RefCell<bool>>,
}

impl<T> ChangeDetector<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            read: Rc::new(RefCell::new(false)),
            write: Rc::new(RefCell::new(false)),
        }
    }

    #[must_use]
    pub fn was_read(&self) -> bool {
        *self.read.borrow()
    }

    #[must_use]
    pub fn was_written(&self) -> bool {
        *self.write.borrow()
    }

    pub fn clear(&self) {
        *self.read.borrow_mut() = false;
        *self.write.borrow_mut() = false;
    }

    #[must_use]
    pub fn borrow(&self) -> ChangeDetectorRead<T> {
        ChangeDetectorRead {
            value: self.value.borrow(),
            read: Rc::clone(&self.read),
        }
    }

    #[must_use]
    pub fn borrow_mut(&self) -> ChangeDetectorWrite<T> {
        ChangeDetectorWrite {
            value: self.value.borrow_mut(),
            read: Rc::clone(&self.read),
            write: Rc::clone(&self.write),
        }
    }
}

#[derive(Debug)]
pub struct ChangeDetectorRead<'a, T> {
    value: Ref<'a, T>,
    read: Rc<RefCell<bool>>,
}

#[derive(Debug)]
pub struct ChangeDetectorWrite<'a, T> {
    value: RefMut<'a, T>,
    read: Rc<RefCell<bool>>,
    write: Rc<RefCell<bool>>,
}

impl<'a, T> Deref for ChangeDetectorRead<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        *self.read.borrow_mut() = true;
        &self.value
    }
}

impl<'a, T> Deref for ChangeDetectorWrite<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        *self.read.borrow_mut() = true;
        &self.value
    }
}

impl<'a, T> DerefMut for ChangeDetectorWrite<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        *self.read.borrow_mut() = true;
        *self.write.borrow_mut() = true;
        &mut self.value
    }
}

// we use `**self` instead of `self.value` to trigger the updating of the `read` flag.

impl<'a, T: DomDisplay> DomDisplay for ChangeDetectorRead<'a, T> {
    fn dom_display(&self) -> String {
        (**self).dom_display()
    }
}

impl<'a, T: DomDisplay> DomDisplay for ChangeDetectorWrite<'a, T> {
    fn dom_display(&self) -> String {
        (**self).dom_display()
    }
}

pub trait UseInEvent:
    serde::Serialize + for<'a> serde::Deserialize<'a>
{
}

impl<T> UseInEvent for T where
    T: serde::Serialize + for<'a> serde::Deserialize<'a>
{
}

pub trait EventWrapper {
    type Real;
}

pub trait Event: UseInEvent {
    const NAME: &'static str;
}

pub fn emit<E: Event>(root_name: &str, event: &E) {
    let root_element = get_by_id(root_name);
    let data = bincode::serialize(&event).unwrap();
    let data = js_sys::Uint8Array::from(data.as_slice());
    let event = web_sys::CustomEvent::new_with_event_init_dict(
        E::NAME,
        web_sys::CustomEventInit::new().detail(&data),
    )
    .unwrap();
    root_element.dispatch_event(&event).unwrap();
}
