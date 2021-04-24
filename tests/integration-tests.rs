mod integration_tests {
    use ::csafe::*;

    #[test]
    fn can_find_unsafe_words() {
        let word_list = [
            "apple".to_string(),
            "bear".to_string(),
            "latenight".to_string(),
            "late".to_string(),
            "night".to_string(),
            "good".to_string(),
            "goodnight".to_string(),
            "sea".to_string(),
            "seas".to_string(),
            "sand".to_string(),
            "and".to_string(),
            "zoo".to_string(),
            "zoology".to_string(),
        ];
        assert_eq!(
            find_unsafe_words(&word_list),
            vec![
                vec!["late", "night", "latenight"],
                vec!["good", "night", "goodnight"],
                vec!["sea", "sand", "seas", "and"],
                vec!["seas", "and", "sea", "sand"],
            ]
        );
    }
    #[test]
    fn can_find_fewest_unsafe_words_to_remove() {
        let contenders_for_removal = vec![
            vec![
                "late".to_string(),
                "night".to_string(),
                "latenight".to_string(),
            ],
            vec![
                "good".to_string(),
                "night".to_string(),
                "goodnight".to_string(),
            ],
            vec![
                "sea".to_string(),
                "sand".to_string(),
                "seas".to_string(),
                "and".to_string(),
            ],
            vec![
                "seas".to_string(),
                "and".to_string(),
                "sea".to_string(),
                "sand".to_string(),
            ],
        ];
        assert_eq!(
            find_fewest_words_to_remove(contenders_for_removal),
            vec!["night", "sea"]
        );
    }

    #[test]
    fn can_remove_words_from_a_wordlist() {
        let list = vec!["bill".to_string(), "harry".to_string(), "ross".to_string()];
        let words_to_remove = vec!["harry".to_string()];
        assert_eq!(
            make_clean_list(words_to_remove, list),
            vec!["bill".to_string(), "ross".to_string()]
        );
    }
}
