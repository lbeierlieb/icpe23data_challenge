#!/bin/sh

echo "Downloading archive with all raw files"
wget --verbose --output-document=jmh.tar.gz https://zenodo.org/record/5961018/files/jmh.tar.gz?download=1

echo "Extracting"
tar -xvf jmh.tar.gz
mv data/jmh jmh
rmdir data

echo "Removing failed benchmarks"
for FILE in $(cat invalid_raw_files)
do
    rm --verbose "jmh/"$FILE
done
