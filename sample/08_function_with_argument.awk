function add(a, b) {
  print a, b
  return a + b
}

BEGIN {
  print add(1, 2)
}
