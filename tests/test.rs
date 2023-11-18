use assert_cmd::Command;

#[test]
fn test_my_command() {
    let test_sets = [
        ["BEGIN{print 1}", "", "1\n"],
        ["{sum+=$1};END{print sum}", "1\n2\n3\n", "6\n"],
        [
            "
                BEGIN {
                    sum = 0
                    count = 1
                    while (count <= 10) {
                        sum += count
                        count += 1
                    }
                    print sum
                }
            ",
            "",
            "55\n",
        ],
        [
            "
                function test() {
                  print 1
                  test2()
                }

                function test2() {
                  print 2
                }

                BEGIN {
                  test()
                  print 3
                }
            ",
            "",
            "1\n2\n3\n",
        ],
        [
            "
                BEGIN {
                  if (1) {
                    print(1)
                  }
                  if (0) {
                    print(2)
                  } else if (1) {
                    print(3)
                  }
                }
            ",
            "",
            "1\n3\n",
        ],
        [
            "
                function add(a, b) {
                  return a + b
                }

                BEGIN {
                  print add(1, 2)
                }
            ",
            "",
            "3\n",
        ],
        [
            "
                function counter() {
                  a += 1
                  return a
                }

                function a(x, y, z) {
                  print x, y, z
                }

                BEGIN {
                  print counter(), counter(), counter()
                  a(counter(), counter(), counter())
                }
            ",
            "",
            "1 2 3\n4 5 6\n",
        ],
        // [
        //     "",
        //     "",
        //     ""
        // ],
    ];
    for set in test_sets {
        let mut cmd = Command::cargo_bin("kawk").expect("Failed to find binary");
        cmd.arg(set[0]);
        cmd.write_stdin(set[1]);
        let assert = cmd.assert();
        assert.success().stdout(set[2]);
    }
}
