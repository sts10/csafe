mod integration_tests {
    use ::csafe::*;
    use std::collections::HashSet;

    #[test]
    fn can_find_unsafe_words() {
        let word_list = [
            "apple",
            "bear",
            "latenight",
            "late",
            "night",
            "good",
            "goodnight",
            "sea",
            "seas",
            "sand",
            "and",
            "zoo",
            "zoology",
        ]
        .iter()
        .map(|&s| s.to_owned())
        .collect();
        let mut unsafe_words = find_unsafe_words(&word_list);
        unsafe_words.sort_by(|a, b| a.first().unwrap().cmp(b.first().unwrap()));
        assert_eq!(
            unsafe_words,
            vec![
                vec!["good", "night", "goodnight"],
                vec!["late", "night", "latenight"],
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
            ["night", "sea"]
                .iter()
                .map(|&s| s.to_owned())
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn can_remove_words_from_a_wordlist() {
        let list = ["bill", "harry", "ross"]
            .iter()
            .map(|&s| s.to_owned())
            .collect::<HashSet<_>>();
        let words_to_remove = ["harry"]
            .iter()
            .map(|&s| s.to_owned())
            .collect::<HashSet<_>>();
        assert_eq!(
            make_clean_list(words_to_remove, list),
            vec!["bill".to_string(), "ross".to_string()]
        );
    }
}
