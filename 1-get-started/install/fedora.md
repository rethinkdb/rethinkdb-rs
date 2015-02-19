---
layout: documentation
title: Install RethinkDB on Fedora
title_image: /assets/images/docs/install-platforms/fedora.png
active: docs
docs_active: install
permalink: docs/install/fedora/
---
{% include install-docs-header.md %}
{% include install-community-platform-warning.md %}

# With binaries #

The <a href="/docs/install/centos/">Centos RPMs</a> are known to work with
Fedora.

To install the server, add the RethinkDB yum repository to your list of repositories and install:

```bash
sudo wget http://download.rethinkdb.com/centos/6/`uname -m`/rethinkdb.repo \
    -O /etc/yum.repos.d/rethinkdb.repo
sudo yum install rethinkdb
```


# Compile from source #

The following instructions were tested on Fedora 20.

## Get the build dependencies ##

Install the main dependencies:

```bash
sudo yum install gcc-c++ protobuf-devel ncurses-devel jemalloc-devel \
         boost-static wget protobuf-compiler which zlib-devel \
         openssl-devel libcurl-devel make m4
```

## Get the source code ##

Download and extract the source tarball:

```bash
wget http://download.rethinkdb.com/dist/rethinkdb-{{site.version.full}}.tgz
tar xf rethinkdb-{{site.version.full}}.tgz
```

## Build RethinkDB ##

Kick off the build process:

```bash
cd rethinkdb-{{site.version.full}}
./configure --dynamic jemalloc
make
sudo make install
```

{% include install-next-step.md %}
