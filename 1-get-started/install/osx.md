---
layout: documentation
title: Install RethinkDB on OS X
title_image: /assets/images/docs/install-platforms/osx.png
active: docs
docs_active: install
permalink: docs/install/osx/
---
{% include install-docs-header.md %}

# Via an installer #

_Prerequisites:_ We provide native binaries for OS X Lion and above (>= 10.7).

[Download](http://download.rethinkdb.com/osx/rethinkdb-{{site.version.major}}.{{site.version.minor}}.dmg) the disk
image, run `rethinkdb.pkg`, and follow the installation instructions.

_Troubleshooting:_ If an alert dialog shows up and says _"rethinkdb.pkg can’t be opened
because it is from an unidentified developer"_, right click on the file and choose open.
You will be given the option to install RethinkDB even if the package is not signed.  
See [Github issue 1565](https://github.com/rethinkdb/rethinkdb/issues/1565) to track
progress.

# Via Homebrew #

_Prerequisites:_ Make sure you're on OS X Lion or above (>= 10.7) and
have [Homebrew](http://mxcl.github.com/homebrew/) installed.

Run the following in your terminal:

```bash
brew update && brew install rethinkdb
```
# Compile from source #

## Get the build dependencies ##

There are a number of packages required for the build process. On OS X,
[Xcode](https://developer.apple.com/xcode/) is required to build from source.

You will also need to install boost with
[Homebrew](http://mxcl.github.com/homebrew/):

```bash
brew install boost
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
./configure --fetch protobuf --fetch v8
make
```

You will find the `rethinkdb` binary in the `build/release/` subfolder.  

{% include install-next-step.md %}
