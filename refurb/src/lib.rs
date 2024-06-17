// SPDX-License-Identifier: Apache-2.0

// Re-export the derive macro.
pub use refurb_derive::Update;

/// A type that can copy non-`None` values from other instances of itself.
pub trait Update {
    /// Update self with the value from other, if present.
    fn update(&mut self, other: &Self);
}

impl<T: Clone> Update for Option<T> {
    fn update(&mut self, other: &Option<T>) {
        if other.is_some() {
            self.clone_from(other);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
