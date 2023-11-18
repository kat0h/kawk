use assert_cmd::Command;

#[test]
fn test_my_command() {
    let test_sets = [
        ["BEGIN{print 1}", "", "1\n"],
        ["{sum+=$1};END{print sum}", "1\n2\n3\n", "6\n"],
    ];
    for set in test_sets {
        let mut cmd = Command::cargo_bin("kawk").expect("Failed to find binary");
        cmd.arg(set[0]);
        cmd.write_stdin(set[1]);
        let assert = cmd.assert();
        assert.success().stdout(set[2]);
    }
}
