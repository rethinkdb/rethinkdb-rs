---
layout: documentation
title: Install RethinkDB on Windows
title_image: /assets/images/docs/install-platforms/windows.png
docs_active: install
permalink: docs/install/windows/
---
{% include docs/install-docs-header.md %}

# Downloading #

_Prerequisites:_ We provide native binaries for Windows 7 and above.

[Download](https://download.rethinkdb.com/windows/rethinkdb-dev-preview-0.zip) the ZIP archive and unpack it in a directory of your choice.

{% infobox alert %}
The Windows port of RethinkDB is still in beta, and is not considered production ready yet!
{% endinfobox %}


# Running RethinkDB #

The Windows version of RethinkDB, like the Linux/OS X versions, is executed from the command line. You'll need to start the Windows command shell.

* Press `Win` + `X` and click "Command Prompt"; or
* Open the Start Menu, click "Run," and type "cmd" `ENTER`

Use the `cd` command to go to the directory that you unpacked `rethinkdb.exe` in.

    C:\Users\Slava\>cd RethinkDB
    C:\Users\Slava\RethinkDB\>

Then, you can start RethinkDB with its default options.

    C:\Users\Slava\RethinkDB\>rethinkdb.exe

You can also use any of the [command line options][cl] to control configuration (as well as specify a [configuration file][cf]).

[cl]: /docs/cli-options/
[cf]: /docs/config-file/

To start with a specific data directory:

    rethinkdb.exe -d c:\RethinkDB\data\

To specify a server name and another cluster to join:

    rethinkdb.exe -n jarvis --j cluster.example.com

# Compile from source #

Since RethinkDB on Windows is still in beta, the build process may change with each beta release. For information on how to build the current release, you'll need to check out the [`atnnn/windows2`][aw] branch and follow the instructions in [`WINDOWS.md`][readme].

[aw]: https://github.com/rethinkdb/rethinkdb/tree/atnnn/windows2
[readme]: https://github.com/rethinkdb/rethinkdb/blob/atnnn/windows2/WINDOWS.md

{% include docs/install-next-step.md %}
