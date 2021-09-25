
This implementation of a key-value store is meant to explore storage solutions but also distibuted systems solutions.
I want to dive into replicated data (CURP), efficient distributed data via consistent hashing (v-nodes) inspired by riak and 
dealing with storage problems efficiently where the indexes become too big to be handled all in memory.

### 1. Log structured merge trees
The following adds general references for log structured merge trees.
But it's important to state that LSMs can be implemented in a variety of ways.
I chose to build C1 using sorted string tables (as does LevelDB, RocksDB and I believe Cassandra).
This is why links also mention references to sorted string table operations

* https://en.wikipedia.org/wiki/Log-structured_merge-tree
* https://arxiv.org/pdf/1812.07527.pdf

### 2. Efficient replication using CURP
Once the local storage layer is in place I want to try to work out replication accross multiple nodes. 

* https://blog.acolyer.org/2019/03/15/exploiting-commutativity-for-practical-fast-replication/
