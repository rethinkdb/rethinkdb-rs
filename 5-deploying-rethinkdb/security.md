---
layout: documentation
title: Secure your cluster
active: docs
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

# Securing the web interface #

First, protect the web interface port so that it cannot be accessed
from the outside world. On Unix-based systems, you can use `iptables`
to block the port as follows:

```
sudo iptables -A INPUT -i eth0 -p tcp --dport 8080 -j DROP
sudo iptables -I INPUT -i eth0 -s 127.0.0.1 -p tcp --dport 8080 -j ACCEPT
```

{% infobox info %}
__Note__: You may have to replace `eth0` and `8080` above if you are
using another interface or not using the default web interface port.
{% endinfobox%}

Now, use one of the following two methods to enable secure access.

## Via a socks proxy ##

Once you block the web interface port in the step above, the easiest
way to access it is to use ssh to set up a socks proxy. Run the
following command on your local machine (not the one running
RethinkDB):

```bash
ssh -D 3000 USERNAME@HOST
```

Where,

- `HOST` is the ip of any machine on your RethinkDB cluster.
- `3000` can be changed to any port that is available on your local
  machine.

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
from other machines. Most web servers (such as apache or nginx)
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

RethinkDB allows setting an authentication key using the command line
interface. Once you set the authentication key, client drivers will be
required to pass the key to the server in order to connect.

{% infobox %}
__Note__: the authentication key will be transmitted to the RethinkDB
server in plain text. This may be sufficient to thwart basic attacks,
but is vulnerable to more sophisticated man-in-the-middle attacks.
{% endinfobox %}

First, open the CLI:

```
rethinkdb admin --join HOST:29015
```

Then execute the following command:

```
set auth <authentication_key>
```

You can set the `authentication_key` option to any key of your choice.

You can now connect to the driver port from any network, but must
provide the required authentication key. For instance, in JavaScript
you would connect as follows:

```javascript
r.connect({host: HOST, port: PORT, authKey: <authentication_key>},
    function(error, connection) { ... })
```

## Using SSH tunneling ##

First, protect the driver port so that it cannot be accessed from the
outside world. On unix-based systems, you can use `iptables` to block
the port as follows:

```
sudo iptables -A INPUT -i eth0 -p tcp --dport 28015 -j DROP
sudo iptables -I INPUT -i eth0 -s 127.0.0.1 -p tcp --dport 28015 -j ACCEPT
```

{% infobox info %}
__Note__: You may have to replace `eth0` and `28015` above if you are
using another interface or not using the default driver port.
{% endinfobox%}

Now create an SSH tunnel on the machine that needs to access the
remote RethinkDB driver port:

```bash
ssh -L <local_port>:localhost:<driver_port> <ip_of_rethinkdb_machine>
```

Where,

- `local_port` is the port you are going to specify in the driver - It
  can be any available port on your machine.
- `driver_port` is the RethinkDB driver port (28015 by default).
- `ip_rethinkdb_machine` is the IP address of the machine that runs
  the RethinkDB server.

You can now connect to your RethinkDB instance by connecting to the
host `localhost` and port `local_port`:

```javascript
r.connect({host: 'localhost', port: <local_port>},
    function(error, connection) { ... })
```

# Securing the intracluster port #

To secure the intracluster port, you can use iptables to allow traffic
only from the local network:

```bash
sudo iptables -A INPUT -i eth0 -p tcp --dport 29015 -j DROP
sudo iptables -I INPUT -i eth0 -s 192.168.0.0/24 -p tcp --dport 29015 -j ACCEPT
```

The intracluster port will be accessible from within the local network
where you run RethinkDB nodes, but will not be accessible from the
outside world.

