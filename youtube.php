<!DOCTYPE html>
<?php $_SESSION['fn'] = basename(__FILE__, '.php'); ?>
<?php include __DIR__ . '/res/head.php'; ?>
<body>
  <?php include __DIR__ . '/res/navbar.php'; ?>
  <div class="content">
    <div class="container">
      <div class="main">
        <h1><a href="https://www.youtube.com/channel/UCaLwTk-JfriVkeqqLYNQYKg/"><i class="fa fa-youtube-play" style="color: red;"></i></a> YouTube</h1>
        <hr>
        <p>
          My latest <a href="https://www.youtube.com/channel/UCaLwTk-JfriVkeqqLYNQYKg/" target="_blank">YouTube</a> videos: <br>
        <center>
          <iframe width="784" height="441" src="https://www.youtube.com/embed/N11NEag5Ocg" frameborder="0" allowfullscreen></iframe><br><br>
          <iframe width="784" height="441" src="https://www.youtube.com/embed/kwOGENp1pQ0" frameborder="0" allowfullscreen></iframe><br><br>
          <iframe width="784" height="441" src="https://www.youtube.com/embed/t5AAPcqC-J4" frameborder="0" allowfullscreen></iframe><br><br><br><br>
        </center>
        </p>
      </div>
    </div>
  </div>
  <?php include __DIR__ . '/res/footer.php'; ?>
</body>
</html>
