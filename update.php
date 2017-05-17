<?php
$output = shell_exec('git pull https://github.com/olback/olback.net.git master');
http_response_code(200);
print($output);
?>
