#!/bin/bash

echo 'make sure the following libraries are available on your system:'
echo "sudo apt install 'cmake' 'binutils-dev' 'libcurl4-openssl-dev' 'zlib1g-dev' 'libdw-dev' 'libiberty-dev'"
echo ''

mkdir 'kcov-install'
wget -O 'kcov-install/kcov.tar.gz' https://github.com/SimonKagstrom/kcov/archive/master.tar.gz

cd 'kcov-install'
tar xzf 'kcov.tar.gz'
cd 'kcov-master'
mkdir 'build'
cd 'build'
cmake -DCMAKE_INSTALL_PREFIX='./' ..
make
make install

cd ../../..
cp 'kcov-install/kcov-master/build/bin/kcov' 'kcov'
