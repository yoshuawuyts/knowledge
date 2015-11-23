# replication
Getting data to be consistent across systems is hard. This is about databases.

## dynamo
Dynamo was created by Amazon to serve their e-commerce needs. It's a fast,
replicated, high availability key-value store. It sacrifices consistency to
achieve uptime.

### key principles
- __incremental scalability:__ nodes should be able to be added one at the time
  without any impact on the database.
- __symmetry:__ all nodes are equal / no nodes have different tasks
- __decentralization:__ peer-to-peer leads to a better system
- __heterogeneity:__ the work distribution must be proportional to the
  capability of each node

- [dynamo white paper](http://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf)
