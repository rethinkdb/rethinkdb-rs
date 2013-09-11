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

# Compile from source #

The following instructions were tested on Fedora 18.

## Get the build dependencies ##
Install the main dependencies:

```bash
sudo yum install gcc-c++ protobuf-compiler protobuf-c protobuf-c-devel protobuf-devel boost-devel openssl-devel v8-devel gperftools-devel nodejs npm curl ctags-etags m4 protobuf-static boost-static git-core python-pip
```

Install PyYAML, which is required for building the internal documentation:

```bash
sudo python-pip install pyyaml
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
./configure --without-tcmalloc
make
```

{% include install-next-step.md %}
