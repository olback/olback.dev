<div class="wrapper">
<div class="header">
      <div class="container">
        <h1 class="header-heading"><a href="https://olback.net/">
            <img src="img/olback.jpg" alt="" width="32" height="32"></a> olback.net</h1>
      </div>
    </div>
    <div class="nav-bar">
      <div class="container">
        <ul>
          <li class="<?php if ($_SESSION['fn'] == index) { echo "active"; } ?>"><a href="./">Home</a></li>
          <li class="<?php if ($_SESSION['fn'] == blog) { echo "active"; } ?>"><a href="blog.php">Blog</a></li>
          <li class="<?php if ($_SESSION['fn'] == projects) { echo "active"; } ?>"><a href="projects.php">Projects</a></li>
          <li class="<?php if ($_SESSION['fn'] == contact) { echo "active"; } ?>"><a href="contact.php">Contact</a></li>
          <li class="<?php if ($_SESSION['fn'] == about) { echo "active"; } ?>"><a href="about.php">About</a></li>
          <li class="<?php if ($_SESSION['fn'] == downloads) { echo "active"; } ?>"><a href="downloads.php">Downloads</a></li>
          <li class="<?php if ($_SESSION['fn'] == youtube) { echo "active"; } ?>"><a href="youtube.php">YouTube</a></li>
          <li style="float:right" class="dropdown">
            <a href="#" class="dropbtn">☰</a>
            <div class="dropdown-content">
              <a class="active" href="https://olback.net">olback.net</a>
              <a href="https://olback.ninja">olback.ninja</a>
            </div>
          </li>
        </ul>
      </div>
    </div>
