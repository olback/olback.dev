<!DOCTYPE html>
<html lang="en"><head>
<meta http-equiv="content-type" content="text/html; charset=UTF-8">
		<meta charset="utf-8">
		<meta http-equiv="X-UA-Compatible" content="IE=edge">
		<title>/dev/urandom</title>
		<meta name="viewport" content="width=device-width, initial-scale=1">
		<meta name="Description" content="olback.net" lang="en">
		<meta name="author" content="Edwin Svensson">

		<!-- icons -->
		<link rel="shortcut icon" href="/img/olback.jpg">

		<!-- Override CSS file - add your own CSS rules -->
		<link rel="stylesheet" href="/css/styles.css">
		<link rel="stylesheet" href="bash.css">

		<script type="text/javascript" src="/js/google.js"></script>
	</head>
	<body>
		<div class="header">
			<div class="container">
				<h1 class="header-heading"><a href="https://olback.net/dev/urandom/" target="">
                <img src="/img/olback.jpg" alt="" width="32" height="32"></a> olback.net</h1>
			</div>
		</div>
		<div class="nav-bar">
			<div class="container">
				<ul>
				    <li><a href="index">Home</a></li>
					<li><a href="mwb">Malwarebytes</a></li>
					<li><a href="gtas">GTA Scripts</a></li>
					<li><a href="jfinger">Jabber</a></li>
					<li><a class="active" href="bash">Bash</a></li>
					<li style="float:right"class="dropdown">
						<a href="#" class="dropbtn">☰</a>
							<div class="dropdown-content">
								<a href="https://olback.net">olback.net</a>
								<a href="https://olback.ninja">olback.ninja</a>
								<a class="active" href="https://olback.net/dev/urandom">/dev/urandom</a>
							</div>
					</li>
				</ul>
			</div>
		</div>
		<div class="content">
			<div class="container">
				<div class="main">
					<h1>Bash tips and tricks</h1>
					<hr>

					<p>
					Some usefull bash aliases.<br><br>
					How to install:<br>
					<div id="box">
					wget https://olback.net/dev/urandom/new.sh <br>
					chmod +x new.sh<br>
					sudo sh new.sh<br>
					</div>
					<br>
					bash_aliases content:<br><br>
					<div id="box">
					<?php
					$myfile = fopen("bash_aliases", "r") or die("Unable to open file!");
					// Output one line until end-of-file
					while(!feof($myfile)) {
					  echo fgets($myfile) . "<br>";
					}
					fclose($myfile);
					?>

					</div>
					<br>
					<a href="new.sh">This</a> simple script updates ~/.bash_aliases. (chmod +x new.sh)<br>
					This won't be usefull on servers but really cool for desktops.<br>
					Create a link in the root directory and call it ".bash_aliases".<br>
					Point the link to ".bash_aliases" in your home directory.<br>
					So when you add an alias it will work as both your own user aswell as the root user.<br>
					<br><br>
					This is the code for the install script.<br><br>
					<div id="box">
					<?php
                                        $myfile = fopen("new.sh", "r") or die("Unable to open file!");
                                        // Output one line until end-of-file
                                        while(!feof($myfile)) {
                                          echo fgets($myfile) . "<br>";
                                        }
                                        fclose($myfile);
                                        ?>
					</div>
					<br><hr>
					</p>
					<br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br> <br>
				</div>
			</div>
		</div>
		<div class="footer">
			<div class="container">
				All rights reserved &copy <script type="text/javascript">var cur = 2015; var year = new Date(); if(cur == year.getFullYear()) year = year.getFullYear(); else year = cur + ' - ' + year.getFullYear(); document.write(year);</script>
			</div>
		</div>

</body>

<!-- <audio autoplay>
<source src="/download/russia.mp3" type="audio/mpeg">
</audio> -->
</html>
