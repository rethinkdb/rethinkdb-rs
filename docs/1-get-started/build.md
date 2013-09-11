---
layout: documentation
title: Building RethinkDB from source
active: docs
docs_active: install
permalink: docs/build/
---

There are two ways to get the RethinkDB source code:

* You can download a prepacked source code distribution
* You can clone the RethinkDB git repository

# Building from a prepackaged source distribution #

## Get the source code ##

To start building RethinkDB from source, first download and extract the RethinkDB source distribution:

```bash
wget http://download.rethinkdb.com/dist/rethinkdb-{{site.version.major}}.{{site.version.minor}}.tgz
tar xf rethinkdb-{{site.version.major}}.{{site.version.minor}}.tgz
cd rethinkdb-{{site.version.major}}.{{site.version.minor}}/
```

To build the latest development version of RethinkDB, follow the instructions below 
for building with git.

## Install build dependencies ##

There are a number of packages required for the build process. Most
should be available for your operating system's repository. On Ubuntu,
you can install build dependencies with apt-get:

```bash
sudo apt-get install g++ protobuf-compiler protobuf-c-compiler libprotobuf-dev libv8-dev libgoogle-perftools-dev make libprotoc-dev default-jre libboost-dev
```

## Build the server ##

Configure and build RethinkDB:

```bash
./configure
make
```

You'll find the `rethinkdb` binary in the `build/release/` subfolder.

# Building with git #

RethinkDB is being developed using git. You can clone the repository via git:

```bash
git clone --depth 1 -b v{{site.version.major}}.x https://github.com/rethinkdb/rethinkdb.git
```

To get the latest working development branch, checkout `next` instead:

```bash
git clone --depth 1 https://github.com/rethinkdb/rethinkdb.git
```

Kick off the build: 

```bash
./configure
make
```
