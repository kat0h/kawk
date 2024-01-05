function hanoi(n, a, b, c) {
  if (n > 0) {
    hanoi(n-1, a, c, b)
    print a, "->", b
    hanoi(n-1, c, b, a)
  }
}

BEGIN {
  hanoi(9, "A", "B", "C")
}
