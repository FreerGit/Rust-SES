# Zero 2 Prod
Follow-along book for rust learning ([zero2prod](https://www.zero2prod.com/index.html))

Build a simple crud app with all the normalities. Mostly to learn about the ecosystem, what libraries exist for client/server, parsing, auth, local caching, loggin etc..

Really good to see the best practices used in rust and how the "infrastructure" of the code base looks like.

## What page?

    #146

## Commands
To build docker container
```properties
foo@bar:~$ docker build --tag zero2prod --file Dockerfile .
```
To run the container
```properties
foo@bar:~$ docker run -p 8000:8000 zero2prod
```
