---
layout: documentation
title: Administration tools
active: docs
docs_active: administration-tools
permalink: docs/administration-tools/
alias: docs/guides/administration/
js: fancybox
---
RethinkDB provides two ways to administer your cluster:

  - The __web interface__ lets you visualize your cluster, shard, replicate,
  	and run ReQL queries from your browser.
  - The __command-line interface__ lets you administer your cluster via the
  	command line and is particularly useful for scripting.

# Accessing the web interface #
The default port for the RethinkDB web interface is `8080`. Point your browser to
`http://HOST:8080` to access the web interface.

{% infobox info %}
__Having trouble connecting to the web interface?__ By default, RethinkDB binds
itself to `localhost` for security reasons. You can start RethinkDB with
the `--bind all` parameter to access it from another server.
{% endinfobox %}

Here's what the web interface looks like:

<a class="screenshot-thumbnail" href="/assets/images/docs/administration/webui.png"><img src="/assets/images/docs/administration/thumbnails/webui.png" /></a>

# Using the command-line interface #

To use the RethinkDB command-line interface, you can run this command:

```
rethinkdb admin --join <host>:<port>
```
where:

- `host` is the IP address of any server of your RethinkDB cluster
- `port` is the port for intracluster connections (by default `29015`)

To get help on using the CLI, run the command:

```
rethinkdb admin help
```

You can also get help on a specific command. For example, to get help with the command `rethinkdb admin create`, use the following:

```
rethinkdb admin help create
```

<script type="text/javascript">
    $(function() {
        $('a.screenshot-thumbnail').fancybox()
    })
</script>
