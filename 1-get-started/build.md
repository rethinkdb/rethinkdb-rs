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

- [GCC (G++)](https://gcc.gnu.org/) or [Clang](http://clang.llvm.org/)
- [Protocol Buffers](https://github.com/google/protobuf/)
- [jemalloc](http://www.canonware.com/jemalloc/)
- [Ncurses](https://www.gnu.org/software/ncurses/)
- [Boost](http://www.boost.org/)
- [Python 2](https://www.python.org/)
- [libcurl](http://curl.haxx.se/libcurl/)
- [libcrypto](https://www.openssl.org/)

On Ubuntu, you can install the build dependencies with apt-get, [following the instructions here](/docs/install/ubuntu/).

The `./configure` script can install some of these dependencies if they are missing.

## Get the source code ##

Download and extract the archive:

```bash
wget http://download.rethinkdb.com/dist/rethinkdb-{{site.version.full}}.tgz
tar xf rethinkdb-{{site.version.full}}.tgz
```

## Build the server ##

Kick off the build process:

```bash
cd rethinkdb-{{site.version.full}}
./configure --allow-fetch
make
```

# Building from git #

The git version of RethinkDB contains unreleased and unstable
changes. It is meant for developers and contributors.

## Get the build dependencies ##

In addition to the standard dependencies, building from git also
depends on [npm](http://nodejs.org/).

## Get the source code ##

Clone the development branch:

```bash
git clone https://github.com/rethinkdb/rethinkdb.git
```

## Build RethinkDB ##

Kick off the build process:

```bash
cd rethinkdb
./configure --allow-fetch
make
```

If you're compiling on a multicore or multiprocessor machine, you may be able to use `make -j #` to speed up the build process, where '#' is the total number of CPU cores. (On a 4-core machine, you can use `make -j 4`.) However, some older versions of `make` will produce a segmentation fault error when using `-j` with RethinkDB; if that happens, just run `make` without the `-j` option.

You'll find the `rethinkdb` binary in the `build/release/` subfolder.
