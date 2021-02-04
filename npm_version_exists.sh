#!/bin/bash

{
    args=("$@")
    LIB_VERSION=${args[0]}

    VERSIONS=$(npm view smmdb versions)
    VERSIONS=$(echo $VERSIONS | sed 's/[][]//g' | sed 's/[[:space:]]//g')

    VERION_EXISTS=false

    while IFS=',' read -ra VERSION; do
        for V in "${VERSION[@]}"; do
            if [ "$(echo \'${LIB_VERSION}\')" == $V ]; then
                VERION_EXISTS=true
                break
            fi
        done
    done <<< "$VERSIONS"
} >/dev/null 2>&1

echo $VERION_EXISTS
