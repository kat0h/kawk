BEGIN {
  sum = 0
  count = 1
  while (count <= 10000000) {
    sum += count
    count += 1
  }

  print sum
}

