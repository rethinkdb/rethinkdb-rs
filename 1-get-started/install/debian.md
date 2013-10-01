---
layout: documentation
title: Install RethinkDB on Debian
title_image: /assets/images/docs/install-platforms/debian.png
active: docs
docs_active: install
permalink: docs/install/debian/
---
{% include install-docs-header.md %}
{% include install-community-platform-warning.md %}
The following instructions were tested on Debian 7 (Wheezy).
If you are using Debian 6, the process is very similar, but you will have to
upgrade a few libraries, such as protobuf.  We will try to document the
necessary library upgrades as soon as possible.

# With Launchpad binaries #

## Add the RethinkDB PPA ##
This approach is inspired by [Anant
Shrivastava](http://blog.anantshri.info/howto-add-ppa-in-debian/)'s
instructions for adding PPAs to Debian systems.

```bash
wget http://blog.anantshri.info/content/uploads/2010/09/add-apt-repository.sh.txt
sudo mv add-apt-repository.sh.txt /usr/sbin/add-apt-repository
sudo chmod o+x /usr/sbin/add-apt-repository
sudo chown root:root /usr/sbin/add-apt-repository
```

## Install RethinkDB ##
```bash
sudo add-apt-repository ppa:rethinkdb/ppa
sudo apt-get update
sudo apt-get install rethinkdb
```

# Compile from source #

## Get the build dependencies ##

```bash
sudo apt-get install g++ protobuf-compiler protobuf-c-compiler libprotobuf-dev \
libprotobuf-c0-dev libboost-dev libssl-dev libv8-dev libboost-program-options-dev \
libgoogle-perftools-dev jsdoc-toolkit libprotoc-dev curl exuberant-ctags m4   \
python-pip git-core libtinfo-dev
```

Debian does not provide Node.js, so it will be built alongside RethinkDB later
in the build process.

Install PyYAML, which is required for building the internal
documentation:

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

{% include install-next-step.md %}
