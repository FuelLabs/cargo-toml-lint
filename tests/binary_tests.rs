use assert_cmd::Command;

#[test]
fn self_test() {
    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./Cargo.toml")
        .assert();

    assert.success();
}

#[test]
fn ok_test() {
    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Ok-but-weird.toml")
        .arg("--no-cargo-verify")
        .assert();

    assert.success();

    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Ok-but-weird.toml")
        .arg("--no-cargo-verify")
        .arg("-Dsection")
        .assert();

    assert.success();

    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Ok-but-weird.toml")
        .arg("--no-cargo-verify")
        .arg("-Dnone")
        .assert();

    assert.success();
}

#[test]
fn unsorted_deps() {
    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Unsorted-deps.toml")
        .arg("--no-cargo-verify")
        .arg("-Dnone")
        .arg("-Tn")
        .assert();

    assert.success();

    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Unsorted-deps.toml")
        .arg("--no-cargo-verify")
        .arg("-Dsection")
        .arg("-Tn")
        .assert();

    assert
        .failure()
        .stderr("Error: \"[dependencies] not sorted correctly: b is specified after c\"\n");

    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Unsorted-deps.toml")
        .arg("--no-cargo-verify")
        .arg("-Dstrict")
        .arg("-Tn")
        .assert();

    assert.failure().stderr(
        "Error: \"[dependencies] not sorted correctly (strict): b is specified after c\"\n",
    );
}

#[test]
fn unsorted_deps_only_strict_fails() {
    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Unsorted-deps-strict.toml")
        .arg("--no-cargo-verify")
        .arg("-Dn")
        .arg("-Ty")
        .arg("-Ay")
        .assert();

    assert.success();

    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Unsorted-deps-strict.toml")
        .arg("--no-cargo-verify")
        .arg("-Dsection")
        .arg("-Ty")
        .arg("-Ay")
        .assert();

    assert.success();

    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Unsorted-deps-strict.toml")
        .arg("--no-cargo-verify")
        .arg("-Dstrict")
        .arg("-Ty")
        .arg("-Ay")
        .assert();

    assert.failure().stderr(
        "Error: \"[dependencies] not sorted correctly (strict): c is specified after d\"\n",
    );
}

#[test]
fn unsorted_tests() {
    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Unsorted-tests.toml")
        .arg("--no-cargo-verify")
        .arg("-Dn")
        .arg("-Tn")
        .arg("-An")
        .assert();

    assert.success();

    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Unsorted-tests.toml")
        .arg("--no-cargo-verify")
        .arg("-Dstrict")
        .arg("-Ty")
        .arg("-Ay")
        .assert();

    assert
        .failure()
        .stderr("Error: \"[[test]] not sorted correctly: item at index 2 with name=b is specified after name=c\"\n");
}

#[test]
fn split_test_array() {
    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Split-test-array.toml")
        .arg("--no-cargo-verify")
        .arg("-Dn")
        .arg("-Tn")
        .arg("-An")
        .assert();

    assert.success();

    let assert = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap()
        .arg("./tests/Split-test-array.toml")
        .arg("--no-cargo-verify")
        .arg("-Dn")
        .arg("-Tn")
        .arg("-Ay")
        .assert();

    assert.failure().stderr("Error: \"Items of [[test]] are separated by other headers, for instance [dependencies.c]\"\n");
}
