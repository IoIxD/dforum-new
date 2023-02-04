<?php
    include("vendor/autoload.php");

    include("static/pages/header.php");

    $client = new WebSocket\Client("ws://localhost:8084");
    // send a test message.
    $client->text(json_encode(array(
        "id"=>"ping"
    )));
    // decode what comes back.
    $result = json_decode($client->receive());
    print_r(((array) $result)["id"]);
    
    $client->close();

    include("static/pages/index.php");
    include("static/pages/footer.php");
?>
