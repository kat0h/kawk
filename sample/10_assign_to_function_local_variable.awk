function func1(var) {
  var = "test";
  var2 = "test2"
  print var
}

BEGIN {
  func1(11)
  print var
  print var2
}
