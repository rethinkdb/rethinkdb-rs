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
upgrade a few libraries, such as protobuf.

# Using the Ubuntu packages on Debian #

Add the [RethinkDB repository](http://download.rethinkdb.com/apt) to
your list of repositories and then install via `apt-get`:

```bash
echo "deb http://download.rethinkdb.com/apt `lsb_release -cs` main" | sudo tee /etc/apt/sources.list.d/rethinkdb.list
wget -qO- http://download.rethinkdb.com/apt/pubkey.gpg | sudo apt-key add -
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
./configure --fetch npm
make
```

_Note_: If you have `nodejs` and `npm` installed, you do not need the `--fetch npm` argument.

{% include install-next-step.md %}
