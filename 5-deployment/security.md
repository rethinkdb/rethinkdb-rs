---
layout: documentation
title: Secure your cluster
docs_active: security
permalink: docs/security/
---

<img alt="Securing your cluster Illustration"
     class="api_command_illustration"
     src="/assets/images/docs/api_illustrations/secure-cluster.png" />

{% toctag %}

The best way to secure a RethinkDB cluster is to run it on a protected
network that doesn't allow access from the outside world. However, this may
not always be feasible. For example, cloud deployments often require access
from wide area networks.

There are two main methods RethinkDB provides for securing the cluster: TLS encryption for connections, and binding the ports the server uses to specific IP addresses to limit outside connections.

# Using TLS #

Starting with version 2.3, RethinkDB offers the ability to secure connections between servers, between servers and clients, and to the web UI using [TLS][] encryption (the successor to SSL). Securing RethinkDB in this fashion is similar to securing a web site with a [self-signed SSL certificate][ssc]: create a private key and a certificate, then tell the server to use them.

[TLS]: https://en.wikipedia.org/wiki/Transport_Layer_Security
[ssc]: https://en.wikipedia.org/wiki/Self-signed_certificate

## Generate a key and matching certificate ##

The easiest way to do this is with the `openssl` command line tool. (Under Linux and OS X, this is already installed; for Windows, you may be able to find [precompiled binaries][win] from the list on the OpenSSL wiki.)

[win]: https://wiki.openssl.org/index.php/Binaries

First, generate a 2048-bit key and save it to `key.pem`:

    openssl genrsa -out key.pem 2048

Then, generate a certificate, `cert.pem`, from that key:

    openssl req -new -x509 -key key.pem -out cert.pem -days 3650

OpenSSL will ask you to enter information for the certificate. While some of these questions can be left at their default, the "Common Name" must match the domain name of your server. For local testing purposes you can use `localhost`, but not in production.

    Country Name (2 letter code) [AU]:US
    State or Province Name (full name) [Some-State]:California
    Locality Name (eg, city) []:Mountain View
    Organization Name (eg, company) [Internet Widgits Pty Ltd]:RethinkDB
    Organizational Unit Name (eg, section) []:
    Common Name (e.g. server FQDN or YOUR name) []:example.com
    Email Address []:

## Telling RethinkDB to use your certificate ##

Certificates and keys may be specified with either [command line options][cli] or as keys in a [configuration file][cfg]. To start RethinkDB and tell it to secure the web administration UI, you can start the server with the following options:

    rethinkdb --http-tls-key key.pem --http-tls-cert cert.pem

Both options are required. In the configuration file, you would specify the key and certificate like so:

    http-tls-key=key.pem
    http-tls-cert=cert.pem

To use TLS connections with client drivers, specify:

    rethinkdb --driver-tls-key key.pem --driver-tls-cert cert.pem

And to use TLS connections between servers in the cluster:

    rethindb --cluster-tls-key key.pem --cluster-tls-cert cert.pem --cluster-tls-ca cert.pem

Note that in the last case, you're required to provide a CA certificate as well. This is a certificate used to sign other certificates. In this case, we're using the same certificate for both, but we could sign our `cert.pem` with a different CA certificate and specify both of them. Servers can only connect to the cluster if the certificates specified by their `cluster-tls-cert` value are signed by the CA certificate specified by `cluster-tls-ca`.

[cfg]: /docs/config-file/
[cli]: /docs/cli-options/

# Binding the web interface port #

Binding the web interface port can prevent it from being accessed
directly from a remote machine. You can bind it to a specific IP address using the `--bind-http` [command line option][cli]; the most secure method is to bind it to the local machine (`localhost`) and then connect via a proxy.

    rethinkdb --bind-http localhost

(You can also specify `bind-http=` in the [configuration file][cfg].)

Now, use one of the following two methods to enable secure access.

## Via a SOCKS proxy ##

Once you block the web interface port in the step above, the easiest
way to access it is to use ssh to set up a SOCKS proxy. Run the
following command on your local server (not the one running
RethinkDB):

