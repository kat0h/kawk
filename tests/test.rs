use assert_cmd::Command;

#[test]
fn test_my_command() {
    let test_sets = [
        // 基本の入出力
        ["BEGIN{print 1}", "", "1\n"],
        // 標準入力
        ["{sum+=$1};END{print sum}", "1\n2\n3\n", "6\n"],
        // While文
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
        // 引数のない関数呼び出し
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
        // if文
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
        // 引数のある関数
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
        // 関数の引数の呼び出し順序
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
        // 関数の再帰呼び出し
        [
            "
                function hanoi(n, a, b, c) {
                  if (n > 0) {
                    hanoi(n-1, a, c, b)
                    print a, \"->\", b
                    hanoi(n-1, c, b, a)
                  }
                }

                BEGIN {
                  hanoi(3, \"A\", \"B\", \"C\")
                }
            ",
            "",
            "A -> B\nA -> C\nB -> C\nA -> B\nC -> A\nC -> B\nA -> B\n",
        ],
        [
            "
            function func1(var) {
              var = \"test\";
              var2 = \"test2\"
              print var
            }

            BEGIN {
              func1(11)
              print var
              print var2
            }
            ",
            "",
            "test\n\ntest2\n",
        ],
        [
            "
            BEGIN {
              for (i=0; i<10; i+=1) {
                print i
              }
            }
            ",
            "",
            "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n",
        ],
        [
            "
            BEGIN {
              for (i=0; i<10; i+=1) {
                print i
              }
            }
            ",
            "",
            "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n",
        ],
        ["BEGIN{print ++i}", "", "1\n"],
        ["BEGIN{print i++}", "", "0\n"],
        [
            "
            BEGIN {
              for (i=0; i<10; i+=1) {
                if (i>5) break
                print i
              }
            }
            ",
            "",
            "0\n1\n2\n3\n4\n5\n",
        ],
        ["BEGIN{printf 123; print 123}", "", "123123\n"],
        ["BEGIN{printf 0 == i}", "", "1"]
        // [
                                                          //     "
                                                          //     BEGIN {
                                                          //       for (i=0; i<10;) {
                                                          //         print i
                                                          //         i += 1
                                                          //       }
                                                          //     }
                                                          //     ",
                                                          //     "",
                                                          //     "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n"
                                                          // ],
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
