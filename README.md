# narnia


## Sandbox design

                        multi process
                        boundary
    parent                  |                   child
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

