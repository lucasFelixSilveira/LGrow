#!/bin/bash
# -*- coding: utf-8 -*-

compiled_file=$(find . -maxdepth 1 -type f -name "compiled.*" | head -n 1)
console_log="console.log"

if [ -x "$compiled_file" ]; then
    export LANG=en_US.UTF-8
    ./"$compiled_file" > /dev/null &
    pid=$!
    
    while true; do
        if ! ps -p $pid > /dev/null; then
            break
        fi
        
        if [ -f "$console_log" ]; then
            cat "$console_log"
            rm "$console_log"
            echo   
        fi        
    done
else
    echo "Arquivo executável 'compiled' não encontrado na pasta atual."
fi

echo 
echo "LGrow - Compiled program testing terminal"
read -n 1 -s -r -p "Press any key to end the process..."