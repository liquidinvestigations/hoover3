#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

cd ../../
mkdir -p data/crime
cd data/crime
pwd

for i in 1 10 100 1000 10000 ; do
    for f in json csv tsv rdf xml ; do
        FILENAME="crimes-${i}k.${f}"
        if [ -f ${FILENAME} ] ; then
            continue
        fi
        wget "https://data.cityofchicago.org/resource/crimes.${f}?\$limit=${i}000" -O ${FILENAME}.tmp &
    done
    wait
    for f in json csv tsv rdf xml ; do
        FILENAME="crimes-${i}k.${f}"
        if [ -f ${FILENAME} ] ; then
            continue
        fi
        mv ${FILENAME}.tmp ${FILENAME}
    done
done
