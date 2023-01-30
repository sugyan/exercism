let leap_year = function
  | year when year mod 400 = 0 -> true
  | year when year mod 100 = 0 -> false
  | year -> year mod 4 = 0
