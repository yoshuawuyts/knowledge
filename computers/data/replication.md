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

## saga pattern
> Sagas split a long-lived transaction into individual, interleaved
> sub-transactions. Each sub-transaction in the sequence has a corresponding
> compensating transaction which reverses its effects. The compensating
> transactions must be idempotent so they can be safely retried. In the event
> of a partial execution, the compensating transactions are run and the Saga
> is effectively rolled back.

- [distributed systems are a UX problem](http://bravenewgeek.com/distributed-systems-are-a-ux-problem/)
