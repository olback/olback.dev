<?php
include __DIR__ . '/config.php';
if(isset($_POST['submit'])) {
    $response = $_POST['g-recaptcha-response'];
    $remoteIp = $_SERVER['REMOTE_ADDR'];


    $reCaptchaValidationUrl = file_get_contents("https://www.google.com/recaptcha/api/siteverify?secret=$secretKey&response=$response&remoteip=$remoteIp");
    $result = json_decode($reCaptchaValidationUrl, TRUE);

    if($result['success'] == 1) {
      //True - What happens when user is verified

      $name = $_POST['name'];
      $email = $_POST['email'];
      $subject = $_POST['subject'];
      $message = $_POST['message'];

      $htmlContent = "
                <h1>Contact request details</h1>
                <p><b>Name: </b>" . $name . "</p>
                <p><b>Email: </b>" . $email . "</p>
                <p><b>Subject: </b>" . $subject . "</p>
                <p><b>Message: </b>" . $message . "</p>
                    ";

      require __DIR__ . '/mail/PHPMailerAutoload.php';

       $mail = new PHPMailer;

       $mail->isSMTP();                                       // Set mailer to use SMTP
       $mail->Host = $mail_host;                              // Var is set in config.php
       $mail->SMTPAuth = true;                                // Enable SMTP authentication
       $mail->Username = $mail_username;                      // Var is set in config.php
       $mail->Password = $mail_pass;                          // Var is set in config.php
       $mail->SMTPSecure = 'tls';                             // Enable TLS encryption, `ssl` also accepted
       $mail->Port = 587;                                     // TCP port to connect to
       $mail->isHTML(true);

       $mail->setFrom($mail_from_addr, $mail_from_text);      // Var is set in config.php
       $mail->addAddress($mail_to_addr, $mail_to_text);       // Var is set in config.php

       $mail->Subject = 'Contact form from olback.net';
       $mail->Body    = $htmlContent;
       $mail->AltBody = 'This is the body in plain text for non-HTML mail clients';

       if(!$mail->send()) {
                echo 'Message could not be sent.';
                echo 'Mailer Error: ' . $mail->ErrorInfo;
            } else {
                $status = '<p class="w3-center w3-large w3-text-green">Mail sent! I\'ll get back to you shortly!</p>';
                $scrlToContact = '<script>document.getElementById("contact").scrollIntoView({behavior:"smooth"});;</script>';
            }

    } else {
        //False - What happens when user is not verified
        $status = '<p class="w3-center w3-large w3-text-red">Robot verification failed. :(</p>';
        $scrlToContact = '<script>document.getElementById("contact").scrollIntoView({behavior:"smooth"});;</script>';
    }
}
?>
<!DOCTYPE html>
<html>
  <head>
    <title>Edwin - olback.net</title>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="theme-color" content="#333">
    <link rel="stylesheet" href="css/w3.css">
    <link rel="stylesheet" href="css/styles.css">
    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Raleway">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">
    <script src='https://www.google.com/recaptcha/api.js'></script> <!-- Google reCaptcha JS -->
    <link rel="shortcut icon" type="image/png" href="img/olback.jpg"/>
  </head>
  <body class="w3-light-grey">

    <!-- Sidebar on small screens when clicking the menu icon -->
    <nav class="w3-sidebar w3-bar-block w3-black w3-card-2 w3-animate-left w3-hide-medium w3-hide-large" style="display:none; top: 0px;" id="mySidebar">
      <hr>
      <a href="#top" onclick="w3_close()" class="w3-bar-item w3-button w3-large">Top <i class="fa fa-arrow-up"></i></a>
      <hr>
      <a href="#about" onclick="w3_close()" class="w3-bar-item w3-button w3-large">About</a>
      <hr>
      <a href="#projects" onclick="w3_close()" class="w3-bar-item w3-button w3-large">Projects</a>
      <hr>
      <a href="#contact" onclick="w3_close()" class="w3-bar-item w3-button w3-large">Contact</a>
      <hr>
      <a href="javascript:void(0)" onclick="w3_close()" class="w3-bar-item w3-button w3-large">Close <i class="fa fa-times" aria-hidden="true"></i></a>
      <hr>
    </nav>

    <div class="span" id="close">
      <!-- Navbar (sit on top) -->
      <div class="w3-top">
        <div class="w3-bar w3-transparent w3-card-2 w3-text-white no-shadows" id="myNavbar">
          <a onclick="scrlTo('home');" class="ch w3-bar-item w3-button w3-wide">olback.net</a>
          <!-- Right-sided navbar links -->
          <div class="w3-right w3-hide-small">
            <a onclick="scrlTo('about');" class="ch w3-bar-item w3-button">About</a>
            <a onclick="scrlTo('projects');" class="ch w3-bar-item w3-button">Projects</a>
            <a onclick="scrlTo('contact');" class="ch w3-bar-item w3-button">Contact</a>
          </div>
          <!-- Hide right-floated links on small screens and replace them with a menu icon -->

          <a href="javascript:void(0)" class="ch w3-bar-item w3-button w3-right w3-hide-large w3-hide-medium" onclick="w3_open()">
            <i class="fa fa-bars"></i>
          </a>
        </div>
      </div>

      <!-- Header with full height image -->
      <header class="bgimg-1 w3-display-container" id="home">
        <div class="w3-text-white centerincssisabitch" style="padding:48px">
          <span class="w3-jumbo w3-hide-small"><img src="img/edwin.jpg" class="w3-circle x300" alt="Edwin - olback.net"></span><br>
          <span class="w3-xxlarge w3-hide-large w3-hide-medium"><img src="img/edwin.jpg" class="w3-circle p100" alt="Edwin - olback.net"></span><br>
          <span class="w3-large">Hey, I'm Edwin.</span>
        </div>

        <!-- Social media buttons -->
        <div class="w3-display-bottomleft w3-text-grey w3-xlarge social-icons">
          <a class="fa fa-twitter twitter-hover" href="https://twitter.com/mrolback"></a>
          <a class="fa fa-facebook-official fb-hover" href="https://www.facebook.com/1edwinsvensson"></a>
          <a class="fa fa-github gh-hover" href="https://github.com/olback"></a>
          <a class="fa fa-instagram insta-hover" href="https://www.instagram.com/mredwinn/"></a>
          <a class="fa fa-snapchat snap-hover" href="https://www.snapchat.com/add/olbackxdd"></a>
        </div>
        <p><a href="javascript:scrlTo('about');" class="arrow bottom-button bounce"></a></p>
      </header>

      <!-- About Section -->
      <div class="w3-container w3-center w3-light-grey" style="padding:128px 16px" id="about">
        <h2 class="w3-center">About me</h2>
        <p class="w3-center w3-large">I'm a student from Sweden and I study engineering/IT at the moment. I really like maths and physics, don't really know why. In my spare time I like to design and develop websites as well as photography. Can't really say I'm good at it but I'm learning and it's a whole lot of fun. </p>
      </div>

      <!-- Projects Section -->
      <div class="w3-container w3-white" style="padding:128px 16px" id="projects">
        <h2 class="w3-center">My projects</h2>
        <p class="w3-center w3-large">This is just a few of my projects, you can find most of them on <a href="https://github.com/olback">GitHub</a>.</p>
        <div class="w3-row-padding w3-center" style="margin-top:64px">
          <div class="w3-quarter">
            <p class="w3-large">olback.ninja</p>
            <p><a href="https://olback.ninja">olback.ninja</a> is available on <a href="https://github.com/olback/olback.ninja">GitHub</a>.<br>100% pointless...</p>
          </div>
          <div class="w3-quarter">
            <p class="w3-large">WHOIS Lookup</p>
            <p>Don't want to use someone else's WHOIS lookup service?<br>Just host your own.<br>Source code available on <a href="https://github.com/olback/simple-whois-lookup">GitHub</a>.</p>
          </div>
          <div class="w3-quarter">
            <p class="w3-large">EsyShop</p>
            <p>This is a dead school project, but you can still try it out. Read more on <a href="https://esyshop.se">esyshop.se</a> for more information.</p>
          </div>
          <div class="w3-quarter">
            <p class="w3-large">olback.net</p>
            <p>You're here right now.<br>Source code is available on <a href="https://github.com/olback/olback.net">GitHub</a>.</p>
          </div>
        </div>
      </div>

      <!-- Contact Section -->
      <div class="w3-light-grey w3-center" style="padding:128px 16px" id="contact">
        <h2 class="w3-center">Contact</h2>
        <p class="w3-center w3-large">Lets get in touch. Send me a message:</p>
        <?php if (isset($status)) {echo $status;}?>
          <form method="POST" action="index.php">
             <p class="contact-form"><input class="w3-input w3-border" type="text" placeholder="Name" required name="name"></p>
             <p class="contact-form"><input class="w3-input w3-border" type="email" placeholder="Email" required name="email"></p>
             <p class="contact-form"><input class="w3-input w3-border" type="text" placeholder="Subject" required name="subject"></p>
             <p class="contact-form"><textarea class="w3-input w3-border" placeholder="Message" required name="message"></textarea></p>
             <div class="g-recaptcha" data-sitekey="<?php echo $publicKey; ?>"></div>
             <p>
               <button class="w3-button w3-black" type="submit" name="submit">
                 <i class="fa fa-paper-plane"></i> Send message
               </button>
             </p>
          </form>
      </div>

      <!-- Footer -->
      <footer class="w3-center footer w3-padding-32">
        <a onclick="scrlTo('home');" class="w3-button w3-light-grey"><i class="fa fa-arrow-up w3-margin-right"></i>To the top</a>
        <div class="w3-xlarge w3-section">
          <a class="fa fa-twitter twitter-hover" href="https://twitter.com/mrolback"></a>
          <a class="fa fa-facebook-official fb-hover" href="https://www.facebook.com/1edwinsvensson"></a>
          <a class="fa fa-github gh-hover" href="https://github.com/olback"></a>
          <a class="fa fa-instagram insta-hover" href="https://www.instagram.com/mredwinn/"></a>
          <a class="fa fa-snapchat snap-hover" href="https://www.snapchat.com/add/olbackxdd"></a>
        </div>
        <p>CSS Framework by <a href="https://www.w3schools.com/w3css/default.asp" title="W3.CSS" target="_blank" class="w3-hover-text-green">w3.css</a><br>
          &copy; <script>document.write(new Date().getFullYear())</script> <a href="https://olback.net">olback.net</a>.</p>
      </footer>

      <script src="js/main.js"></script>
      <?php if (isset($scrlToContact)) {echo $scrlToContact;}?>
    </div>
  </body>
</html>
