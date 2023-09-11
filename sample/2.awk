BEGIN {
  print "入力の合計を出力します"
}

{
  sum += $1
}

END {
  print "合計: " sum
}
