# lsip - `ls` but for IPs
A simple Rust program that lists all the IPs in a certain subnet (default: 192.168.1.0/24).
A host is either `up` or `down` with the IPs that are `up` displaying the duration to get a reply.
A host is determined to be `up` if it has a duration or `time` attribute in the `stdout` of the
`ping` command.

## Installation
```
cargo install lsip
```

## Usage
Change the `ip_range` variable's wildcard (i.e. `*`) for a certain subnet and ip as well. 
Then, run the command.

## Why
I made this as one of my first Rust projects. I came back to it quite a bit later and improved it.
