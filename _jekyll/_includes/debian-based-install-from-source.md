# Compile from source #

## Get the build dependencies ##

Install the dependencies:

```bash
sudo apt-get install build-essential protobuf-compiler python \
                     libprotobuf-dev libcurl4-openssl-dev \
                     libboost-all-dev libncurses5-dev \
                     libjemalloc-dev wget
```

## Get the source code ##

Download and extract the archive:

```bash
wget https://download.rethinkdb.com/dist/rethinkdb-{{site.version.full}}.tgz
tar xf rethinkdb-{{site.version.full}}.tgz
```

## Build the server ##

Kick off the build process:

```bash
cd rethinkdb-{{site.version.full}}
./configure --allow-fetch
make
sudo make install
```

If you're compiling on a multicore or multiprocessor machine, you may
be able to use `make -j #` to speed up the build process, where '#' is
the total number of CPU cores. (On a 4-core machine, you can use `make
-j 4`.) However, some older versions of `make` will produce a
segmentation fault error when using `-j` with RethinkDB; if that
happens, just run `make` without the `-j` option.
