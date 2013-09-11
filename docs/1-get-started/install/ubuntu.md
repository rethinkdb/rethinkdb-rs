---
layout: documentation
title: Install RethinkDB on Ubuntu
title_image: /assets/images/docs/install-platforms/ubuntu.png
active: docs
docs_active: install
permalink: docs/install/ubuntu/
---
{% include install-docs-header.md %}

# With binaries #
We provide binaries for both 32-bit and 64-bit Ubuntu Lucid and above (>= 10.04).

To install the server, you have to add the [RethinkDB
PPA](https://launchpad.net/~rethinkdb/+archive/ppa) to your list of
repositories and install via `apt-get`. 
To do this, paste the
following lines into your terminal:

```bash
sudo add-apt-repository ppa:rethinkdb/ppa   && \
sudo apt-get update                         && \
sudo apt-get install rethinkdb
```

If you do not have the `add-apt-repository` command, install it first:

* Ubuntu Quantal and above (>= 12.10) &mdash; `sudo apt-get install software-properties-common`
* Earlier Ubuntu versions (< 12.10) &mdash; `sudo apt-get install python-software-properties`

# Compile from source #

## Get the build dependencies ##

There are a number of packages required for the build process. On Ubuntu, you can install build dependencies with apt-get:

```bash
sudo apt-get install g++ protobuf-compiler protobuf-c-compiler libprotobuf-dev
libv8-dev libgoogle-perftools-dev make libprotoc-dev default-jre libboost-dev
python-pip python-dev libyaml-dev nodejs npm git-core
```

Install PyYAML, which is required for building the internal documentation:

```bash
sudo pip install pyyaml
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
./configure
make
```

You will find the `rethinkdb` binary in the `build/release/` subfolder.  

{% include install-next-step.md %}
