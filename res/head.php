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
    <link href="https://maxcdn.bootstrapcdn.com/font-awesome/4.7.0/css/font-awesome.min.css" rel="stylesheet" integrity="sha384-wvfXpqpZZVQGK6TAh5PVlGOfQNHSoD2xbE+QkPxCAFlNEevoEH3Sl0sibVcOQVnN" crossorigin="anonymous">
    <link rel="stylesheet" href="css/form.css">

    <!-- JS -->
    <script type="text/javascript" src="js/jsLog.js"></script>
  </head>
