pub mod attributes;
pub mod attributes_sub;
pub mod events;
pub mod events_sub;
pub mod ifs;
pub mod simple_rendering;
pub mod styling;
pub mod styling_sub;

#[cfg(test)]
mod test_utils {
    use fluent_web_runtime::forget;
    use fluent_web_runtime::internal::{render_component, Component};

    pub const MOUNT_POINT: &str = "MOUNT";

    pub fn setup_dom() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        if let Some(exsisting) = document.get_element_by_id(MOUNT_POINT) {
            exsisting.remove();
        }

        let mount_point = document.create_element("div").unwrap();
        mount_point.set_id(MOUNT_POINT);
        document.body().unwrap().append_child(&mount_point).unwrap();
    }

    pub fn setup<C: Component + 'static>() -> web_sys::Document {
        setup_dom();
        forget(render_component::<C>(crate::test_utils::MOUNT_POINT));
        web_sys::window().unwrap().document().unwrap()
    }

    pub fn html(element: web_sys::Element) -> web_sys::HtmlElement {
        use wasm_bindgen::JsCast;
        element.dyn_ref::<web_sys::HtmlElement>().unwrap().clone()
    }
}
