# https://azisava.sakura.ne.jp/mandelbrot/algorithm.html

function min(a, b) {
  if (a < b) {
    return a
  } else {
    return b
  }
}

BEGIN {
  print "P3"

  size = 4
  pixel = 1000
  print pixel, pixel
  print 255
  for (i = 0; pixel > i; i++) {
      x = i * size / pixel - size / 2
      for (j = 0; pixel > j; j++) {
          y = j * size / pixel - size / 2
          a = 0
          b = 0
          d = 0
          for (k = 0; 50 > k; k++) {
              _a = a * a - b * b + x
              _b = 2 * a * b + y
              a = _a
              b = _b
              if (a * a + b * b > 4) {
                  d = 1
                  break
              }
          }
          if (d) {
              printf min(k*12, 255) " "
              printf "0 0 "
          } else {
              printf "255 255 255 "
          }
      }
      print
  } 

}
