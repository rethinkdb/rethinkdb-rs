---
layout: documentation
title: Crash recovery
active: docs
docs_active: crashes
permalink: docs/crashes/
---

While we strive to give you perfect uptime, like other complex applications RethinkDB is not immune to crashing. Here are some tips on how to recover from a crash, how to submit a bug report, and how to maximize availability.

# What to do after a crash

## Check if you ran out of memory

You may be able to check if the kernel's out-of-memory killer is responsible for the crash by checking the system message buffer:

```
sudo dmesg | grep oom
```

This may show you messages similar to this:

```
rethinkdb invoked oom-killer: gfp_mask=0x201da, order=0, oom_adj=0, oom_score_adj=0
 [<ffffffff8111d272>] ? oom_kill_process+0x82/0x2a0
```

If this is the case, you may be able to avoid crashes by changing RethinkDB's cache size. For information on in-memory caches, how to check their current size, and how to change them, read [Understanding RethinkDB memory requirements](/docs/memory-usage).

## Check the log

The log file's location is dependent on your system configuration and how you started RethinkDB.

* If you started `rethinkdb` on a terminal rather than from a startup script, it will log to the `rethinkdb_data` directory. By default it will write to `log_file` but this may be overridden with the `--log-file` startup option.

* If your Linux system uses `systemd`, use `journalctl` to view the log:

    `journalctl -u rethinkdb@<instance>`

* If you installed RethinkDB through a package manager on a system that does *not* use `systemd`, then you may have to check where it's configured to log. It's very likely this will be in the `/var/log/` directory (i.e., `/var/log/rethinkdb`).

The log may give you information as to what caused the crash.

## Community support

If it doesn't appear to be a memory issue and the log provides no clue, you can try asking for support on our official IRC channel, [#rethinkdb on freenode](http://webchat.freenode.net/?channels=#rethinkdb) or our [Google Group](http://groups.google.com/group/rethinkdb). If your problem is a crash that we've seen before&mdash;or our users have&mdash;this may get you a quick answer.

# How to submit a bug report

We use Github for issue tracking: <https://github.com/rethinkdb/rethinkdb/issues>. If you want to report a suspected bug in RethinkDB, open an issue there.

The most important things for you to provide for us are:

* The full output from `rethinkdb --version`, something like:

    ```
    rethinkdb 1.13.3 (CLANG 5.1 (clang-503.0.40))
    ```

* The full output from `uname -a`, something like:

    ```
    Darwin rethink.local 13.3.0 Darwin Kernel Version 13.3.0:
    Tue Jun  3 21:27:35 PDT 2014; root:xnu-2422.110.17~1/RELEASE_X86_64 x86_64
    ```

* The backtrace from the crash, if it's available in the logs.

Other things that might be helpful to us, if you have them:

* A dump of the [system tables](/docs/system-tables/) (see below)
* The core file, if it was dumped on crash
* The data files if RethinkDB cannot restart&sup1;
* The output of `rethinkdb` on startup
* Your cluster configuration (number of servers, basic network topology, etc.)
* Information about the server:
    * How much memory it has
    * The file system it's using
    * Are you running RethinkDB in a VM?
    * Other unusual configuration details
* Is the crash reproducible, and if so, under what conditions?

# Dumping the system tables

In the Data Explorer, the following command will output the contents of all the configuration/status tables and the most recent 50 lines of the `logs` table:

```js
r.expr(["current_issues", "jobs", "stats", "server_config", "server_status",
"table_config", "table_status", "db_config", "cluster_config"]).map(
    [r.row, r.db('rethinkdb').table(r.row).coerceTo('array')]
).coerceTo('object').merge(
    {logs: r.db('rethinkdb').table('logs').limit(50).coerceTo('array')}
)
```

# Setting up high availability

RethinkDB supports replication of data: every table in a database can be replicated as many times as you have servers in a cluster. Setting up replication is a simple operation with the web interface or the command line tool. For details, read [Sharding and replication](/docs/sharding-and-replication/).

RethinkDB does not have fully automatic failover (yet), but if a server in a cluster crashes it can be manually removed from the cluster. In most cases, RethinkDB will recover from such a situation automatically. For information on this process, read [Failover](/docs/failover).

----

1. We'll sign an NDA if necessary, and can set up an FTP server for you to transfer the file to if it's large.
