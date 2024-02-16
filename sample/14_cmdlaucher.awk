function prompt() {
  system("clear")
  print "== Command Launcher =="
  print "1 : vim"
  print "2 : top"
  print "type 1 or 2"
}

BEGIN {
  prompt()
}

{
  if ($0 == 1) {
    system("vim")
  } else if ($0 == 2) {
    system("top")
    prompt()
  } else {
    prompt()
    print "Error!! Type Correct command"
  }
}
