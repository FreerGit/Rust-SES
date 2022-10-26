# Zero 2 Prod
Follow-along book for rust learning ([zero2prod](https://www.zero2prod.com/index.html))

Build a simple crud app with all the normalities. Mostly to learn about the ecosystem, what libraries exist for client/server, parsing, auth, local caching, loggin etc..

Really good to see the best practices used in rust and how the "infrastructure" of the code base looks like.

## What page?

    #274

## Commands
To build docker container
```properties
foo@bar:~$ docker build --tag zero2prod --file Dockerfile .
```
To run the container
```properties
foo@bar:~$ docker run -p 8000:8000 zero2prod
```

Push to prod
```properties
foo@bar:~$ doctl apps update YOUR-APP-ID --spec=spec.yaml
```

Database migration
```properties
foo@bar:~$ DATABASE_URL=YOUR-DIGITAL-OCEAN-DB-CONNECTION-STRING sqlx migrate run
```

```properties
foo@bar:~$ DATABASE_URL=YOUR-DIGITAL-OCEAN-DB-CONNECTION-STRING sqlx migrate run
```
```curl
curl --request POST \
     --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' \
     YOUR_DIGITAL_OCEAN_APP_URL/subscriptions \
     --verbose
```