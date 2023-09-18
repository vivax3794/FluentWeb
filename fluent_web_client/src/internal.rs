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

pub fn get_by_id(id: &str) -> web_sys::Element {
    let selector = ::std::format!("#{}", id);
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document.query_selector(&selector).unwrap().unwrap()
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

pub fn log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}

#[derive(Derivative)]
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

    pub fn was_read(&self) -> bool {
        *self.read.borrow()
    }

    pub fn was_written(&self) -> bool {
        *self.write.borrow()
    }

    pub fn clear(&self) {
        *self.read.borrow_mut() = false;
        *self.write.borrow_mut() = false;
    }

    pub fn borrow(&self) -> ChangeDetectorRead<T> {
        ChangeDetectorRead {
            value: self.value.borrow(),
            read: self.read.clone(),
        }
    }

    pub fn borrow_mut(&self) -> ChangeDetectorWrite<T> {
        ChangeDetectorWrite {
            value: self.value.borrow_mut(),
            read: self.read.clone(),
            write: self.write.clone(),
        }
    }
}

pub struct ChangeDetectorRead<'a, T> {
    value: Ref<'a, T>,
    read: Rc<RefCell<bool>>,
}

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
    #[inline(always)]
    fn dom_display(&self) -> String {
        (**self).dom_display()
    }
}

impl<'a, T: DomDisplay> DomDisplay for ChangeDetectorWrite<'a, T> {
    #[inline(always)]
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
