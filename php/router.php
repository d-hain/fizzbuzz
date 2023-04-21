<?php
// Inspired by https://stackoverflow.com/questions/1678010/php-server-on-local-machine/21872484#21872484
if (preg_match('/\.(?:png|jpg|jpeg|gif)$/', $_SERVER["REQUEST_URI"])) {
    return false;    // serve the requested resource as-is.
} else {
    require_once('fizzbuzz.php');
}

