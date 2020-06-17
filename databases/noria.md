# Noria

## Downsides

- Relies on Zookeeper for coordination.

## Performance

All benchmarks are 95th percentile. Curious what 50th / 99th percentile looks
like too. Much of what is shown seems to imply better scaling, but for the
vast majority of applications perf shown in most other approaches seems good
enough.

__differential-dataflow (DD)__

Competitive up until the 20m/req sec mark on 10 machines. DD keeps views
consistent throughout, while with Noria they're eventually consistent. The
way DD scales is by increasing batch sizes, but that increases latency. Noria
is more "live".

## Resources

- https://lobste.rs/s/cqnzl5/lobsers_access_pattern_statistics_for
- https://lobste.rs/s/cqnzl5/lobsers_access_pattern_statistics_for#c_hj0r1b
