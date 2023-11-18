function counter() {
  a += 1
  return a
}

BEGIN {
  print "1 2 3の出力が期待されます"
  print counter(), counter(), counter()
}
