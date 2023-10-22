pub mod simple_rendering;

#[cfg(test)]
mod test_utils {
    use fluent_web_runtime::internal::Component;
    use fluent_web_runtime::{forget, render_component};

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
}
