#!/bin/bash

CIRCLE_BUILD_NUM=190
CIRCLE_NODE_INDEX=0
# URL="https://olback.net/update"

JSON="{\"build_num\": \"$CIRCLE_BUILD_NUM\",\"node_index\": \"$CIRCLE_NODE_INDEX\"}"

curl \
    --header "Content-Type: application/json"\
    --data "$JSON" \
    --request POST \
    --include \
    --show-error \
    --fail \
    $URL
