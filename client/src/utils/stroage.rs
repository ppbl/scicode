pub fn get_token() -> Option<String> {
    gloo::utils::window()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item("token")
        .unwrap()
}
pub fn sign_out() {
    let ss = gloo::utils::window()
        .local_storage()
        .unwrap()
        .unwrap()
        .remove_item("token")
        .unwrap();
}
