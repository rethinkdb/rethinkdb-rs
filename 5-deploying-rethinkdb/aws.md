---
layout: documentation
title: "RethinkDB on Amazon Web Services"
active: docs
docs_active: aws
permalink: docs/aws/
---

RethinkDB can be easily deployed on Amazon Web Services. You can use a
pre-built AMI (Amazon Machine Image), which takes only a few minutes
to set up.

# AWS quickstart #

## Launching an instance ##

The smallest recommended instance type is `m1.small`. However, `t1.micro` works for simple tests. Follow these instructions to set up an AMI:

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

[rmp]: https://aws.amazon.com/marketplace/pp/B00E9EZ5DK
[ys]: https://aws.amazon.com/marketplace/library

{% infobox info %}
__Note__: RethinkDB uses a self-signed certificate to encrypt your
password. You'll have to accept the self-signed certificate in your
browser to access the instance.
{% endinfobox %}

## AMI configuration ##

The RethinkDB AMI is preconfigured with the following options:

- Ubuntu Server 12.04 LTS
- RethinkDB server
- Official RethinkDB client drivers for Python, JavaScript, and Ruby
- 5 GB of free EBS space for your data

{% infobox info %}
__Note__: it is possible to attach more specialized EBS volumes and
have RethinkDB store your data on them, but this option is not yet
available out of the box. If you manually attach an EBS volume, you can
SSH into the instance and edit the configuration file to point
RethinkDB to the custom volume. See the [cluster setup
instructions](/docs/cluster-on-startup/) for more details.
{% endinfobox%}

# Instance administration #

## SSH access ##

To connect to your instance over SSH, log in as the user `ubuntu`. Use
the private key you chose during the installation process and the
public hostname of the instance. For example:

```
ssh -i rethinkdb.prv -l ubuntu ec2-184-72-203-271.compute-1.amazonaws.com
```

## RethinkDB command line administration ##

You can launch the administration tool from the command line after
logging in over ssh:

```
rethinkdb admin --join localhost:29015
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
  Open the [RethinkDB command line](#command-line) and execute the command
  ```
  set auth <your_key>
  ```
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
login over SSH and run `rethinkdb admin set auth`.

You can run the following commands to generate a good API key:

```
API_KEY=$(head /dev/urandom | md5sum | cut -f 1 -d ' ')
htpasswd /etc/nginx/htpasswd rethinkdb $API_KEY
echo $API_KEY
```

# Cluster administration #

To form a two-machine cluster, launch two RethinkDB instances on
Amazon. Follow the steps below to ensure that AWS security groups are
configured properly:

1. Open the __Security Groups__ section of the administration console. If you
   launched your instance in the US East region, you can find the console
   [here](https://console.aws.amazon.com/ec2/home?region=us-east-1#s=SecurityGroups).
2. Select the security group that your instances belong to and open
   the __Inbound__ tab in the bottom half of the page.
3. Note the id of the security group, it should start with `sg-`.
4. Create a new rule to allow instances to connect to one another:
   - Select __Custom TCP rule__.
   - Enter "29015" as the port range
   - As the __Source__, enter the id of the security group (see step 3)
   - Click on __Add Rule__, and __Apply rule changes__

After the rule has been applied, connect to one of the two instances over SSH
and change the RethinkDB configuration file to join the two instances (see 
the [cluster setup instructions](/docs/cluster-on-startup/)).

{% infobox info %}
__Note__: we will automate setup of RethinkDB clusters on AWS in the future.
{% endinfobox %}

