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

Install the main dependencies:

```
sudo apt-get install git g++ protobuf-compiler libprotobuf-dev libv8-dev \
libncurses5-dev libgoogle-perftools-dev libboost-dev curl
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
./configure --allow-fetch
make
```

_Note_: If you have `nodejs` and `npm` installed, you do not need the `--fetch npm` argument.

{% include install-next-step.md %}
