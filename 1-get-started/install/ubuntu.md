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
repository](http://download.rethinkdb.com/apt) to your list of
repositories and install via `apt-get`.
To do this, paste the
following lines into your terminal:

```bash
source /etc/lsb-release && echo "deb http://download.rethinkdb.com/apt $DISTRIB_CODENAME main" | sudo tee /etc/apt/sources.list.d/rethinkdb.list
wget -O- http://download.rethinkdb.com/apt/pubkey.gpg | sudo apt-key add -
sudo apt-get update
sudo apt-get install rethinkdb
```

# Compile from source on Ubuntu 13.10 #

## Get the build dependencies ##
Install the main dependencies:

```
sudo apt-get install git-core g++ nodejs npm libprotobuf-dev libgoogle-perftools-dev \
    libncurses5-dev libboost-all-dev nodejs-legacy
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


# Compile from source on Ubuntu 12.04 #

## Get the build dependencies ##

Install the main dependencies:

```
sudo apt-get install git-core g++ nodejs npm libprotobuf-dev libgoogle-perftools-dev \
    libncurses5-dev libboost-all-dev
```

Then install a more recent version of `node` with `n`.

```
sudo npm install -g n
sudo apt-get install curl
sudo n stable
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
./configure npm=/usr/local/bin/npm
make
```

You will find the `rethinkdb` binary in the `build/release/` subfolder.  


{% include install-next-step.md %}
