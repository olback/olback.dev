<?php
$_SESSION['title'] = ucfirst($_SESSION['fn']);

if ($_SESSION['title'] == Index) {
  $_SESSION['title'] = Home;
}
?>

<!DOCTYPE html>
<html lang="en"><head>
    <meta http-equiv="content-type" content="text/html; charset=UTF-8">
    <title><?php echo $_SESSION['title']; ?> - olback.net</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="shortcut icon" href="img/olback.jpg">

    <!-- CSS -->
    <link rel="stylesheet" href="css/styles_v2.css">
    <link rel="stylesheet" href="css/fa/css/font-awesome.css">
    <link rel="stylesheet" href="css/form.css">

    <!-- JS -->
    <script type="text/javascript" src="js/google.js"></script>
    <script type="text/javascript" src="js/jsLog.js"></script>
  </head>
