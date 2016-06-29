---
layout: documentation
title: RethinkDB command line options
docs_active: cli-options
permalink: /docs/cli-options/
---

These options can be passed to the `rethinkdb` server on the command line on startup. Many of these options can also be set in the configuration file; read [Configuration file options](/docs/config-file/) for a detailed list.

## File path options ##

* `-d [ --directory ] path`: specify directory to store data and metadata
* `--io-threads n`: how many simultaneous I/O operations can happen at the same time
* `--direct-io`: use direct I/O for file access
* `--cache-size mb`: total cache size (in megabytes) for the process. Can be 'auto'.

## Server name options ##

* `-n [ --server-name ] arg`: the name for this server (as will appear in the metadata).  If not specified, it will be randomly chosen from a short list of names.
* `-t [ --server-tag ] arg`: a tag for this server. Can be specified multiple times.

## Network options ##

* `--bind {all | addr}`: add the address of a local interface to listen on when accepting connections; loopback addresses are enabled by default
* `--bind-http {all | addr}`: bind the web administration UI port to a specific address
* `--bind-cluster {all | addr}`: bind the cluster connection port to a specific address
* `--bind-driver {all | addr}`: bind the client driver to a specific address
* `--no-default-bind`: disable automatic listening on loopback addresses, unless explicitly specified in a separate `--bind` option
* `--cluster-port port`: port for receiving connections from other nodes
* `--driver-port port`: port for RethinkDB protocol client drivers
* `-o [ --port-offset ] offset`: all ports used locally will have this value added
* `-j [ --join ] host:port`: host and port of a RethinkDB node to connect to
* `--reql-http-proxy [protocol://]host[:port]`: HTTP proxy to use for performing `r.http(...)` queries, default port is 1080
* `--canonical-address addr`: address that other RethinkDB instances will use to connect to us, can be specified multiple times
* `--cluster-reconnect-timeout secs`: the amount of time, in seconds, this server will try to reconnect to a cluster if it loses connection before giving up; default `86400`

The `--bind` option controls the default behavior for all RethinkDB ports. If it's specified, the `--bind-http`, `--bind-cluster` and `--bind-driver` options will override that behavior for a specific port. So:

    rethinkdb --bind all --bind-cluster 192.168.0.1

This will bind the HTTP and driver ports on all available interfaces, while the cluster port will only be bound on the loopback interface and `192.168.0.1`.

## TLS options ##

* `--http-tls-key key_filename`: private key to use for web administration console TLS
* `--http-tls-cert cert_filename`: certificate to use for web administration console TLS

__Note:__ `--http-tls-key` and `--http-tls-cert` must be used together.

* `--driver-tls-key key_filename`: private key to use for client driver connection TLS
* `--driver-tls-cert cert_filename`: certificate to use for client driver connection TLS
* `--driver-tls-ca ca_filename`: CA certificate bundle used to verify client certificates; TLS client authentication disabled if omitted

__Note:__ `--driver-tls-key` and `--driver-tls-cert` must be used together; `--driver-tls-ca` is optional.

* `--cluster-tls-key key_filename`: private key to use for intra-cluster connection TLS
* `--cluster-tls-cert cert_filename`: certificate to use for intra-cluster connection TLS
* `--cluster-tls-ca ca_filename`: CA certificate bundle used to verify cluster peer certificates

__Note:__ all three `--cluster-tls-*` options must be used together.

* `--tls-min-protocol protocol`: the minimum TLS protocol version the server accepts, one of `TLSv1`, `TLSv1.1`, `TLSv1.2`; default is `TLSv1.2`
* `--tls-ciphers cipher_list`: specify a list of TLS ciphers to use; default is `EECDH+AESGCM`
* `--tls-ecdh-curve curve_name`: specify a named elliptic curve to use for ECDHE; default is `prime256v1`
* `--tls-dhparams dhparams_filename`: provide parameters for DHE key agreement; REQUIRED if using DHE cipher suites; at least 2048-bit recommended

For details about these options, read [Securing your cluster][sec].

[sec]: /docs/security/

## Web options ##

* `--web-static-directory directory`: the directory containing web resources for the http interface
* `--http-port port`: port for web administration console
* `--no-http-admin`: disable web administration console

## CPU options ##

* `-c [ --cores ] n`: the number of cores to use

## Service options ##

* `--pid-file path`: a file in which to write the process id when the process is running
* `--daemon`: daemonize this rethinkdb process

## Set User/Group options ##

* `--runuser user`: run as the specified user
* `--rungroup group`: run with the specified group

## Security options ##

* `--initial-password`: set a password for the `admin` user if none has previously been set; use `auto` to choose a random password that will be printed to `stdout` (see [Secure your cluster][sec] for more information)

[sec]: /docs/security

## Help options ##

* `-h [ --help ]`: print this help
* `-v [ --version ]`: print the version number of rethinkdb

## Log options ##

* `--log-file file`: specify the file to log to, defaults to 'log_file'
* `--no-update-check`: disable checking for available updates.  Also turns off anonymous usage data collection.

## Configuration file options ##

* `--config-file`: take options from a configuration file

## Subcommands ##

* `rethinkdb create`: prepare files on disk for a new server instance
* `rethinkdb serve`: use an existing data directory to host data and serve queries
* `rethinkdb proxy`: run a proxy server (serve queries from an existing cluster but don't host data)
* `rethinkdb export`: export data from an existing cluster into a file or directory
* `rethinkdb import`: import data from from a file or directory into an existing cluster
* `rethinkdb dump`: export and compress data from an existing cluster
* `rethinkdb restore`: import compressed data into an existing cluster
* `rethinkdb index-rebuild`: rebuild outdated secondary indexes

For more information about subcommands, type `rethinkdb help [subcommand]` at the command line.

For information about running RethinkDB as a proxy server, read [Running a proxy node](/docs/sharding-and-replication/#running-a-proxy-node) in "Scaling, sharding and replication."
