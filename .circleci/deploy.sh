#!/bin/bash

URL="https://olback.net/update"

if [ $DEV ]; then

    CIRCLE_BUILD_NUM=244
    CIRCLE_NODE_INDEX=0
    URL="http://localhost:5000/update"

fi

JSON="{\"build_num\": \"$CIRCLE_BUILD_NUM\",\"node_index\": \"$CIRCLE_NODE_INDEX\"}"

echo $JSON

curl \
    --header "Content-Type: application/json"\
    --data "$JSON" \
    --request POST \
    --include \
    --show-error \
    --fail \
    $URL
