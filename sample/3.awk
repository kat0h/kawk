BEGIN {
  print "1~10までの合計を計算する"

  sum = 0
  count = 1
  while (count <= 10) {
    sum += count
    count += 1
  }

  print "合計: " sum
}
