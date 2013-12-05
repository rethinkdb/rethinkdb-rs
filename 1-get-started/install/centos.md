---
layout: documentation
title: Install RethinkDB on CentOS
title_image: /assets/images/docs/install-platforms/centos.png
active: docs
docs_active: install
permalink: docs/install/centos/
---
{% include install-docs-header.md %}

# With binaries #

We provide binaries for both 32-bit and 64-bit CentOS 6.

To install the server, add the [RethinkDB yum repository](http://download.rethinkdb.com/centos) to your list of repositories and install:

```bash
sudo wget http://download.rethinkdb.com/centos/6/`uname -m`/rethinkdb.repo \
          -O /etc/yum.repos.d/rethinkdb.repo
sudo yum install rethinkdb
```
# Compile from source with the Epel repository #

These instructions have been tested on CentOS 6.5.

## Get the build dependencies ##

CentOS provides neither a v8-devel package nor Node.js, so we need to get them
from the Epel repository:


```bash
sudo rpm -Uvh http://download.fedoraproject.org/pub/epel/6/x86_64/epel-release-6-8.noarch.rpm
```

Install the main dependencies:

```
sudo yum install git-core gcc-c++ ncurses-devel boost-static protobuf-devel nodejs \
    npm gperftools-devel
```

## Get the source code ##
Clone RethinkDB repository:

```bash
git clone --depth 1 -b v{{site.version.major}}.x https://github.com/rethinkdb/rethinkdb.git
```

## Build RethinkDB ##

Kick off the build process:

```bash
cd rethinkdb
./configure --dynamic tcmalloc_minimal
make
```

# Compile from source without the Epel repository #

These instructions have been tested on CentOS 6.5.

## Get the build dependencies ##

Install the main dependencies:

```
sudo yum install git-core gcc-c++ ncurses-devel boost-static svn
```

## Get the source code ##

Clone RethinkDB repository:

```bash
git clone --depth 1 -b v{{site.version.major}}.x https://github.com/rethinkdb/rethinkdb.git
```

## Build RethinkDB ##

Kick off the build process:

```
cd rethinkdb
./configure --fetch protoc
make
```


{% include install-next-step.md %}
