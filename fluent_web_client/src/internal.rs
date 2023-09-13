use std::{
    cell::{Cell, Ref, RefMut},
    fmt::{Debug, Display},
};

pub use wasm_bindgen;
use wasm_bindgen::JsCast;
pub use web_sys;

pub trait Component {
    fn render_init(&self) -> String;
    fn create(root_id: String) -> Self;

    fn setup_events(&self);
    fn spawn_sub(&self);
    fn update_all(&self);
}

pub fn uuid() -> String {
    let id = uuid::Uuid::new_v4().to_string();
    format!("__Fluent_UUID_{id}")
}

pub fn get_element(
    component_id: &str,
    element_id: &str,
) -> web_sys::Element {
    let selector = ::std::format!(
        "#{} #{}:not(#{} .__Fluent_Component *)",
        component_id,
        element_id,
        component_id
    );
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document.query_selector(&selector).unwrap().unwrap()
}

pub fn get_elements(
    component_id: &str,
    selector: &str,
) -> Vec<web_sys::Element> {
    let selector = ::std::format!(
        "#{} {}:not(#{} .__Fluent_Component *)",
        component_id,
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
    #[inline(always)]
    fn dom_display(&self) -> String {
        format!("{}", self)
    }
}

#[inline(always)]
pub fn display<T: DomDisplay>(value: &T) -> String {
    value.dom_display()
}

pub struct ReadDetector<'a, T> {
    value: Ref<'a, T>,
    read: Cell<bool>,
}

impl<'a, T> ReadDetector<'a, T> {
    pub fn new(value: Ref<'a, T>) -> Self {
        Self {
            value,
            read: Cell::new(false),
        }
    }

    pub fn is_read(&self) -> bool {
        self.read.get()
    }

    pub fn clear(&self) {
        self.read.set(false);
    }
}

impl<'a, T> std::ops::Deref for ReadDetector<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.read.set(true);
        &self.value
    }
}

impl<'a, T: DomDisplay> DomDisplay for ReadDetector<'a, T> {
    fn dom_display(&self) -> String {
        self.value.dom_display()
    }
}

pub struct WriteDetector<'a, T> {
    value: RefMut<'a, T>,
    write: bool,
}

impl<'a, T> WriteDetector<'a, T> {
    pub fn new(value: RefMut<'a, T>) -> Self {
        Self {
            value,
            write: false,
        }
    }

    pub fn is_write(&self) -> bool {
        self.write
    }

    pub fn clear(&mut self) {
        self.write = false;
    }
}

impl<'a, T> std::ops::Deref for WriteDetector<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T> std::ops::DerefMut for WriteDetector<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.write = true;
        &mut self.value
    }
}

pub fn log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}
