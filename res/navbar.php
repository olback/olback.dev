<div class="wrapper">
  <div class="header">
    <div class="container">
      <h1 class="header-heading"><a href="https://olback.net/">
      <img src="img/olback.jpg" alt="" width="32" height="32"></a> olback.net</h1>
    </div>
  </div>


  <ul class="topnav">
    <div class="container">
      <li><a href="./" class="<?php if ($_SESSION['fn'] == index) { echo "active"; } ?>">Home</a></li>
      <li><a href="blog.php"  class="<?php if ($_SESSION['fn'] == blog) { echo "active"; } ?>">Blog</a></li>
      <li><a href="projects.php" class="<?php if ($_SESSION['fn'] == projects) { echo "active"; } ?>">Projects</a></li>
      <li><a href="contact.php" class="<?php if ($_SESSION['fn'] == contact) { echo "active"; } ?>">Contact</a></li>
      <!--<li><a href="about.php"  class="<?php if ($_SESSION['fn'] == about) { echo "active"; } ?>">About</a></li>-->
      <li><a href="youtube.php"  class="<?php if ($_SESSION['fn'] == youtube) { echo "active"; } ?>">YouTube</a></li>
      <center>
        <li class="dropdown right">
          <a href="javascript:void(0)" class="dropbtn"><i class="fa fa-bars" aria-hidden="true"></i></a>
            <div class="dropdown-content black">
              <a class="active" href="https://olback.net">olback.net</a>
              <a href="https://olback.ninja">olback.ninja</a>
            </div>
        </li>
      </center>
    </div>
  </ul>
