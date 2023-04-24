use strict;
use warnings;

for my $number (0..100) {
  my $ouput = "";

  if ($number % 3 == 0) {
    $ouput .= "Fizz";
  }
  if ($number % 5 == 0) {
    $ouput .= "Buzz";
  }
  if (length($ouput) == 0) {
    $ouput = $number;
  }

  print $ouput . "\n";
}
