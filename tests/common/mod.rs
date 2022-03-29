pub fn common_config() -> (String, String, String) {
    let project = String::from("minigrep");
    let query = String::from("to");
    let filename = String::from("poem.txt");

    (project, query, filename)
}
