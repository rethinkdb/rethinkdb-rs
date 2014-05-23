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
libboost-dev curl
```

## Prepare the raspberrypi ##

Building RethinkDB require more memory than what is available on a Raspberry Pi.
Make sure you have a SWAP partition of at least 1GB.

Make also sure that you have at least 1GB available on your SD card.


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
make
```

The binary will be in `build/release_notcmalloc/`.


{% include install-next-step.md %}
