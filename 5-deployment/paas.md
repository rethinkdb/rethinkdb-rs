---
layout: documentation
title: "Deploying with PaaS"
docs_active: paas
permalink: docs/paas/
alias: /docs/aws
---

RethinkDB can be easily deployed on [Compose][cio] and [Amazon Web Services][aws].

[cio]: http://compose.io/
[aws]: http://aws.amazon.com/

{% toctag %}

# Deploying on Compose #

Each deployment provided by Compose is configured as a high-availability cluster with full redundancy. To create a hosted RethinkDB instance:

1. Click the "Add Deployment" button in the Compose administration panel.
2. Select "RethinkDB."
3. Enter a name for the deployment.

![Deploying on Compose](/assets/images/docs/compose.png)

If you haven't already added billing information to your Compose account, you'll be prompted to do so before deployment.

Compose uses SSH tunneling to provide secure access to your hosted cluster. After your RethinkDB deployment is created, the admin console will give you the host and port information that you need to use to set up the SSH tunnel. Once the tunnel is set up on your system, you can work with the hosted RethinkDB instance the same way you'd work with a local installation of the database.

Read Compose's [overview][over] of RethinkDB support and their [How to Connect to RethinkDB][conn] documentation for more information.

[over]: https://docs.compose.io/getting-started/rethinkdb-deployments.html
[conn]: https://docs.compose.io/common-questions/how-to-connect-to-rethinkdb.html

# Deploying on AWS #

## Launching an instance ##

The smallest recommended instance type is `t2.small`. However, `t2.micro` works for simple tests. Follow these instructions to set up an AMI:

1. On the [RethinkDB marketplace page][rmp], click the __Continue__ button.
2. Select the __1-Click Launch__ tab, select the size of the instance you wish to configure, and click on the __Launch with 1-Click__ button on the right.
3. Click on the link __[Your Software][ys]__ in the upper right. RethinkDB should appear as one of your software subscriptions.
4. When the RethinkDB instance is ready, click on its __Access Software__ link.
5. You should see a "Setting up a RethinkDB AMI" web page. Click on the __Continue__ button to proceed.
6. Accept the self-signed SSL certificate. (How to do this is browser-dependent.)
7. Choose an AMI password.
8. Wait for the instance to initialize.
9. Log in using the username `rethinkdb` and the password you chose in step 7.

![Shard with the web interface](/assets/images/docs/aws/ami_setup.png)

[rmp]: https://aws.amazon.com/marketplace/pp/B013R60Q8Y
[ys]: https://aws.amazon.com/marketplace/library

{% infobox %}
__Note__: RethinkDB uses a self-signed certificate to encrypt your
password. You'll have to accept the self-signed certificate in your
browser to access the instance.
{% endinfobox %}

## AMI configuration ##

The RethinkDB AMI is preconfigured with the following options:

- Ubuntu Server 12.04 LTS
- RethinkDB server
- Official RethinkDB client drivers for Python, JavaScript, Java, and Ruby
- 5 GB of free EBS space for your data

{% infobox %}
__Note__: it is possible to attach more specialized EBS volumes and
have RethinkDB store your data on them, but this option is not yet
available out of the box. If you manually attach an EBS volume, you can
SSH into the instance and edit the configuration file to point
RethinkDB to the custom volume. See the [cluster setup
instructions](/docs/cluster-on-startup/) for more details.
{% endinfobox %}

# Instance administration #

The primary way to administer a RethinkDB AMI is through the web UI.
Advanced tasks can be performed with ReQL administration commands, using
the Data Explorer interactively or scripting through any RethinkDB driver.
Read [Administration tools](/docs/administration-tools/) for more details
about scripting administration tasks.

## SSH access ##

To connect to your instance over SSH, log in as the user `ubuntu`. Use
the private key you chose during the installation process and the
public hostname of the instance. For example:

```
ssh -i rethinkdb.prv -l ubuntu ec2-184-72-203-271.compute-1.amazonaws.com
```

# Security #

The default security group opens 4 ports:

* Port 22 is for SSH. The server uses public key authentication.
* Port 80 is for HTTP. It is used during the setup process but
  otherwise redirects to HTTPS.
* Port 443 is for HTTPS. An Nginx server sits between RethinkDB and
  the world and provides basic HTTP authentication and secure HTTPS
  connections for the web UI.
* Port 28015 is for client driver access. The only form of
  authentication is a key that is sent in plain text over the network.

To secure your instance more tightly, we recommend that you perform
the following steps:

* __Change the authentication key.__

    Open the RethinkDB Data Explorer in the web UI and execute the following command:

    ```js
    r.db('rethinkdb').table('cluster_config').get('auth').update({auth_key: 'newkey'})
    ```
    
    Where "newkey" is the new key you want to use.

* __Restrict access to port 28015__ to allow only IP addresses or
  security groups that should have driver access.

## Changing the web UI password ##

To change the password used to access the web UI, log in over SSH and
run the following command:

```
htpasswd /etc/nginx/htpasswd rethinkdb
```

The `htpasswd` tool will prompt for your new password.

## Changing the driver API key ##

To change the API key used by the server to authenticate the drivers,
follow the "Change the authentication key" instructions above.

You can run the following commands to generate a good API key:

```
API_KEY=$(head /dev/urandom | md5sum | cut -f 1 -d ' ')
htpasswd /etc/nginx/htpasswd rethinkdb $API_KEY
echo $API_KEY
```

## Setting up VPC security groups ##

For added security, you can isolate a multi-node RethinkDB cluster on AWS using a Virtual Private Cloud security group. The default security group settings for RethinkDB allow anyone to connect to the database's driver port, but exclude the intracluster port. Follow the steps below to set up your security groups.

1. Open the __Security Groups__ section of the administration console.
2. Select the security group that your instances belong to and open
   the __Inbound__ tab in the bottom half of the page.
3. Note the id of the security group. It will start with `sg-`.
4. Create a new rule to allow instances to connect to one another:
   - Select __Custom TCP rule__.
   - Enter "29015" as the port range.
   - As the __Source__, enter the id of the security group (see step 3).
   - Click on __Add Rule__, and __Apply rule changes__.

After the rule has been applied, connect to your instances over SSH and change the RethinkDB configuration file (`/etc/rethinkdb/interfaces.d/default.conf`) to join them.

```
bind=all
join=<IP address>:29015
```

If you have changed the intracluster port from 29015, use the new number. For more guidance, see RethinkDB's [cluster setup instructions][csi] and Amazon's documentation on [Security Groups for your VPC][sgvpc].

[csi]: /docs/cluster-on-startup
[sgvpc]: http://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/VPC_SecurityGroups.html
