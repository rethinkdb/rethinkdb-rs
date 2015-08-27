---
layout: documentation
title: Install RethinkDB on CentOS
title_image: /assets/images/docs/install-platforms/centos.png
docs_active: install
permalink: docs/install/centos/
---
{% include docs/install-docs-header.md %}

# With binaries #

We provide binaries for both 32-bit and 64-bit CentOS 6.

To install the server, add the [RethinkDB yum repository](http://download.rethinkdb.com/centos) to your list of repositories and install:

```bash
sudo wget https://download.rethinkdb.com/centos/6/`uname -m`/rethinkdb.repo \
          -O /etc/yum.repos.d/rethinkdb.repo
sudo yum install rethinkdb
```

# Compile from source #

These instructions have been tested on CentOS 6.5.

## Get the build dependencies ##

The version of GCC included with Centos 6 is too old to compile RethinkDB. A newer version can be installed using devtoolset:

```bash
rpm --import http://ftp.scientificlinux.org/linux/scientific/5x/x86_64/RPM-GPG-KEYs/RPM-GPG-KEY-cern
sudo wget -O /etc/yum.repos.d/slc6-devtoolset.repo http://linuxsoft.cern.ch/cern/devtoolset/slc6-devtoolset.repo
```

Install the main dependencies:

```
sudo yum install devtoolset-2 ncurses-devel boost-static openssl-devel \
                 libcurl-devel wget tar which m4
```

### Install optional build dependencies ###

CentOS provides neither a protobuf-devel package nor a jemalloc-devel
package. Installing these dependencies from the EPEL repositories will
allow RethinkDB to build more quickly:

```bash
sudo rpm -Uvh https://download.fedoraproject.org/pub/epel/6/x86_64/epel-release-6-8.noarch.rpm
sudo yum install protobuf-devel jemalloc-devel
```

## Get the source code ##

Download and extract the source tarball:

```bash
wget https://download.rethinkdb.com/dist/rethinkdb-{{site.version.full}}.tgz
tar xf rethinkdb-{{site.version.full}}.tgz
```

## Build RethinkDB ##

Kick off the build process:

```bash
cd rethinkdb-{{site.version.full}}
scl enable devtoolset-2 -- ./configure --dynamic jemalloc --allow-fetch
scl enable devtoolset-2 -- make
sudo make install
```

{% include docs/install-next-step.md %}
