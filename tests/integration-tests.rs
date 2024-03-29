mod integration_tests {
    use ::csafe::*;
    use fxhash::FxHashSet;

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
        let mut unsafe_ambiguities = find_unsafe_ambiguities(&word_list, false);
        unsafe_ambiguities.sort_by(|a, b| a.root_word.cmp(&b.root_word));
        let ambiguities_should_find = vec![
            Ambiguity {
                root_word: "good".to_string(),
                second_word: "night".to_string(),
                head: "goodnight".to_string(),
                ..Default::default()
            },
            Ambiguity {
                root_word: "late".to_string(),
                second_word: "night".to_string(),
                head: "latenight".to_string(),
                ..Default::default()
            },
            Ambiguity {
                root_word: "sea".to_string(),
                second_word: "sand".to_string(),
                head: "seas".to_string(),
                tail: "and".to_string(),
            },
            Ambiguity {
                root_word: "seas".to_string(),
                second_word: "and".to_string(),
                head: "sea".to_string(),
                tail: "sand".to_string(),
            },
        ];
        assert_eq!(unsafe_ambiguities, ambiguities_should_find);
    }
    #[test]
    fn can_find_fewest_unsafe_words_to_remove() {
        let ambiguities_for_removal = vec![
            Ambiguity {
                root_word: "good".to_string(),
                second_word: "night".to_string(),
                head: "goodnight".to_string(),
                ..Default::default()
            },
            Ambiguity {
                root_word: "late".to_string(),
                second_word: "night".to_string(),
                head: "latenight".to_string(),
                ..Default::default()
            },
            Ambiguity {
                root_word: "sea".to_string(),
                second_word: "sand".to_string(),
                head: "seas".to_string(),
                tail: "and".to_string(),
            },
            Ambiguity {
                root_word: "seas".to_string(),
                second_word: "and".to_string(),
                head: "sea".to_string(),
                tail: "sand".to_string(),
            },
        ];
        assert_eq!(
            find_fewest_words_to_remove(ambiguities_for_removal),
            ["night", "sea"]
                .iter()
                .map(|&s| s.to_owned())
                .collect::<FxHashSet<_>>()
        );
    }

    #[test]
    fn can_remove_words_from_a_wordlist() {
        let list = ["bill", "harry", "ross"]
            .iter()
            .map(|&s| s.to_owned())
            .collect::<FxHashSet<_>>();
        let words_to_remove = ["harry"]
            .iter()
            .map(|&s| s.to_owned())
            .collect::<FxHashSet<_>>();
        assert_eq!(
            make_clean_list(words_to_remove, &list),
            vec!["bill".to_string(), "ross".to_string()]
        );
    }

    #[test]
    fn can_find_unsafe_words_with_accents() {
        let word_list = [
            "cliché", "éspirit", "spirit", "pass", "passé", "dog", "meal", "equality", "quality",
        ]
        .iter()
        .map(|&s| s.to_owned())
        .collect();
        let mut unsafe_ambiguities = find_unsafe_ambiguities(&word_list, false);
        unsafe_ambiguities.sort_by(|a, b| a.root_word.cmp(&b.root_word));
        let ambiguities_should_find = vec![
            Ambiguity {
                root_word: "pass".to_string(),
                second_word: "éspirit".to_string(),
                head: "passé".to_string(),
                tail: "spirit".to_string(),
            },
            Ambiguity {
                root_word: "passé".to_string(),
                second_word: "spirit".to_string(),
                head: "pass".to_string(),
                tail: "éspirit".to_string(),
            },
        ];
        assert_eq!(unsafe_ambiguities, ambiguities_should_find);
    }
}
