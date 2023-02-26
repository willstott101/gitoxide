mod name_partial {
    mod valid {
        use bstr::ByteSlice;
        macro_rules! mktest {
            ($name:ident, $input:expr) => {
                #[test]
                fn $name() {
                    assert!(gix_validate::reference::name_partial($input.as_bstr()).is_ok())
                }
            };
        }

        mktest!(refs_path, b"refs/heads/main");
        mktest!(main_worktree_pseudo_ref, b"main-worktree/HEAD");
        mktest!(main_worktree_ref, b"main-worktree/refs/bisect/good");
        mktest!(other_worktree_pseudo_ref, b"worktrees/id/HEAD");
        mktest!(other_worktree_ref, b"worktrees/id/refs/bisect/good");
        mktest!(worktree_private_ref, b"refs/worktree/private");
        mktest!(refs_path_with_file_extension, b"refs/heads/main.ext");
        mktest!(refs_path_underscores_and_dashes, b"refs/heads/main-2nd_ext");
        mktest!(relative_path, b"etc/foo");
        mktest!(all_uppercase, b"MAIN");
        mktest!(all_uppercase_with_underscore, b"NEW_HEAD");
        mktest!(partial_name_lowercase, b"main");
        mktest!(chinese_utf8, "heads/你好吗".as_bytes());
        mktest!(parantheses_special_case_upload_pack, b"(null)");
    }

    mod invalid {
        use bstr::ByteSlice;
        use gix_validate::{reference::name::Error as RefError, tag::name::Error as TagError};

        macro_rules! mktest {
            ($name:ident, $input:literal, $expected:pat) => {
                #[test]
                fn $name() {
                    match gix_validate::reference::name_partial($input.as_bstr()) {
                        Err($expected) => {}
                        got => panic!("Wanted {}, got {:?}", stringify!($expected), got),
                    }
                }
            };
        }

        mktest!(
            refs_path_double_dot,
            b"refs/../somewhere",
            RefError::Tag(TagError::DoubleDot)
        );
        mktest!(
            refs_path_name_starts_with_dot,
            b".refs/somewhere",
            RefError::Tag(TagError::StartsWithDot)
        );
        mktest!(
            refs_path_component_is_singular_dot,
            b"refs/./still-inside-but-not-cool",
            RefError::SingleDot
        );
        mktest!(any_path_starts_with_slash, b"/etc/foo", RefError::StartsWithSlash);
        mktest!(empty_path, b"", RefError::Tag(TagError::Empty));
        mktest!(refs_starts_with_slash, b"/refs/heads/main", RefError::StartsWithSlash);
        mktest!(
            ends_with_slash,
            b"refs/heads/main/",
            RefError::Tag(TagError::EndsWithSlash)
        );
        mktest!(
            path_with_duplicate_slashes,
            b"refs//heads/main",
            RefError::RepeatedSlash
        );
        mktest!(
            path_with_spaces,
            b"refs//heads/name with spaces",
            RefError::Tag(TagError::InvalidByte { .. })
        );
        mktest!(
            path_with_backslashes,
            b"refs\\heads/name with spaces",
            RefError::Tag(TagError::InvalidByte { .. })
        );
    }
}

mod name {
    mod valid {
        use bstr::ByteSlice;
        macro_rules! mktest {
            ($name:ident, $input:expr) => {
                #[test]
                fn $name() {
                    assert!(gix_validate::refname($input.as_bstr()).is_ok())
                }
            };
        }

        mktest!(main_worktree_pseudo_ref, b"main-worktree/HEAD");
        mktest!(main_worktree_ref, b"main-worktree/refs/bisect/good");
        mktest!(other_worktree_pseudo_ref, b"worktrees/id/HEAD");
        mktest!(other_worktree_ref, b"worktrees/id/refs/bisect/good");
        mktest!(worktree_private_ref, b"refs/worktree/private");
        mktest!(refs_path, b"refs/heads/main");
        mktest!(refs_path_with_file_extension, b"refs/heads/main.ext");
        mktest!(refs_path_underscores_and_dashes, b"refs/heads/main-2nd_ext");
        mktest!(relative_path, b"etc/foo");
        mktest!(all_uppercase, b"MAIN");
        mktest!(all_uppercase_with_underscore, b"NEW_HEAD");
        mktest!(chinese_utf8, "refs/heads/你好吗".as_bytes());
    }

    mod invalid {
        use bstr::ByteSlice;
        use gix_validate::{reference::name::Error as RefError, tag::name::Error as TagError};

        macro_rules! mktest {
            ($name:ident, $input:literal, $expected:pat) => {
                #[test]
                fn $name() {
                    match gix_validate::reference::name($input.as_bstr()) {
                        Err($expected) => {}
                        got => panic!("Wanted {}, got {:?}", stringify!($expected), got),
                    }
                }
            };
        }

        mktest!(
            refs_path_double_dot,
            b"refs/../somewhere",
            RefError::Tag(TagError::DoubleDot)
        );
        mktest!(refs_name_special_case_upload_pack, b"(null)", RefError::SomeLowercase);
        mktest!(
            refs_path_name_starts_with_dot,
            b".refs/somewhere",
            RefError::Tag(TagError::StartsWithDot)
        );
        mktest!(
            refs_path_component_is_singular_dot,
            b"refs/./still-inside-but-not-cool",
            RefError::SingleDot
        );
        mktest!(capitalized_name_without_path, b"Main", RefError::SomeLowercase);
        mktest!(lowercase_name_without_path, b"main", RefError::SomeLowercase);
        mktest!(any_path_starts_with_slash, b"/etc/foo", RefError::StartsWithSlash);
        mktest!(empty_path, b"", RefError::Tag(TagError::Empty));
        mktest!(refs_starts_with_slash, b"/refs/heads/main", RefError::StartsWithSlash);
        mktest!(
            ends_with_slash,
            b"refs/heads/main/",
            RefError::Tag(TagError::EndsWithSlash)
        );
        mktest!(
            a_path_with_duplicate_slashes,
            b"refs//heads/main",
            RefError::RepeatedSlash
        );
    }
}
