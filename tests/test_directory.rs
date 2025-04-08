use assert_cmd::Command;
use assertables::assert_contains;
use std::fs;
use std::path::PathBuf;

#[test]
fn default_sort_directory() -> Result<(), String> {
    let tempdir = tempfile::tempdir().unwrap();
    let temp_path = PathBuf::from(&tempdir.path());

    // create dirs
    let test_paths = vec!["test1/", "test1/folder1/", "test2/"];
    for p in test_paths {
        let mut t = temp_path.clone();
        t.push(p);
        fs::create_dir(&t).unwrap();
    }

    for (i, p) in TEST_FILE_PATHS.iter().enumerate() {
        let mut t = temp_path.clone();
        t.push(p);
        let path_s = t.to_str().unwrap();
        fs::write(path_s, TEST_FILES[i]).unwrap();
    }


    let mut cmd = Command::cargo_bin("roast").unwrap();
    let res = cmd.arg(tempdir.path()).arg("--spaces").assert().success();

    let out = res.get_output();
    let stderr = String::from_utf8(out.stderr.clone()).unwrap();

    for (i, f) in TEST_FILE_PATHS.iter().enumerate() {
        let status_line = match f.contains("not_json") {
            true => format!("{} - ParseError", f),
            false => format!("{} - OK", f),
        };
        assert_contains!(stderr, &status_line);

        let mut t = temp_path.clone();
        t.push(f);
        let path_s = t.to_str().unwrap();
        let updated_file = fs::read_to_string(path_s).unwrap();
        assert_eq!(updated_file, SORTED_FILES[i])
    }

    tempdir.close().unwrap();
    Ok(())
}

static TEST_FILE_PATHS: &'static [&'static str] = &[
    "test1/file1.json",
    "test1/.sneakyrc",
    "test1/folder1/file3.json",
    "test2/file4.json",
    "file5.json",
    "not_json_broken.json",
    "not_json_liar.json",
    "not_json.yml"
];

static TEST_FILES: &'static [&'static str] = &[
    // test1/file1.json
    r#"{
    "b": "bbb1", 
    "a": "aaa1",
    "c": "ccc1"
  }"#,
    // test1/.sneakyrc
    r#"{
    "c": "ccc2",
    "b": "bbb2",
    "a": "aaa2"
  }"#,
    // test1/folder1/file3.json
    r#"{
    "d": "ddd3",
    "c": "ccc3",
    "b": "bbb3",
    "a": "aaa3"
  }"#,
    // test2/file4.json
    r#"{
    "a": "aaa4",
    "c": [
      {
        "z": "adasad",
        "a": "sdfgdfgd",
        "m": "dfgdfgdf"
      }
    ],
    "b": "bbb4"
  }"#,
    // file5.json
    r#"{
    "package": true
  }"#,
    // not_json_broken.json
    r#"{
    "package": true,
  }"#,
    // not_json_liar.json
    "i am a text file",
    // not_json.yml
    "a: value",
];

static SORTED_FILES: &'static [&'static str] = &[
    // test1/file1.json
    r#"{
  "a": "aaa1",
  "b": "bbb1",
  "c": "ccc1"
}
"#,
    // test1/.sneakyrc
    r#"{
  "a": "aaa2",
  "b": "bbb2",
  "c": "ccc2"
}
"#,
    // test1/folder1/file3.json
    r#"{
  "a": "aaa3",
  "b": "bbb3",
  "c": "ccc3",
  "d": "ddd3"
}
"#,
    // test2/file4.json
    r#"{
  "a": "aaa4",
  "b": "bbb4",
  "c": [
    {
      "a": "sdfgdfgd",
      "m": "dfgdfgdf",
      "z": "adasad"
    }
  ]
}
"#,
    // file5.json
    r#"{
  "package": true
}
"#,
    // not_json_broken.json
    r#"{
    "package": true,
  }"#,
    // not_json_liar.json
    "i am a text file",
    // not_json.yml
    "a: value",
];
