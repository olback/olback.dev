<!DOCTYPE html>
<?php $_SESSION['fn'] = basename(__FILE__, '.php'); ?>
<?php include __DIR__ . '/res/head.php'; ?>
<body>
  <?php include __DIR__ . '/res/navbar.php'; ?>
  <div class="content">
    <div class="container">
      <div class="main">
        <h1>Links</h1>
        <hr>
        <p style="font-size:150%"> Your local date and time: <span id="date-time"></p>
        <hr>
        <p style="font-size:100%">
          <i class="fa fa-google" style="color: #0080ff;"></i> <a href="https://encrypted.google.com/">Google</a><br>
          <i class="fa fa-youtube-play" style="color: red;"></i> <a href="https://www.youtube.com/feed/subscriptions">Youtube</a><br>
          <i class="fa fa-steam" style="color: black;"></i> Steam <a href="https://steamcommunity.com/market">Community Market</a> <a href="https://steamcommunity.com/id/olback/">olback</a> <a href="https://steamcommunity.com/id/xXx_PussySlayer1337_xXx/">xXx_PussySlayer1337_xXx</a><br>
          <i class="fa fa-steam" style="color: black;"></i> <a href="https://steamstat.us/">Steam Status</a><br>
          <i class="fa fa-lock" style="color: #00ff00;"></i> <a href="https://lastpass.com/">Lastpass</a><br>
          <img src="./img/olback.jpg" alt="Image not found." width="16" height="16"> <a href="cheats" >olback's cheat </a><br>
          <i class="fa fa-google" style="color: #0080ff;"></i> <a href="https://translate.google.com/"> Google Translate</a><br>
        </p>
      </div>
    </div>
  </div>
  <?php include __DIR__ . '/res/footer.php'; ?>
 </body>
  <script type="text/javascript" src="js/date-time.js"></script>
</html>
