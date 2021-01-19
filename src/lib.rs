pub mod cmd;

#[cfg(test)]
mod word_test {
    use super::*;

    #[test]
    fn make_camel_works() {
        let examples = vec![
            "new_method",
            "new-method",
            "new method",
            "NEW METHOD",
            "NewMethod",
        ];

        for before in examples.iter() {
            assert_eq!(cmd::Words::new(before).into_case("camel"), "newMethod")
        }
    }

    #[test]
    fn capitalize_works() {
        let examples = vec![
            "new_method",
            "new-method",
            "new method",
            "NEW METHOD",
            "newMethod",
        ];

        for before in examples.iter() {
            assert_eq!(cmd::Words::new(before).into_case("pascal"), "NewMethod")
        }
    }

    #[test]
    fn make_lowercase_works() {
        let examples = vec![
            "new_method",
            "new-method",
            "new method",
            "NEW METHOD",
            "NewMethod",
        ];

        for before in examples.iter() {
            assert_eq!(cmd::Words::new(before).into_case("snake"), "new_method")
        }
    }

    #[test]
    fn make_uppercase_works() {
        let examples = vec![
            "new_method",
            "new-method",
            "new method",
            "NEW METHOD",
            "NewMethod",
        ];

        for before in examples.iter() {
            assert_eq!(
                cmd::Words::new(before).into_case("uppersnake"),
                "NEW_METHOD"
            )
        }
    }
}
