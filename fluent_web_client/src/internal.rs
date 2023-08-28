use std::fmt::{Debug, Display};

pub use wasm_bindgen;
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

pub fn get_element(component_id: &str, element_id: &str) -> web_sys::Element {
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

pub trait FormatDisplay {
    fn format_display(&self) -> String;
}

impl<T> FormatDisplay for T
where
    T: Debug + Display,
{
    fn format_display(&self) -> String {
        format!("{}", self)
    }
}

impl<T> FormatDisplay for T
where
    T: Debug,
{
    default fn format_display(&self) -> String {
        format!("{:?}", self)
    }
}

pub fn display<T: FormatDisplay>(value: T) -> String {
    value.format_display()
}
