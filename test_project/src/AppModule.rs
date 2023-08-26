use super::Sub;
struct __Fluid_Data {}
struct __Fluid_Sub_Components {}
pub struct Component {
    root_name: ::std::string::String,
    sub_components: __Fluid_Sub_Components,
    data: __Fluid_Data,
}
impl Component {}
impl ::fluent_web_client::internal::Component for Component {
    fn render_init(&self) -> ::std::string::String {
        "<h1>Main Component</h1>\n    <sub>\n</sub>".into()
    }
    fn create(root_id: String) -> Self {
        Self {
            root_name: root_id,
            sub_components: __Fluid_Sub_Components {},
            data: __Fluid_Data {},
        }
    }
    fn update_all(&self) {}
}
