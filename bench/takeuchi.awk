# 竹内関数

function tarai(x,y,z) {
  count += 1
  if (x <= y) {
    return y
  } else {
    return tarai(tarai(x-1, y, z), tarai(y-1, z, x), tarai(z-1, x, y)) # 非常に多い回数の呼び出し
  }
}

BEGIN {
  # 呼び出し(たくさん時間がかかる)
  # 竹内関数
  tarai(12, 6, 0)
  print count
}
