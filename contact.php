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
      $headers = "MIME-Version: 1.0" . "\r\n";
      $headers .= "Content-type:text/html;charset=UTF-8" . "\r\n";
      $headers .= 'From:' . $name . ' <' . $email . '>' . "\r\n";
      @mail($to, $subject, $htmlContent, $headers);

      header('Location: contact.html');
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

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta http-equiv="content-type" content="text/html; charset=UTF-8">
    <title>olback.net</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="shortcut icon" href="./img/olback.jpg">
    <link rel="stylesheet" href="./css/styles.css">
    <link rel="stylesheet" href="./css/form.css">
    <link rel="stylesheet" href="css/fa/css/font-awesome.css">
    <script type="text/javascript" src="/js/google.js"></script>
    <script src='https://www.google.com/recaptcha/api.js'></script>
  </head>
  <body>
    <?php include __DIR__ . '/res/navbar.php'; ?>
    <div class="content">
      <div class="container">
        <div class="main">
          <h1>Contact</h1>
          <hr>
          <form name="contactform" method="POST" action="send_form_email2.php">
            <input name="name" type="text" class="feedback-input" placeholder="Name">   
            <input name="email" type="email" class="feedback-input" placeholder="Email">
            <textarea name="message" class="feedback-input" placeholder="Message"></textarea>
            <center><div class="g-recaptcha" data-sitekey="6LeFkykTAAAAAKO1TkoZtHvxoQuzZq6o3FKPj958"></div><br></center>
            <input type="submit" value="Submit" name="submit">
          </form>
          <br>
          <p>
          <center>
            <a href="https://twitter.com/mrolback">
              <i class="fa fa-twitter" aria-hidden="true" style="font-size: 500%; color: #55acee;"></i>
            </a><br><br>
            <a href="https://github.com/olback">
              <i class="fa fa-github" aria-hidden="true" style="font-size: 500%; color: black;"></i>
            </a><br><br>
            <p>Jabber: <a href="https://olback.net/dev/urandom/jfinger.html"> XMPP</a>
            </p>
          </center>
        </div>
      </div>
    </div>
    <?php include __DIR__ . '/res/footer.php'; ?>
  </body>
</html>
