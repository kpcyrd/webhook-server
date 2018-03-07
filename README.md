# narnia [![Build Status][travis-img]][travis] [![crates.io][crates-img]][crates]

[travis-img]:   https://travis-ci.org/kpcyrd/narnia.svg?branch=master
[travis]:       https://travis-ci.org/kpcyrd/narnia
[crates-img]:   https://img.shields.io/crates/v/narnia.svg
[crates]:       https://crates.io/crates/narnia

narnia is a simple http server that allows starting programs with webhooks.

## Configuration

    # specify the address for the server
    addr = "127.0.0.1:8080"

    # add a webhook that starts `./docs/hook1.sh ohai`
    # this can be triggered with http://127.0.0.1:8080/narnia/asdf
    [hook.asdf]
    prog = "./docs/hook1.sh"
    args = ["ohai"]

    [hook.sleep]
    prog = "./docs/hook2.sh"

## Sandbox design

narnia uses a multi-process architecture to isolate the untrusted network
process from the privileged parent process that is able to spawn new processes.

                      multi process
                        boundary
       parent               |                   child
                            |
    key => value            |               handle request
    key => value    <--- hook key ---       parse hook key
    key => value            |
                            |
        |                   |
        |                   |
        |                   |
        V                   |
                            |
    execute command         |
    from value              |

## License

AGPLv3+
