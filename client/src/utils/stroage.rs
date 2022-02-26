pub fn get_token() -> Option<String> {
    gloo::utils::window()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item("token")
        .unwrap()
}
pub fn get_userid() -> Option<String> {
    gloo::utils::window()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item("userid")
        .unwrap()
}
pub fn sign_out() {
    let local_stroage = gloo::utils::window().local_storage().unwrap().unwrap();
    local_stroage.remove_item("token").unwrap();
    local_stroage.remove_item("userid").unwrap();
}
