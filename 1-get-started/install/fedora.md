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

The following instructions were tested on Fedora 19.

## Get the build dependencies ##
Install the main dependencies:

```bash
sudo yum install git-core gcc-c++ protobuf-devel nodejs npm ncurses-devel \
    gperftools-devel boost-static
```

## Get the source code ##
Clone the RethinkDB repository:

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

{% include install-next-step.md %}
