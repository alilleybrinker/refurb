// SPDX-License-Identifier: Apache-2.0

use refurb::Update;

#[derive(Debug, Default, Update, PartialEq, Eq)]
struct Config {
    a: Option<usize>,
    b: Option<bool>,
    c: Option<String>,
    d: Option<u64>,
}

#[test]
fn basic_derive_test() {
    let config_1 = Config {
        a: Some(10),
        b: Some(false),
        ..Config::default()
    };

    let config_2 = Config {
        a: Some(20),
        c: Some(String::from("hello, world")),
        ..Config::default()
    };

    let expected_config = Config {
        a: Some(20),
        b: Some(false),
        c: Some(String::from("hello, world")),
        d: None,
    };

    let mut config_3 = Config::default();
    config_3.update(&config_1);
    config_3.update(&config_2);

    assert_eq!(config_3, expected_config);
}
