function counter(m) {
  print m
  a += 1
  return a
}

function a(x, y, z) {
  print x, y, z
}

BEGIN {
  print counter("A"), counter("B"), counter("C")
  a(counter("D"), counter("E"), counter("F"))
}
