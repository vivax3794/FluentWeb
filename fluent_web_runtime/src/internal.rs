// This code is called by generated code and should always be sound.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::cell::{Cell, RefCell};
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

pub use derivative::Derivative;
use wasm_bindgen::JsCast;
pub use {base64, bincode, js_sys, serde, wasm_bindgen, web_sys};

pub type Wrapped<C> = Rc<RefCell<C>>;
pub type WeakRef<C> = Weak<RefCell<C>>;

pub trait Component {
    fn render_init(&self) -> String;
    fn create(root_id: Box<str>) -> Self;
    fn root(&self) -> &str;
    fn set_weak(&mut self, weak: WeakRef<Self>);

    fn setup_onetime(&mut self, root: Option<&str>);
    fn update_all(&mut self, root: Option<&str>);
    fn update_props(&mut self);
}

fn setup_watcher<C: Component + 'static>(component: WeakRef<C>, root_name: &str) {
    let function = move || {
        component
            .upgrade()
            .expect("Component despawned")
            .borrow_mut()
            .update_props();
    };
    let function = wasm_bindgen::closure::Closure::<dyn Fn()>::new(function);
    let js_function = function.as_ref().unchecked_ref();
    let observer = web_sys::MutationObserver::new(js_function).unwrap();
    // NOTE We are not planning to store this as props will be refactored to not use the JS boundry
    function.forget();

    let element = get_by_id(root_name);

    let mut options = web_sys::MutationObserverInit::new();
    options.attributes(true);
    observer.observe_with_options(&element, &options).unwrap();
}

#[allow(clippy::needless_pass_by_value)]
pub fn do_if<C: Component>(
    node_id: &str,
    true_case: &str,
    condition: bool,
    parent_id: Option<&str>,
    comp: &mut C,
) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let elements = get_elements(comp.root(), &format!(".{node_id}"), parent_id);
    for element in elements {
        let current_state = element
            .get_attribute("__Fluent_If")
            .unwrap_or_else(|| "false".to_owned());

        let new_element = match (condition, &*current_state) {
            (true, "false") => {
                let new_element = document.create_element("div").unwrap();
                new_element.set_inner_html(true_case);
                let new_element = new_element.first_child().unwrap();
                let new_element = new_element.dyn_into::<web_sys::Element>().unwrap();
                new_element.set_attribute("__Fluent_If", "true").unwrap();
                new_element
            }
            (false, "true") => {
                let new_element = document.create_element("div").unwrap();
                new_element.set_attribute("__Fluent_If", "false").unwrap();
                new_element.class_list().add_1(node_id).unwrap();
                new_element
            }
            _ => element.clone(),
        };

        element.replace_with_with_node_1(&new_element).unwrap();
        if condition && current_state == "false" {
            comp.update_all(Some(node_id));
            comp.setup_onetime(Some(node_id));
        }
    }
}

#[must_use = "This is the only strong reference to this component, if this is dropped then nothing will work. consider using `fluent_web_runtime::forget` to leak its memory and keep it alive until the end of the program."]
pub fn render_component<C: Component + 'static>(mount_point: impl Into<Box<str>>) -> Wrapped<C> {
    // WORKAROUND: bug in rust analyzer makes it see this type wrong
    // WORKAROUND: https://github.com/rust-lang/rust-analyzer/issues/5514
    let mount_point: Box<str> = mount_point.into();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.get_element_by_id(&mount_point).unwrap();

    element.class_list().add_1("__Fluent_Component").unwrap();
    element
        .class_list()
        .remove_1("__Fluent_Needs_Init")
        .unwrap();

    let component = C::create(mount_point);
    element.set_inner_html(&component.render_init());

    let component = Rc::new(RefCell::new(component));
    component.borrow_mut().set_weak(Rc::downgrade(&component));

    component.borrow_mut().update_all(None);
    setup_watcher(Rc::downgrade(&component), component.borrow().root());
    component.borrow_mut().setup_onetime(None);

    component
}

#[must_use]
pub fn uuid() -> Box<str> {
    let id = uuid::Uuid::new_v4().to_string();
    format!("__Fluent_UUID_{id}").into_boxed_str()
}

#[must_use]
pub fn get_by_id(id: &str) -> web_sys::Element {
    let selector = ::std::format!("#{id}");
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document.query_selector(&selector).unwrap().unwrap()
}

#[must_use]
pub fn get_element(component_id: &str, element_id: &str) -> web_sys::Element {
    let selector =
        ::std::format!("#{component_id} #{element_id}:not(#{component_id} .__Fluent_Component *)");
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document.query_selector(&selector).unwrap().unwrap()
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn get_elements(
    component_id: &str,
    selector: &str,
    root_selector: Option<&str>,
) -> Vec<web_sys::Element> {
    let selector = ::std::format!(
        "#{} {} {}:not(#{} .__Fluent_Component *)",
        component_id,
        root_selector.map(|s| format!(".{s}")).unwrap_or_default(),
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

#[derive(Debug)]
pub struct ChangeDetector<T> {
    value: T,
    read: Cell<bool>,
    write: bool,
}

impl<T> ChangeDetector<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            read: Cell::new(false),
            write: false,
        }
    }

    #[must_use]
    pub fn was_read(&self) -> bool {
        self.read.get()
    }

    #[must_use]
    pub const fn was_written(&self) -> bool {
        self.write
    }

    pub fn clear(&mut self) {
        self.read.set(false);
        self.write = false;
    }

    #[must_use]
    pub const fn borrow(&self) -> ChangeDetectorRead<T> {
        ChangeDetectorRead {
            value: &self.value,
            read: &self.read,
        }
    }

    #[must_use]
    pub fn borrow_mut(&mut self) -> ChangeDetectorWrite<T> {
        ChangeDetectorWrite {
            value: &mut self.value,
            read: &self.read,
            write: &mut self.write,
        }
    }
}

#[derive(Debug)]
pub struct ChangeDetectorRead<'a, T> {
    value: &'a T,
    read: &'a Cell<bool>,
}

#[derive(Debug)]
pub struct ChangeDetectorWrite<'a, T> {
    value: &'a mut T,
    read: &'a Cell<bool>,
    write: &'a mut bool,
}

impl<'a, T> Deref for ChangeDetectorRead<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.read.set(true);
        self.value
    }
}

impl<'a, T> Deref for ChangeDetectorWrite<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.read.set(true);
        self.value
    }
}

impl<'a, T> DerefMut for ChangeDetectorWrite<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.read.set(true);
        *self.write = true;
        self.value
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

pub trait UseInEvent: serde::Serialize + for<'a> serde::Deserialize<'a> {}

impl<T> UseInEvent for T where T: serde::Serialize + for<'a> serde::Deserialize<'a> {}

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
