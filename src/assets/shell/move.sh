#!/bin/bash

for file in ./bin/compiled*; do
    cp "$file" "./"
        if [ $? -eq 0 ]; then
            echo "Arquivo $file copiado com sucesso!"
        else
            echo "Falha ao copiar o arquivo $file."
        fi
done

rm $0