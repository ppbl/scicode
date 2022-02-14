pub fn get_token() -> Option<String> {
    gloo::utils::window()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item("token")
        .unwrap()
}
