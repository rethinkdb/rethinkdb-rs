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

### SWAP ###

Building RethinkDB requires more RAM than what the raspberrypi can provide. To build
from source, you must have a SWAP partition of at least 1GB.


If you want to create a SWAP partition on a USB key on `/dev/sda`, you can run:

```
sudo fdisk /dev/sda
```

Then hit `n` to create a new partition, `p` to set it as primary, and write `+1G` to set the size
of the swap to 1GB.

Change the type of the new partition. Hit `t`, select your partition, and enter `82` for `Linux swap/Solaris`.

Activate the swap.

```
sudo mkswap /dev/sda1
sudo swapon /dev/sda1
```

### Disk ###

The default root partition does not have enough space to build RethinkDB. Make sure to create a
bigger partition with at least 1GB available.


For example, if you want to create a partition on a USB key on `/dev/sda`

```
sudo fdisk /dev/sda
```

Then hit `n` to create a new partition, `p` to set it as primary, and write `+1G` to set the size
of the partition.

Format the partition

```
sudo mkfs.ext4 /dev/sda1
sudo mkdir /mnt/usbkey
sudo mount /dev/sda1 /mnt/usbkey
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
make
```

The binary will be in `build/release_notcmalloc/`.


{% include install-next-step.md %}
