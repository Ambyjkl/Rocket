#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod common;

#[test]
fn test_development_config() {
    common::test_config(rocket::config::Environment::Development);
}
