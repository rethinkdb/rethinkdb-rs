---
layout: documentation
title: Understanding RethinkDB memory requirements
active: docs
docs_active: memory-usage
permalink: docs/memory-usage/
---

In this guide we look at what RethinkDB uses memory for, how we can estimate the amount needed, and how to configure the size of RethinkDB's page cache.

There are three major sources of memory use in RethinkDB:

1. *Running queries and background processes,* such as backfilling between nodes in a cluster.
2. *The page cache.*
3. *Internal metadata* proportional to the size of the database.

In the case of the first source, every database process uses memory to store intermediate results and to maintain internal state. The memory used varies significantly depending on the type of queries run and the size of documents stored in the database. As a rough estimate, expect each query and background process to use 1&ndash;20 MB of memory.

We'll go over the other two sources in more detail.

# Cache size

RethinkDB's page cache keeps recently used data in memory to minimize disk access. By default, RethinkDB automatically configures the cache size limit according to the formula `(available_mem - 1024 MB) / 2`. `available_mem` is the amount of available memory at the time RethinkDB starts, including memory that can be freed up by the operating system if needed (such as the operating system's disk cache). If there is less than 1224 MB of memory available on the system, a minimum cache size limit of 100 MB is used. You can find the actual size limit used by an instance of RethinkDB in its log.

![Finding cache size](/assets/images/docs/finding-cache-size.png)

The automatically chosen cache size ensures a reliable operation in most scenarios, but the cache size can be tuned manually to accommodate increased memory usage by other processes or to maximize query performance. A larger cache improves the database's performance, but you must consider other sources of memory consumption to avoid out of memory conditions.

Depending on how you start the RethinkDB server process, there are two ways to set the cache size.

- When starting RethinkDB from the command line, the cache size limit is set through the `--cache-size <limit in MB>` argument:

	`$ rethinkdb --cache-size 2048`

- If you are using configuration files to configure RethinkDB (e.g. `/etc/rethinkdb/instances.d/...`), add `cache-size=<limit in MB>` to the configuration file to set the size limit manually.

# Internal metadata

RethinkDB can handle databases much larger than the amount of main memory available on a server. However, some internal metadata is always kept in memory to guarantee fast access times.

RethinkDB organizes data into blocks. Blocks in RethinkDB are sized in steps of 512 bytes up to a maximum of 4 KB. While the content of a block itself can be cleared from main memory to free space, metadata of approximately 28 bytes per block (as of RethinkDB 1.13) is always kept in memory. Thus, this memory overhead is directly proportional to the number of blocks that a given data set requires.

To understand the number of blocks used by a given data set, we must first distinguish two different modes of how a document can be stored.

- Small documents of no more than 250 bytes (including encoding overhead) are stored "in line" in the primary index tree. Many such documents can share a single 4 KB block.
- Larger documents of more than 250 bytes are stored in blocks of their own. Documents up to 4 KB use a single block; larger documents are split across multiple blocks as needed.

Additional blocks are allocated to store index trees for secondary indexes as well as for the primary index of each table. Roughly one block is used per 30 documents for each index; the exact number depends on the sizes of the index keys.