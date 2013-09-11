---
layout: documentation
title: Install RethinkDB on Arch Linux
title_image: /assets/images/docs/install-platforms/arch.png
active: docs
docs_active: install
permalink: docs/install/arch/
---
{% include install-docs-header.md %}
{% include install-community-platform-warning.md %}
# With AUR #
Using yaourt, you need to run this command: 

```bash
yaourt -S rethinkdb
```

See [the Arch wiki article on RethinkDB](https://wiki.archlinux.org/index.php/RethinkDB) for more information.

# Compile from source #

## Get the build dependencies ##
To compile RethinkDB from source, you will need to install the following packages:

```bash
sudo pacman -S make gcc boost-libs protobuf boost python2 libunwind gperftools java-runtime nodejs protobuf
```

You will also need an older version of the v8 library (see [this Github
issue](https://github.com/rethinkdb/rethinkdb/issues/1195) to track progress).  
Use the Arch Rollback Machine to get the package:

```
wget http://arm.konnichi.com/2013/05/15/community/os/x86_64/v8-3.16.4.1-3-x86_64.pkg.tar.xz
```

Then install it with:

```
sudo pacman -U v8-3.16.4.1-3-x86_64.pkg.tar.xz
```

_Note:_ If you use a different architecture than x86_64, get the appropriate
package from the Arch Rollback Machine.

## Tweak the system ##

RethinkDB uses Python 2 to build some documentation. You can use `sed`
to replace `python` with `python2`, or just change the symbolic link in
`/usr/bin/python` to `/usb/bin/python2` using:

{% infobox %}
__Warning:__ This command will break all other applications that require
`/usr/bin/python` to be Python 3. Use at your own risk.
{% endinfobox %}

```bash
sudo ln -s /usr/bin/python2 /usr/bin/python
```
## Build RethinkDB ##
Kick off the build process:

```bash
cd rethinkdb
./configure PROTOBUF=/usr/lib/libprotobuf.so
make
```

You will find the `rethinkdb` binary in the `build/release/` subfolder.  

{% include install-next-step.md %}
