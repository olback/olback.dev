<?php include __DIR__.'/res/head.php';?>
  <body>
    <?php $_SESSION['fn'] = basename(__FILE__, '.php'); ?>
    <?php include __DIR__.'/res/navbar.php';?>
    <div class="content">
      <div class="container">
        <div class="main">
          <h1>Welcome to olback.net</h1>
          <hr>
		  <p id="budgie-desktop">
			<h2>Budgie-Desktop</h2><br>
			1. Install updates. <br>
			<code>sudo apt update && sudo apt upgrade -y && sudo apt autoremove -y</code><br><br>

			2. Add the budgie-desktop repository.<br>
			<code>sudo add-apt-repository ppa:budgie-remix/ppa</code><br><br>

			3. Update repositories.<br>
			<code>sudo apt update</code><br><br>

			4. Install desktop environment.<br>
			<code>sudo apt-get install budgie-desktop</code>


		  </p>
		  <hr>
		  <p id="arc">
			<h2>Arc theme and Icons</h2><br>
			1. Install updates. <br>
			<code>sudo apt update && sudo apt upgrade -y && sudo apt autoremove -y</code><br><br>

			2. Add Arc-Icons and Arc-Theme repositories.<br>
			Theme repo: <code>sudo add-apt-repository ppa:noobslab/themes</code><br>
			Icons repo: <code>sudo add-apt-repository ppa:noobslab/icons</code><br><br>

			3. Update repositories.<br>
			<code>sudo apt update</code><br><br>

			4. Install theme and icons.<br>
			Theme: <code>sudo apt-get install arc-theme</code><br>
			Icons: <code>sudo apt-get install arc-icons</code><br><br>
		  </p>
		  <hr>
          <p id="xrdp">
			<h2>Ubuntu 16.04 and XRDP</h2><br>
			1. Install updates. <br>
			<code>sudo apt update && sudo apt upgrade -y && sudo apt autoremove -y</code><br><br>

			2. Make sure that you don't have any VNC servers installed.<br>
			<code>sudp apt remove vnc4server tightvnc</code><br><br>

			3. Download and extract TigerVNC. You can either download it <a href="download/tigervncserver_1.6.80-4_amd64.zip">here</a> or build it yourself. Read more <a href="https://www.hiroom2.com/2016/05/24/ubuntu-16-04-remote-connect-to-gnome-classic-desktop-with-vnc-xrdp/" target="_blank">here</a>.<br><br>

			4. Install TigerVNC.<br><br>

			5. Install XRDP.<br>
			<code>sudo apt-get install xrdp</code><br><br><br>
		  </p>
        </div>
      </div>
      <?php include __DIR__.'/res/footer.php'; ?>
    </div>
  </body>
</html>
