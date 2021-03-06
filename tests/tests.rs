use std::env;

use minigrep::Config;

mod common;

#[test]
fn get_case_sensitive_by_true() {
    let (project, query, filename) = common::common_config();
    let case_sensitive = String::from("true");
    let args = vec![project, query, filename, case_sensitive];

    assert_eq!(Ok(true), Config::get_case_sensitive(&args));
}

#[test]
fn get_case_sensitive_by_false() {
    let (project, query, filename) = common::common_config();
    let case_sensitive = String::from("false");
    let args = vec![project, query, filename, case_sensitive];

    assert_eq!(Ok(false), Config::get_case_sensitive(&args));
}

#[test]
fn get_case_sensitive_by_other() {
    let (project, query, filename) = common::common_config();
    let case_sensitive = String::from("other");
    let args = vec![project, query, filename, case_sensitive];

    assert_eq!(
        Err("the third argument must be true or false"),
        Config::get_case_sensitive(&args)
    );
}

#[test]
fn get_case_sensitive_by_env() {
    env::set_var("CASE_INSENSITIVE", "whatever");

    let (project, query, filename) = common::common_config();
    let args = vec![project, query, filename];

    assert_eq!(Ok(false), Config::get_case_sensitive(&args));
}

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
        vec!["safe, fast, productive."],
        minigrep::search(query, contents)
    );
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        minigrep::search_case_insensitive(query, contents)
    );
}
