function sleep(ms) {
  system("sleep " int(ms)/1000)
}

BEGIN {
  SLEEP = 300
  printf "H"; flush(); sleep(SLEEP)
  printf "e"; flush(); sleep(SLEEP)
  printf "l"; flush(); sleep(SLEEP)
  printf "l"; flush(); sleep(SLEEP)
  printf "o"; flush(); sleep(SLEEP)
  printf " "; flush(); sleep(SLEEP)
  printf "W"; flush(); sleep(SLEEP)
  printf "o"; flush(); sleep(SLEEP)
  printf "r"; flush(); sleep(SLEEP)
  printf "l"; flush(); sleep(SLEEP)
  printf "d"; flush(); sleep(SLEEP)
  printf "!"; flush(); sleep(SLEEP)

  while(1) {
    printf "\e[1G\e[1mHello World!\e[0m"; flush()
    sleep(500)
    printf "\e[1GHello World!"; flush()
    sleep(500)
  }
}
