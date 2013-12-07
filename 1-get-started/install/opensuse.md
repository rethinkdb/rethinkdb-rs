---
layout: documentation
title: Install RethinkDB on openSUSE
title_image: /assets/images/docs/install-platforms/opensuse.png
active: docs
docs_active: install
permalink: docs/install/opensuse/
---
{% include install-docs-header.md %}
{% include install-community-platform-warning.md %}

These instructions have been tested on OpenSuse 13.1.

# Compile from source #

## Get the dependencies ##

Install the main dependencies:
```
sudo zypper in git-core make gcc gcc-c++ protobuf-devel nodejs v8-devel ncurses-devel gperftools-devel boost-devel
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
./configure --dynamic tcmalloc_minimal
make
```

{% include install-next-step.md %}
