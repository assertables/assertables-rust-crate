//! build.rs build step for assertables
//!
//! This build step does proofreading, and doesn't do any transformation.
//!
//! To see print output:
//!
//! ```sh
//! cargo -vv build
//! ```

use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;

const SKIP_FILE_NAME_LIST: [&str; 2] = ["mod.rs", "lib.rs"];

const HYBRID_FILE_NAME_LIST: [&str; 3] = ["assert.rs", "assert_eq.rs", "assert_ne.rs"];

const DEPRECATED_FILE_NAME_LIST: [&str; 28] = [
    "assert_abs_diff_eq.rs",
    "assert_abs_diff_ge.rs",
    "assert_abs_diff_gt.rs",
    "assert_abs_diff_le.rs",
    "assert_abs_diff_lt.rs",
    "assert_abs_diff_ne.rs",
    "assert_command_stderr_contains.rs",
    "assert_command_stderr_is_match.rs",
    "assert_command_stdout_contains.rs",
    "assert_command_stdout_is_match.rs",
    "assert_fs_read_to_string_matches.rs",
    "assert_io_read_to_string_matches.rs",
    "assert_option_none.rs",
    "assert_option_some_eq.rs",
    "assert_option_some_ne.rs",
    "assert_option_some.rs",
    "assert_poll_pending.rs",
    "assert_poll_ready_eq.rs",
    "assert_poll_ready_ne.rs",
    "assert_poll_ready.rs",
    "assert_program_args_stderr_contains.rs",
    "assert_program_args_stderr_is_match.rs",
    "assert_program_args_stdout_contains.rs",
    "assert_program_args_stdout_is_match.rs",
    "assert_result_err.rs",
    "assert_result_ok_eq.rs",
    "assert_result_ok_ne.rs",
    "assert_result_ok.rs",
];

fn do_assert_file_entry(e: walkdir::DirEntry) {
    let path = Path::new(e.path());
    let stem = e
        .path()
        .file_stem()
        .expect("file_stem")
        .to_str()
        .expect("to_str");
    let text =
        std::fs::read_to_string(path).unwrap_or_else(|path| panic!("read_to_string {}", path));
    // println!("path:{}, stem:{}, len:{}", path.display(), stem, text.len());
    vet_test_assert_x_as_result(path, stem, &text);
    vet_test_assert_x(path, stem, &text);
    vet_test_debug_assert_x(path, stem, &text);
}

fn vet_test_assert_x_as_result(path: &Path, stem: &str, text: &str) {
    let regex_string = format!(
        r"(?m)(?s)^\#\[cfg\(test\)\]\nmod (?P<mod>test_{}_as_result) ",
        stem
    );
    match Regex::new(&regex_string).unwrap().captures(text) {
        Some(_captures) => {
            // let x = String::from(captures.name("mod").unwrap().as_str());
            // println!("{}", x);
        }
        None => panic!(
            "vet_test_assert_foo_as_result path:{}, stem:{}, no match.",
            path.display(),
            stem
        ),
    }
}

fn vet_test_assert_x(path: &Path, stem: &str, text: &str) {
    let regex_string = format!(r"(?m)(?s)^\#\[cfg\(test\)\]\nmod (?P<mod>test_{}) ", stem);
    match Regex::new(&regex_string).unwrap().captures(text) {
        Some(_captures) => {
            // let x = String::from(captures.name("mod").unwrap().as_str());
            // println!("{}", x);
        }
        None => panic!(
            "vet_test_assert_x path:{}, stem:{}, no match.",
            path.display(),
            stem
        ),
    }
}

// fn fix_test_debug_assert_x(path: &Path, stem: &str, text: &str) {
//     let regex_string = format!(r"(?m)(?s)^(?P<block>\#\[cfg\(test\)\]\nmod test_{} .\n.*?\n}})", stem);
//     match Regex::new(&regex_string).unwrap().captures(text) {
//         Some(captures) => {
//             let test_assert_x = String::from(captures.name("block").unwrap().as_str());
//             let test_debug_assert_x = test_assert_x
//                 .replace(&format!("mod test_{} ", stem), &format!("mod test_debug_{} ", stem))
//                 .replace(&format!(" = {}", stem), &format!(" = debug_{}", stem))
//                 .replace("let actual = ", "let _actual = ")
//                 .replace("let expect = ", "let _expect = ")
//                 .replace("    assert_eq!(actual, expect)", "    // assert_eq!(actual, expect)");
//             println!("{}", test_debug_assert_x);
//             let output = format!("{}\n\n{}", text.trim(), test_debug_assert_x);
//             std::fs::write(path, output).expect(&format!("fix_test_debug_assert_x std::fs::write path:{}, stem:{}", path.display(), stem));
//         },
//         None => panic!("fix_test_debug_assert_x path:{}, stem:{}, no match.", path.display(), stem)
//     }
// }

fn vet_test_debug_assert_x(path: &Path, stem: &str, text: &str) {
    let regex_string = format!(
        r"(?m)(?s)^\#\[cfg\(test\)\]\nmod (?P<mod>test_debug_{}) ",
        stem
    );
    match Regex::new(&regex_string).unwrap().captures(text) {
        Some(_captures) => {
            // let x = String::from(captures.name("mod").unwrap().as_str());
            // println!("{}", x);
        }
        None => panic!(
            "vet_test_debug_assert_x path:{}, stem:{}, no match.",
            path.display(),
            stem
        ),
    }
}

fn main() {
    WalkDir::new("src")
        .into_iter()
        .map(|e| e.unwrap_or_else(|e| panic!("{:?}", e)))
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            let file_name = e.file_name().to_str().expect("file_name().to_str()");
            !SKIP_FILE_NAME_LIST.contains(&file_name)
                && !HYBRID_FILE_NAME_LIST.contains(&file_name)
                && !DEPRECATED_FILE_NAME_LIST.contains(&file_name)
        })
        .for_each(do_assert_file_entry)
}
