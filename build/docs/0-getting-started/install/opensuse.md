---
layout: documentation
title: Install RethinkDB on openSUSE
title_image: /assets/images/docs/install-platforms/opensuse.png
docs_active: install
permalink: docs/install/opensuse/
---
{% include docs/install-docs-header.md %}
{% include docs/install-community-platform-warning.md %}

These instructions have been tested on OpenSuse 13.1.

# Compile from source #

## Get the dependencies ##

Install the main dependencies:

```
sudo zypper in make gcc gcc-c++ protobuf-devel ncurses-devel \
               jemalloc-devel boost-devel tar wget m4 which \
               openssl-devel libcurl-devel
```

## Get the source code ##

Download and extract the archive:

```bash
wget https://download.rethinkdb.com/dist/rethinkdb-{{site.version.full}}.tgz
tar xf rethinkdb-{{site.version.full}}.tgz
```

## Build RethinkDB ##

Kick off the build process:

```
cd rethinkdb-{{site.version.full}}
./configure --dynamic jemalloc
make
sudo make install
```

{% include docs/install-next-step.md %}
