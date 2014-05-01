---
layout: documentation
title: Install RethinkDB on Raspbian
title_image: /assets/images/docs/install-platforms/raspbian.png
active: docs
docs_active: install
permalink: docs/install/raspbian/
---
{% include install-docs-header.md %}
{% include install-community-platform-warning.md %}
The following instructions were tested on Raspbian January 2014 (Debian Wheezy)

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
