---
layout: documentation
title: Configuration file options
docs_active: config-file
permalink: docs/config-file/
---

The `.conf` file includes a number of options exclusively for the
init script. The rest of the options are exactly the same as the ones
that go on the command line to the RethinkDB server. For more details
about these options run `rethinkdb help`.

The configuration file's location depends on the startup system your distribution uses. A configuration file may also be specified on the command line with the `--config-file` option.

## Format ##

A [sample `.conf` file][conf] is available with full comments. (It may already be installed on your distribution as `default.conf.sample`.)

[conf]: https://github.com/rethinkdb/rethinkdb/blob/next/packaging/assets/config/default.conf.sample

The file uses a simple format of `key=value`, with one key specified per line. A simple configuration file that uses the default ports, assigns a server to a virtual group using server tags and joins an existing cluster might be:

```
server-name=Kismet
server-tag=default
server-tag=fremont_ca
join=layered:29015
daemon
```

## Supported options ##

For some options below, the default value depends on `<name>`, the name of the
config file without the `.conf` extension.

* `runuser` and `rungroup`: specifies which
  user and group should be used launch the Rethinkdb process.   
  *Default*: `rethinkdb` and `rethinkdb`.

* `pid-file`: the location of the file with the RethinkDB instance process ID (used by the init script to communicate with
  the server process).   
  *Default*: `/var/run/rethinkdb/<name>/pid_file` 

* `directory`: the data directory where
  database tables will be stored. This location must be readable and
  writable by the user or group (or both) specified by `runuser`
  and `rungroup`.   
  _Note_: It is best to create the database manually via
  `rethinkdb create --directory ...` as `runuser` or `rungroup` before
  enabling auto-start.  
  *Default*: `/var/lib/rethinkdb/<name>/`

* `log-file`: path to the log file.  
  *Default*: `<directory>/log_file`

* `bind`: Address of local interfaces to listen on when accepting connections.
   May be 'all' or an IP address, loopback addresses are enabled by default.  
   *Default*: all local addresses

* `bind-http`: Similar to `bind`, but only for the web UI connection port. This option will override `bind` for this port if both are specified in the configuration file.

* `bind-cluster`: Similar to `bind`, but only for the cluster connection port. This option will override `bind` for this port if both are specified in the configuration file.

* `bind-driver`: Similar to `bind`, but only for the client driver connection port. This option will override `bind` for this port if both are specified in the configuration file.

* `http-tls-key`: the filename of a private key to use with TLS for the web administration console. Both `http-tls-key` and `http-tls-cert` must be specified.

* `http-tls-cert`: the filename of a TLS certificate to use for the web administration console. Both `http-tls-key` and `http-tls-cert` must be specified.

* `driver-tls-key`: the filename of a private key to use with TLS for client driver connections. Both `driver-tls-key` and `driver-tls-cert` must be specified.

* `driver-tls-cert`: the filename of a TLS certificate to use for client driver connections. Both `driver-tls-key` and `driver-tls-cert` must be specified.

* `driver-tls-ca`: the filename of a CA certificate bundle to use for verifying client driver connections. If specified, the server will only accept connections from clients that provide a certificate signed with the CA certificate.

* `cluster-tls-key`: the filename of a private key to use with TLS for cluster connections. All three `cluster-tls-*` configurations must be specified.

* `cluster-tls-cert`: the filename of a TLS certificate to use for cluster connections. All three `cluster-tls-*` configurations must be specified.

* `cluster-tls-ca`: the filename of a CA certificate to use for verifying cluster connections. All three `cluster-tls-*` configurations must be specified.

* `tls-ciphers`: A list of TLS ciphers to use.  
  *Default*: `EECDH+AESGCM`

* `tls-ecdh-curve`: A named elliptic curve to use for ECDHE.  
  *Default*: `prime256v1`
  
* `tls-dhparams`: A filename containing parameters for DHE key agreement; this is required if using DHE cipher suites, and unused otherwise. At least a 2048-bit key is recommended.

* `canonical-address`: Address that other rethinkdb instances will use to connect to this machine.
  It can be specified multiple times.

* `http-port`, `driver-port`, and `cluster-port`: the web UI
  port (default `8080`), the client driver port (default
  `28015`), and intracluster traffic port (default `29015`),
  respectively.

* `join`: The `host:port` of a node that Rethinkdb will connect to.
  It can be specified multiple times.

* `port-offset` All ports used locally will have this value added.  
  *Default*: 0
  
* `no-http-admin`: Disable web administration console.

* `cores`: Number of cores to use.  
  *Default*: Number of cores of the CPU.

* `cache-size`: Size of the cache in MB.  
  *Default*: Half of the available RAM on startup.

* `io-threads`: Number of simultaneous I/O operations can happen at the same time.  
  *Default*: 64

* `direct-io`: Use direct I/O for file system access. 

* `server-name`: The name for this machine (as it will appear in the metadata).  
  *Default*: Randomly chosen from a short list of names.

* `server-tag`: Specifies tags for this server, which can be used to group servers together for administration purposes (for instance, servers in the same data center). See [Sharding and replication][sar] for more details. To assign multiple tags to a server, repeat `server-tag` lines for each tag.

[sar]: /docs/sharding-and-replication/
