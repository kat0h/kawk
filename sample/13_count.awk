# awk -f sample/13_count.awk < sample/wordlist
# それぞれの果物が何個あるかを表示

{
  count[tolower($1)]++
}

END {
  for (fruit in count) {
    print fruit ": " count[fruit]
  }
}
