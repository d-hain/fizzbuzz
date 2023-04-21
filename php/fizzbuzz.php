<?php

echo "<div class='center'>";

foreach (range(0, 100) as $number) {
    $output = "";
    
    if($number % 3 == 0) {
        $output .= "Fizz";
    }
    if($number % 5 == 0) {
        $output .= "Buzz";
    }
    if (strlen($output) == 0) {
        $output = $number;
    }
    
    echo $output . "<br />";
}

echo "</div>";

?>

<style>
    .center {
        display: flex;
        justify-content: center;
    }
</style>
