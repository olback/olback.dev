<?php
if (isset($_POST['submit']) && !empty($_POST['submit'])):
  if (isset($_POST['g-recaptcha-response']) && !empty($_POST['g-recaptcha-response'])):
    $secret = '6LeFkykTAAAAANm39F0-v-CCFFgPw3stCNaiHwAK';
    $verifyResponse = file_get_contents('https://www.google.com/recaptcha/api/siteverify?secret=' . $secret . '&response=' . $_POST['g-recaptcha-response']);
    $responseData = json_decode($verifyResponse);
    if ($responseData->success):
      $name = !empty($_POST['name']) ? $_POST['name'] : '';
      $email = !empty($_POST['email']) ? $_POST['email'] : '';
      $message = !empty($_POST['message']) ? $_POST['message'] : '';

      $to = 'contact@olback.net';
      $subject = 'New contact form have been submitted';
      $htmlContent = "
                <h1>Contact request details</h1>
                <p><b>Name: </b>" . $name . "</p>
                <p><b>Email: </b>" . $email . "</p>
                <p><b>Message: </b>" . $message . "</p>
            ";
	    include __DIR__ . '/res/config.php';
            require __DIR__ . '/mail/PHPMailerAutoload.php';
            $mail = new PHPMailer;

            $mail->isSMTP();                                      // Set mailer to use SMTP
            $mail->Host = 'mail.olback.net';  // Specify main and backup SMTP servers
            $mail->SMTPAuth = true;                               // Enable SMTP authentication
            $mail->Username = 'olback@olback.net';                 // SMTP username
            $mail->Password = $mail_pass; // SMTP password
            $mail->SMTPSecure = 'tls';                            // Enable TLS encryption, `ssl` also accepted
            $mail->Port = 587;                                    // TCP port to connect to
            $mail->isHTML(true);

            $mail->setFrom('cfo@olback.net', 'olback.net');
            $mail->addAddress('cfo@olback.net', 'Contact-Form olback.net');     // Add a recipient

            $mail->Subject = $subject;
            $mail->Body    = $message;
            $mail->AltBody = 'This is the body in plain text for non-HTML mail clients';

            if(!$mail->send()) {
                echo 'Message could not be sent.';
                echo 'Mailer Error: ' . $mail->ErrorInfo;
            } else {
                //echo 'Message has been sent';
            }

      $succMsg = "Mail sent! We'll get back to you shortly.";
      //header('Location: contact.php');
    else:
      $errMsg = 'Robot verification failed, please try again.';
    endif;
  else:
    $errMsg = 'Please click on the reCAPTCHA box.';
  endif;
else:
  $errMsg = '';
  $succMsg = '';
endif;
?>

<?php $_SESSION['fn'] = basename(__FILE__, '.php'); ?>
<?php include __DIR__.'/res/head.php';?>
  <body>
    <?php include __DIR__ . '/res/navbar.php'; ?>
    <div class="content">
      <div class="container">
        <div class="main">
          <h1>Contact</h1>
          <hr>
          <p style="color: red;">
	    <?php
		if(isset($errMsg)){
		echo $errMsg;
		}
	     ?>
	   </p>
	   <p style="color: green;">
             <?php
		if(isset($succMsg)){
		echo $succMsg;
		}
	      ?>
 	  </p>
	  <div class="l">
          <form name="contactform" method="POST" action="">
            <input name="name" type="text" class="feedback-input" placeholder="Name">
            <input name="email" type="email" class="feedback-input" placeholder="Email" pattern="[a-zA-Z0-9!#$%&'*+\/=?^_`{|}~.-]+@[a-zA-Z0-9-]+(\.[a-zA-Z0-9-]+)*">
            <textarea name="message" class="feedback-input" placeholder="Message"></textarea>
	    <script src='https://www.google.com/recaptcha/api.js'></script>
            <div class="g-recaptcha" data-sitekey="6LeFkykTAAAAAKO1TkoZtHvxoQuzZq6o3FKPj958" data-theme="dark"></div><br>
            <input type="submit" value="Submit" name="submit">
          </form>
  	  </div>
          <div class="r">
          <center>
           <p style="margin: 10px; margin-top: 50px;">
	    If you'd like to contact me, please use the contact form below. Messages are NOT encrypted. Do not send any sensitive information.
	   </p>
           <br><br><br><br><br><br><br><br><br><br><br><br><br><br>
          </center>
	  </div>
        </div>
      </div>
    </div>
    <?php include __DIR__ . '/res/footer.php'; ?>
  </body>
</html>
