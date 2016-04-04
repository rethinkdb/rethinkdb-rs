---
layout: documentation
title: Secure your cluster
docs_active: security
permalink: docs/security/
---

<img alt="Securing your cluster Illustration"
     class="api_command_illustration"
     src="/assets/images/docs/api_illustrations/secure-cluster.png" />

The best way to secure a RethinkDB cluster is to run it on a protected
network that doesn't allow access from the outside world. However,
this may not always be feasible. For example, cloud deployments often
require access from wide area networks.

The following is a list of techniques that help mitigate the risk of
attacks for RethinkDB setups that require access from the outside
world.

# The admin account #

All RethinkDB servers have an `admin` account with full access to the cluster, and by default this account has no password. (For full details on this topic, read [Permissions and user accounts][pa].) One of the first things you should do to secure a cluster is to assign a password to `admin`. You can do this when the first server starts up by using the `--initial-password` [command line option][cli], or by updating the `admin` record with a new password in the user [system table][st].

[pa]:  /docs/permissions-and-accounts/
[cli]: /docs/cli-options/
[st]:  /docs/system-tables/#users

When new servers (including proxies) join an existing cluster, it will synchronize with all the user account information, including passwords, from that cluster. This includes the `admin` account. However, a server with no admin password set cannot join an existing cluster that _does_ have a password set. Otherwise, it would technically be possible for an attacker to connect to that server as an admin and run arbitrary queries in the time between that server starting and the accounts synchronizing from the rest of the cluster.

If you're setting up a cluster in a secure environment (for instance, the whole cluster is on a local network behind a firewall), you can simply start the servers in the cluster without an admin password, then update the `admin` record in the `users` table with a new password. However, if you're joining a new server to a cluster that's already password-protected, the best way to do it is to specify the `--initial-password auto` option.

This option creates a random password for that server's `admin` account. Once that server is synchronized with the cluster, the random password will be overwritten by the `admin` password, preventing the exploit described above.

    rethinkdb --initial-password auto --join cluster

# Securing the web interface #

The web UI always effectively runs as the `admin` user, including queries in the Data Explorer. Therefore, it's important to secure this port so it can't be accessed by an unauthorized remote machine.

You can bind the web UI port to a specific IP address using the `--bind-http` [command line option][cli]; to bind it to the local machine, simply bind it to `localhost`:

    rethinkdb --bind-http localhost

Now, use one of the following two methods to enable secure access.

## Via a socks proxy ##

Once you block the web interface port in the step above, the easiest
way to access it is to use ssh to set up a socks proxy. Run the
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

You can use a reverse http proxy to allow access to the web interface
from other servers. Most web servers (such as apache or nginx)
support this feature. In the following example we'll use apache to set
up a reverse proxy.

First, install apache with relevant modules as follows:

```
sudo apt-get install libapache2-mod-proxy-html
sudo a2enmod proxy
sudo a2enmod proxy_http
```

Then create a new virtual host:

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

RethinkDB allows you to set an authentication key by modifying the
`cluster_config` [system table](/docs/system-tables/). Once you set an
authentication key, client drivers will be required to pass the key to the
server in order to connect.

{% infobox %}
__Note__: The authentication key affects _client drivers,_ not the web interface. Follow the directions above to secure the web UI.
{% endinfobox %}

Open the Data Explorer in the web administration console and execute the following command:

```js
r.db('rethinkdb').table('cluster_config').get('auth').update({auth_key: 'newkey'})
```

Instead of "newkey" you can use any string of your choice as the key.

You can now connect to the driver port from any network, but must provide the
required authentication key with the `connect` command. For instance, in
JavaScript you would connect as follows:

```javascript
r.connect({host: HOST, port: PORT, authKey: <authentication_key>},
    function(error, connection) { ... })
```

You can remove an authentication key by writing `null` to the `auth_key` field in `cluster_config`:

```js
r.db('rethinkdb').table('cluster_config').get('auth').update({auth_key: null})
```

You can use any ReQL driver for this operation, not just the Data Explorer. Read [Administration tools](/docs/administration-tools/) for more details about scripting RethinkDB administration tasks.

{% infobox alert %}
__Note__: the authentication key will be transmitted to and stored on the
RethinkDB server in plain text, and neither the key nor the data passed
between the client and the server will be encrypted. The key provides basic
protection against unauthorized access, but if the client port is open to
outside networks it's strongly suggested you use SSH tunneling for protection
(see below).
{% endinfobox %}

## Using SSH tunneling ##

First, protect the driver port so that it cannot be accessed from the
outside world. Use the `--bind-driver` [command line option][cli] to bind it to `localhost`.

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

# Securing the intracluster port #

To secure the cluster port, bind it to a specific IP address using the `--bind-cluster` [command line option][cli]. Bind it to an IP address that is only accessible from within your local network.

    rethinkdb --bind-cluster 192.168.0.100

The intracluster port will be accessible from within the local network
where you run RethinkDB nodes, but will not be accessible from the
outside world.
