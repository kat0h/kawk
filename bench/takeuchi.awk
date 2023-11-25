function tarai(x,y,z) {
  count += 1
  if (x <= y) {
    return y
  } else {
    return tarai(tarai(x-1,y,z),tarai(y-1,z,x),tarai(z-1,x,y))
  }
}

BEGIN {
  tarai(12,6,0)
  print count
}
