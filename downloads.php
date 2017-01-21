<!DOCTYPE html>
<?php include __DIR__ . '/res/head.php'; ?>
<?php $_SESSION['fn'] = basename(__FILE__, '.php'); ?>
<body>
  <?php include __DIR__ . '/res/navbar.php'; ?>
  <div class="content">
    <div class="container">
      <div class="main">
        <h1>Downloads</h1>
        <hr>
        <h3>CSGO Configs</h3>
        <img src="./img/olback.jpg" alt="" width="16" height="16"> olback's cfg: <a href="cfg" target="_blank">Download</a> <br>
        <img src="./img/mrdogxd.ico" alt="" width="13" height="16"> MrDogXd's cfg: <a href="joncfg" target="_blank">Download</a> <br>
        <hr>
        <h3>Sounds</h3>
        <img src="./img/russia.ico" alt="" width="16" height="16"> Russian national anthem: <a href="https://olback.net/download/russia.mp3" download="russia.mp3">Download</a> <a href="/download/russia.mp3">Listen</a><br>
        <img src="./img/sonic.ico" alt="" width="16" height="16"> Sonic: <a href="/download/sonic.mp3" download="sonic.mp3">Download</a> <a href="/download/sonic.mp3">Listen</a><br>
        <img src="./img/dank.ico" alt="" width="16" height="16"> Dankstorm: <a href="/download/dankstorm.mp3" download="dankstorm.mp3">Download</a> <a href="/download/dankstorm.mp3">Listen</a><br>
        <img src="./img/dank.ico" alt="" width="16" height="16"> WeedTrain: <a href="/download/weedTrain.mp3" download="weedTrain.mp3">Download</a> <a href="/download/weedTrain.mp3">Listen</a><br>
        <hr>
      </div>
    </div>
  </div>
  <?php include __DIR__ . '/res/footer.php'; ?>
</body>
</html>
