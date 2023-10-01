function test() {
  print "test"
  test2()
}

function test2() {
  print "test2"
}

BEGIN {
  test()
  print "end"
}
