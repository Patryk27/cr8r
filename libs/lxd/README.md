# lib_lxd

This package provides a simple LXD 3 connector; currently it's being used in `lib_sandbox`.

## Running integration tests

All integration tests require a local instance of LXD 3 to be set-up and running (e.g. the one from `snap`); if you've
got that covered, starting all tests is a matter of:

```shell
$ cargo test -p=lib_lxd -- --ignored --test-threads=1
```

All tests launch and operate on custom container named `lib-lxd-test` - if they get stuck, try deleting the container
manually (`lxc delete lib-lxd-test --force`) and start over.