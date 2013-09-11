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
# Compile from source #

## Get the dependencies ##

Install the main dependencies:

```
sudo zypper in git-core gcc gcc-c++ nodejs protobuf-devel v8-devel gperftools-devel make python-pip boost-devel
```

Install PyYAML, which is required for building the internal documentation:

```
sudo pip install pyyaml
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
./configure tcmalloc_minimal=-ltcmalloc_minimal
make
```

{% include install-next-step.md %}
