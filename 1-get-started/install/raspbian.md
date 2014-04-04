---
layout: documentation
title: Install RethinkDB on Rapsbian
title_image: /assets/images/docs/install-platforms/raspbian.png
active: docs
docs_active: install
permalink: docs/install/raspbian/
---
{% include install-docs-header.md %}
{% include install-community-platform-warning.md %}
The following instructions were tested on Rasbian January 2014 (Debian Wheezy

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
lib64ncurses5-dev libgoogle-perftools-dev libboost-dev curl
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
./configure --without-tcmalloc --allow-fetch
nano external/v8_3.22.24.17/third_party/icu/make
```
In the make file, remove all the text -m32 that you can find in the first part of the document

```bash
make
```

_Note_: Make sure you have more than 1GB of swap space available

{% include install-next-step.md %}
