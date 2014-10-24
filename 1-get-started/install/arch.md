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

# Install binary packages with `pacman` #
RethinkDB is in the `community` repository. To install the server, run:

```bash
# pacman -S rethinkdb
```

See [the ArchWiki article on RethinkDB](https://wiki.archlinux.org/index.php/RethinkDB) for more information.


# Install with Arch Build System #
RethinkDB can be compiled automatically by the Arch Build System, the ports-like system for building and packaging 
software from source code in Arch Linux. Note that ABS may lag slightly behind the Arch binary repositories.

{% infobox %}
According to the `PKGBUILD`, 'some tests might be flaky on btrfs'. If you use Btrfs and you are unable to build 
RethinkDB due to failed tests, you may be tempted to edit the `PKGBUILD` so as to skip the test suite. This 
might, of course, expose you to security problems in production. Workarounds would include: use binary packages as 
above, use another filesystem, or edit the failing test to tolerate Btrfs's unusual behaviour.  
{% endinfobox %}

## `yaourt` shortcut ##
If you have `yaourt` installed, simply run:

```bash
$ yaourt -Sb rethinkdb
```

This will automatically download the `PKGBUILD` script provided by ABS, download and extract the RethinkDB source,
compile and test RethinkDB, create a `pacman`-compatible package, and install the package on the local system. Full 
customization is possible by editing the `PKGBUILD` when prompted.

## Semi-manual build ##
Ensure you have the `abs` package installed, `/etc/abs.conf` configured for the `community` repository (see 
[the ArchWiki article on ABS](https://wiki.archlinux.org/index.php/Arch_Build_System#How_to_use_ABS)), and 
`/etc/makepkg.conf` configured to your liking (see 
[the ArchWiki article on makepkg](https://wiki.archlinux.org/index.php/Makepkg)).

Copy the `PKGBUILD` and related files to a working directory (here `~/rethinkdb`):

```bash
$ sudo abs community/rethinkdb
$ cp -r /var/abs/community/rethinkdb/ ~
```

Full customization is possible by editing the `PKGBUILD` at this point.

Now you can install the build dependencies, build and install the package (the `-s` flag causes `makepkg` to attempt to 
install *explicit* build dependencies):

```bash
# pacman -S base-devel
$ cd ~/rethinkdb
$ makepkg -s
# pacman -U rethinkdb-1.15.1-1-x86_64.pkg.tar.xz
```


# Build from official source #

## Install build dependencies  ##
You will need to install the `base-devel` group and several additional build dependencies:

```bash
# pacman -S base-devel protobuf boost python2 gperftools v8 nodejs
```

## Get the source code ##
Clone the RethinkDB repository:

```bash
$ git clone --depth 1 -b v{{site.version.major}}.x https://github.com/rethinkdb/rethinkdb.git
```

## Build RethinkDB ##
{% infobox %}
RethinkDB's `configure` script assumes that the `python` executable will be Python 2 (i.e., that `/usr/bin/python` is
symlinked to `/usr/bin/python2`), which will break your build ('python 3.4.2 is too recent' etc.). Rather than 
rewriting this symlink and potentially breaking other software, you might install the following script as 
`/usr/local/bin/python` (replace `/home/user/rethinkdb` with the absolute path of your own working directory):

```bash
#!/bin/bash
script=$(readlink -f -- "$1")
case "$script" in (/home/user/rethinkdb)
    exec python2 "$@"
    ;;
esac

exec python3 "$@"
```

This will redirect calls to `python` originating from your RethinkDB working directory to `python2` and leave others 
untouched. Ensure that the script is executable (`chmod +x`) and, if necessary, reload your shell before continuing.
{% endinfobox %}

To run the build:

```bash
$ cd ~/rethinkdb
$ ./configure --dynamic tcmalloc_minimal
$ make
```

Once successfully built, the `rethinkdb` binary may be found in the `build/release/` subdirectory.  

To install RethinkDB globally:

```bash
$ cd ~/rethinkdb
# make install
```

{% include install-next-step.md %}