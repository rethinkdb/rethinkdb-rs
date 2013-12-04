---
layout: documentation
title: Install RethinkDB on Linux Mint
title_image: /assets/images/docs/install-platforms/mint.png
active: docs
docs_active: install
permalink: docs/install/mint/
---
{% include install-docs-header.md %}
{% include install-community-platform-warning.md %}
The following instructions were tested on Linux Mint 16.

# With binaries #
Add the [RethinkDB PPA](https://launchpad.net/~rethinkdb/+archive/ppa) to your list of
repositories and then install via `apt-get`:  

```bash
sudo add-apt-repository ppa:rethinkdb/ppa   && \
sudo apt-get update                         && \
sudo apt-get install rethinkdb
```


# Compile from source #

## Get the dependencies ##
Install the main dependencies:

```
sudo apt-get install git-core g++ nodejs npm libprotobuf-dev libncurses5-dev \
    libgoogle-perftools-dev libboost-dev nodejs-legacy
```

## Get the source code ##
Clone the RethinkDB repository:

```
git clone --depth 1 -b v{{site.version.major}}.x https://github.com/rethinkdb/rethinkdb.git
```

## Build RethinkDB ##

Kick off the build process:

```
cd rethinkdb
./configure
make
```

{% include install-next-step.md %}
