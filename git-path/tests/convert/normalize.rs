use std::{borrow::Cow, path::Path};

use git_path::normalize;

fn p(input: &str) -> &Path {
    Path::new(input)
}

#[test]
fn no_change_if_there_are_no_trailing_relative_components() {
    for input in ["./a/b/c/d", "/absolute/path", "C:\\hello\\world"] {
        let path = p(input);
        assert_eq!(normalize(path, std::env::current_dir().unwrap()).unwrap(), path);
    }
}

#[test]
fn special_cases_around_cwd() -> crate::Result {
    let cwd = std::env::current_dir()?;
    assert_eq!(
        normalize(p("./../../.git/modules/src/llvm-project"), &cwd).unwrap(),
        cwd.parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(".git/modules/src/llvm-project"),
        "'.' is handled specifically to not fail to swap in the CWD"
    );
    assert_eq!(
        normalize(&cwd, &cwd).unwrap(),
        cwd,
        "absolute inputs yield absolute outputs"
    );
    assert_eq!(
        normalize(p("a/../.."), &cwd).unwrap(),
        cwd.parent().expect("parent"),
        "it automatically extends the pop-able items by using the current working dir"
    );
    assert_eq!(
        normalize(p("a/.."), &cwd).unwrap(),
        p("."),
        "absolute CWDs are always shortened…"
    );
    assert_eq!(normalize(p("./a/.."), &cwd).unwrap(), p("."), "…like this as well…");
    assert_eq!(
        normalize(&cwd, &cwd).unwrap(),
        cwd,
        "…but only if there were relative to begin with."
    );
    assert_eq!(
        normalize(p("."), &cwd).unwrap(),
        p("."),
        "and this means that `.`. stays `.`"
    );
    {
        let mut path = cwd.clone();
        let last_component = path.file_name().expect("directory").to_owned();
        path.push("..");
        path.push(last_component);

        assert_eq!(
            normalize(path, &cwd).unwrap(),
            cwd,
            "absolute input paths stay absolute"
        );
    }
    Ok(())
}

#[test]
fn parent_dirs_cause_the_cwd_to_be_used() {
    assert_eq!(
        normalize(p("./a/b/../../.."), "/users/name").unwrap().as_ref(),
        p("/users")
    );
}

#[test]
fn multiple_parent_dir_movements_eat_into_the_current_dir() {
    assert_eq!(
        normalize(p("../../../d/e"), "/users/name/a/b/c").unwrap().as_ref(),
        p("/users/name/d/e")
    );
    assert_eq!(
        normalize(p("c/../../../d/e"), "/users/name/a/b").unwrap().as_ref(),
        p("/users/name/d/e")
    );
}

#[test]
fn walking_up_too_much_yield_none() {
    let cwd = "/users/name";
    assert_eq!(normalize(p("./a/b/../../../../../."), cwd), None);
    assert_eq!(normalize(p("./a/../../../.."), cwd), None);
}

#[test]
fn trailing_directories_after_too_numereous_parent_dirs_yield_none() {
    assert_eq!(
        normalize(p("./a/b/../../../../../actually-invalid"), "/users").as_ref(),
        None,
    );
    assert_eq!(normalize(p("/a/b/../../.."), "/does-not/matter").as_ref(), None,);
}

#[test]
fn trailing_relative_components_are_resolved() {
    let cwd = Path::new("/a/b/c");
    for (input, expected) in [
        ("./a/b/./c/../d/..", "./a/b"),
        ("a/./b/c/.././..", "a"),
        ("/a/b/c/.././../.", "/a"),
        ("./a/..", "."),
        ("a/..", "."),
        ("./a", "./a"),
        ("./a/./b", "./a/./b"),
        ("./a/./b/..", "./a/."),
        ("/a/./b/c/.././../.", "/a"),
        ("/a/./b", "/a/./b"),
        ("/a/././c/.././../.", "/"),
        ("/a/b/../c/../..", "/"),
        ("C:/hello/../a", "C:/a"),
        ("./a/../b/..", "./"),
        ("/a/../b", "/b"),
    ] {
        let path = p(input);
        assert_eq!(
            normalize(path, cwd).unwrap_or_else(|| panic!("{path:?}")),
            Cow::Borrowed(p(expected)),
            "'{}' got an unexpected result",
            input
        );
    }
}
