pub use web_sys;

pub trait Component {
    fn render_init(&self) -> String;
    fn create(root_id: String) -> Self;

    fn update_all(&self);
}

pub fn uuid() -> String {
    let id = uuid::Uuid::new_v4().to_string();
    format!("__Fluent_UUID_{id}")
}