```bash
ssh -D 3000 USERNAME@HOST
```

Where,

- `HOST` is the ip of any server on your RethinkDB cluster.
- `3000` can be changed to any port that is available on your local
  server.

Then open your browser:

- __If you're using Chrome__, go to _Settings > Advanced settings >
  Network > Change proxy settings_, and set the _Network proxy_ option
  to manual mode with the following settings:
  - Host: `localhost`
  - Port: `3000`
  - Ignored host: (remove everything)

- __If you are using Firefox__, go to _Edit > Preferences_. Then click
  on _Advanced > Network > Settings_ and create a manual proxy
  configuration with these settings:
  - Socks host: `localhost`
  - Port: `3000`
  - Check socks v5
  - No proxy for: (remove everything)

You can now visit `localhost:8080` to see the RethinkDB web admin.

## Via a reverse proxy ##

You can use a reverse HTTP proxy to allow access to the web interface
from other servers. Most web servers (such as Apache or Nginx)
support this feature. In the following example we'll use Apache to set
up a reverse proxy.

You'll need the following modules installed for Apache:

* proxy
* proxy_http

Depending on your OS, you may need to install a library such as `libapache2-mod-proxy-html`.

Create a new virtual host:

```
<VirtualHost *:80>
    ServerName domain.net

    ProxyRequests Off

    <Proxy *>
        Order deny,allow
        Allow from all
        AuthType Basic
        AuthName "Password Required"
        AuthUserFile password.file
        AuthGroupFile group.file
        Require group dbadmin
    </Proxy>

    ProxyErrorOverride On
    ProxyPass   /rethinkdb_admin/   http://localhost:8080/
    ProxyPassReverse   /rethinkdb_admin/   http://localhost:8080/

</VirtualHost>
```

Create the password file in `/etc/apache2/`:

```
htpasswd.exe -c password.file username
```

Almost done. All we have to do now is create a file `group.file` with
this the following content:

```
dbadmin: username
```

You can now access the web interface using the following URL:
`http://HOST/rethinkdb_admin`.

# Securing the driver port #

## Using the RethinkDB authentication system ##

Clients may specify `user` and `password` values in the [connect](/api/javascript/connect) command. For more information about creating and managing user accounts and permissions, read [Permissions and user accounts][pua].

[pua]: /docs/permissions-and-accounts/

Note that passwords will be sent in plaintext unless you are using TLS encryption. Instead of (or in addition to) TLS, you can bind the driver port and use SSH tunneling, as described below.

{% infobox %}
__Note__: The authentication system affects _client drivers,_ not the web interface. Follow the directions above to secure the web UI.
{% endinfobox %}

## Using SSH tunneling ##

First, protect the driver port so that it cannot be accessed from the
outside world. Use the `--bind-driver` [command line option][cli] or the corresponding [configuration file option][cfg] to bind it to `localhost`.

    rethinkdb --bind-driver localhost

Now create an SSH tunnel on the server that needs to access the
remote RethinkDB driver port:

```bash
ssh -L <local_port>:localhost:<driver_port> <ip_of_rethinkdb_server>
```

Where,

- `local_port` is the port you are going to specify in the driver - It
  can be any available port on your server.
- `driver_port` is the RethinkDB driver port (28015 by default).
- `ip_of_rethinkdb_server` is the IP address of the server that runs
  the RethinkDB server.

You can now connect to your RethinkDB instance by connecting to the
host `localhost` and port `local_port`:

```javascript
r.connect({host: 'localhost', port: <local_port>},
    function(error, connection) { ... })
```

# Binding the intracluster port #

To secure the cluster port, bind it to a specific IP address using the `--bind-cluster` [command line option][cli] or the corresponding [configuration file option][cfg]. Bind it to an IP address that is only accessible from within your local network.

    rethinkdb --bind-cluster 192.168.0.100

The intracluster port will be accessible from within the local network
where you run RethinkDB nodes, but will not be accessible from the
outside world.
