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
# Compile from source #

## Get the build dependencies ##
CentOS provides neither a v8-devel package nor Node.js, so we need to get them
from the Epel repository:

```bash
sudo rpm -Uvh http://download.fedoraproject.org/pub/epel/6/x86_64/epel-release-6-8.noarch.rpm
```

Install the main dependencies:

```bash
sudo yum install git-core boost-static m4 gcc-c++ python-pip v8-devel nodejs npm
```

Install PyYAML, which is required for building the internal documentation:

```bash
sudo python-pip install pyyaml
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
./configure --without-tcmalloc --fetch protoc --fetch protobuf
make
```

If you want to build faster and use multiple cores, you will have to use
this command to avoid a bug in the unpatched `make` available on CentOS (see
[https://github.com/rethinkdb/rethinkdb/issues/475](https://github.com/rethinkdb/rethinkdb/issues/475)):

```
`make command-line` -j3
```

{% include install-next-step.md %}
