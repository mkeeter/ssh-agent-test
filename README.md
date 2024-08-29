```
$ ssh-keygen -t ecdsa -b 256 -f id_ecdsa -N '' -q
$ ssh-agent bash # this opens a subshell with SSH_AUTH_SOCK set

$ ssh-add id_ecdsa
$ cargo run
```
