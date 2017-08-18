<?php
include __DIR__ . '/assets/config.php';
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
      require __DIR__ . '/assets/mail/PHPMailerAutoload.php';
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
                $status = '<p style="color:green;">Mail sent! I\'ll get back to you shortly!</p>';
                $scrlToContact = '<script>document.getElementById("contact").scrollIntoView({behavior:"smooth"});;</script>';
            }
    } else {
        //False - What happens when user is not verified
        $status = '<p style="color:red;">Robot verification failed. :(</p>';
        $scrlToContact = '<script>document.getElementById("contact").scrollIntoView({behavior:"smooth"});;</script>';
    }
}
?>
<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Edwin - olback.net</title>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <meta name="theme-color" content="#333">
        
        <!-- CSS -->
        <link href="assets/styles.css" rel="stylesheet">
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">
        
        <!-- Google reCaptcha JS -->
        <script src='https://www.google.com/recaptcha/api.js'></script>

        <!-- Fav icon -->
        <link rel="shortcut icon" type="image/png" href="assets/olback.jpg"/>
    </head>
    <body>
        <nav class="transparent" id="nav">
            <a onclick="scrlTo('body');" class="item title">olback.net</a>
            <div>
                <a onclick="scrlTo('contact');" class="item right">Contact</a>
                <a onclick="scrlTo('projects');" class="item right">Projects</a>
                <a onclick="scrlTo('about');" class="item right">About</a>
            </div>
        </nav>

        <div id="body">
            <header>
                <div class="edwin">
                    <img src="./assets/edwin.jpg"/><br>
                    <p>Hey, I'm Edwin.</p>
                </div>
                <div class="social-icons">
                    <a class="fa fa-twitter" href="https://twitter.com/mrolback"></a>
                    <a class="fa fa-facebook-official" href="https://www.facebook.com/1edwinsvensson"></a>
                    <a class="fa fa-github" href="https://github.com/olback"></a>
                    <a class="fa fa-instagram" href="https://www.instagram.com/mredwinn/"></a>
                    <a class="fa fa-snapchat" href="https://www.snapchat.com/add/olbackxdd"></a>
                </div>
                <span class="showArrow"><a href="javascript:scrlTo('about');" class="arrow bounce"></a></span>
            </header>

            <div class="text-container lgray" id="about">
                <h2>About me</h2>
                <p>Hey, my name is Edwin and i study IT/engineering in Sweden at the moment. I am <span id="myAge"></span><noscript>&nbsp|&nbsp&nbspEnable JavaScript to see my age 😂&nbsp&nbsp|&nbsp</noscript> years old. I really like maths and physics, don't really know why. In my spare time I like to design and develop websites. I also like photogrophy. I can't really say that I'm particulary good at any of the aformentioned things but it's a whole lot of fun. I'm also a fan of open source software and hardware. </p>
            </div> <!-- end of about -->

            <div class="text-container dib white" id="projects">
                <h2>Projects</h2>
                <p>This is just a few of my projects, you can find most of them on <a href="https://github.com/olback">GitHub</a>.</p>
                <div class="quarter">
                    <div class="padding">
                        <h3>RaspberryPi Web Interface</h3>
                        <p>A nice, clean web interface that let's you monitor CPU usage, memory usage and more. Check it out <a href="https://github.com/olback/rpi-webint">here</a>.</p>
                    </div>
                </div>
                <div class="quarter">
                    <div class="padding">
                        <h3>WHOIS Lookup</h3>
                        <p>Don't want to use someone else's WHOIS lookup service? Just host your own.<br>Source code available on <a href="https://github.com/olback/simple-whois-lookup">GitHub</a>.</p>
                    </div>
                </div>
                <div class="quarter">
                    <div class="padding">
                        <h3>EsyShop</h3>
                        <p>This is a dead school project, but you can still try it out. Read more on <a href="https://esyshop.se">esyshop.se</a> for more information.</p>
                    </div>
                </div>
                <div class="quarter">
                    <div class="padding">
                        <h3>GTA Session Maker</h3>
                        <p>Are you annoyed by hackers and/or griefers in GTA Online? Here's the soloution!<br><a href="https://github.com/olback/gta-session/releases/latest">Download</a> | <a href="https://github.com/olback/gta-session">Source</a></p>
                    </div>
                </div>
            </div><!-- end of projects -->

            <div class="text-container lgray" id="contact">
                <h2>Contact me</h2>
                <p>Please feel free to contact me using the form below.</p>
                <div class="form">
                    <?php if (isset($status)) {echo $status;}?>
                    <form method="POST">
                        <input class="input" type="name" name="name" id="name" placeholder="Your name" required />
                        <input class="input" type="email" name="email" id="email" placeholder="Your email" required />
                        <input class="input" type="text" name="subject" id="subject" placeholder="Subject" required />
                        <textarea class="input" type="text" name="message" id="message" placeholder="Message" required></textarea>
                        <div class="g-recaptcha" data-sitekey="<?php echo $publicKey; ?>"></div><br>
                        <button type="submit" name="submit"><i class="fa fa-paper-plane" aria-hidden="true"></i> Send message</button>
                    </form>
                </div>
            </div><!-- end of contact -->

            <div class="text-container footer" id="footer">
                <div class="social-icons">
                    <a class="fa fa-twitter" href="https://twitter.com/mrolback"></a>
                    <a class="fa fa-facebook-official" href="https://www.facebook.com/1edwinsvensson"></a>
                    <a class="fa fa-github" href="https://github.com/olback"></a>
                    <a class="fa fa-instagram" href="https://www.instagram.com/mredwinn/"></a>
                    <a class="fa fa-snapchat" href="https://www.snapchat.com/add/olbackxdd"></a>
                </div>
                <p>&copy <span id="year"></span> olback</p>
            </div> <!-- end of footer -->
        </div>

    </body>
    <script src="assets/main.js"></script>
    <?php if (isset($scrlToContact)) {echo $scrlToContact;}?>
</html>