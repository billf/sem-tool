use assert_cmd::Command;

const TEST_PKG_NAME: &str = "sort";

#[test]
fn cli_sort_boring_cases() {
    // Passing status with no args.
    //
    // Why? Because the default behavior with no args is to read from STDIN,
    // and STDIN provides nothing, so the list is truly empty.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("sort")
        .assert()
        .append_context(TEST_PKG_NAME, "no args")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("sort").arg("--help").assert();
    assert.append_context(TEST_PKG_NAME, "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("sort").arg("a.b.c").assert();
    assert
        .append_context(TEST_PKG_NAME, "1 bad semver args")
        .failure();
}

// NOTE(canardleteer): These are not very robust at all.
#[test]
fn cli_sort_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("sort").arg("0.1.2-rc0").assert();
    assert.append_context(TEST_PKG_NAME, "1 item").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("-f >0")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert.append_context(TEST_PKG_NAME, "help").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("-r")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, -r")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("--flatten")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, --flatten")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("--lexical-sorting")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, --lexical-sorting,")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg("sort")
        .arg("--lexical-sorting")
        .arg("--flatten")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(TEST_PKG_NAME, "2 items, --lexical-sorting, --flatten")
        .success();
}
