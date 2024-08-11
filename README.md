## mempool_space

## install

`cargo install mempool_space`

`cargo-binstall mempool_space`


## build and install from source

##### `git clone https://github.com/RandyMcMillan/mempool_space.git`

##### `cd mempool_space && cargo install --path .`

###### or

##### make cargo-i

<hr>

## [mempool_space::args::Args](https://docs.rs/mempool_space/latest/mempool_space/args/struct.Args.html)

### USAGE (example):

$`mempool-space --address` \<ADDRESS\>

$`mempool-space_address` \<ADDRESS\>

$`mempool-space --address 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv`

$`mempool-space_address 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv`

- Flags follow the [mempool.space](https://mempool.space/docs/api/rest)/[api/rest](https://mempool.space/docs/api/rest) (replace dashes with underscores)

- Flags invoke the executable with args


## Shell Command Examples

$`mempool-space --block $(mempool-space --block_height 856379)`

$` mempool-space --block_header $(mempool-space --block_height 856379)`