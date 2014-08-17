---
layout: documentation
title: Building RethinkDB from source
active: docs
docs_active: install
permalink: docs/build/
---

These are generic build instructions. Take a look at the [install page](/docs/install/)
if you are looking for a specific platform.

# Building from source #

## Get the build dependencies ##

There are a number of packages required for the build process. Most
should be available for your operating system's repository. These packages are:

- git
- g++
- protobuf
- gperftools
- ncurses
- boost
- nodejs/npm
- Python 2
- libcurl


On Ubuntu 13.10+, you can install build dependencies with apt-get ([see instructions for previous versions](/docs/install/ubuntu/)):

```bash
sudo apt-get install git-core g++ nodejs npm libprotobuf-dev \
libgoogle-perftools-dev libncurses5-dev libboost-all-dev nodejs-legacy \
curl libcurl3 libcurl4-openssl-dev protobuf-compiler
```

## Get the source code ##

Clone the RethinkDB repository:

```bash
git clone --depth 1 -b v{{site.version.major}}.x https://github.com/rethinkdb/rethinkdb.git
```

## Build the server ##

Kick off the build process:

```bash
cd rethinkdb
./configure --allow-fetch
make
```

If you're compiling on a multicore or multiprocessor machine, you may be able to use `make -j #` to speed up the build process, where '#' is the total number of CPU cores. (On a 4-core machine, you can use `make -j 4`.) However, some older versions of `make` will produce a segmentation fault error when using `-j` with RethinkDB; if that happens, just run `make` without the `-j` option.

You'll find the `rethinkdb` binary in the `build/release/` subfolder.
