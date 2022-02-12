pub fn get_origin() -> String {
    gloo::utils::window().location().origin().unwrap()
}
