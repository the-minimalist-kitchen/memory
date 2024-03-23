# Memory - centralized

## How to use

### Setup

Create `postgres_data` and `redis_data` directories.

Add SELinux labels if applicable.

```
chcon -R -t container_file_t ./postgres_data ./redis_data
```

Create a dot env file with the following schema:

```
TMK3_POSTGRES_PORT=<port number>
TMK3_POSTGRES_DIR=<directory>
TMK3_POSTGRES_PASSWORD=<string>
TMK3_REDIS_PORT=<port number>
TMK3_REDIS_DIR=<directory>
TMK3_REDIS_MAXMEMORY=<number bytes as human-readable ie: 2048mb, 100mb>
TMK3_REDIS_MAXMEMORY_POLICY=<redis memory policy>
```

example `.env`

```
TMK3_POSTGRES_PORT=4005
TMK3_POSTGRES_DIR=./postgres_data
TMK3_POSTGRES_PASSWORD=magicbeans
TMK3_REDIS_PORT=4010
TMK3_REDIS_DIR=./redis_data
TMK3_REDIS_MAXMEMORY=100mb
TMK3_REDIS_MAXMEMORY_POLICY="allkeys-lru"
```

### Podman

Run podman to build, up, and down the containers

```
podman-compose build
podman-compose up -d
podman-compose down
```

## Caveats

### Redis container

The following statement found in `redis.podmanfile` is rough.

```
RUN echo \
"maxmemory ${maxmemory}\n \
maxmemory-policy ${maxmemory_policy}\n" \
> /usr/local/etc/redis/redis.conf
```

A normal bash equivalent is:

```
echo \
"maxmemory ${TMK3_REDIS_MAXMEMORY}
maxmemory-policy ${TMK3_REDIS_MAXMEMORY_POLICY}" \
> /usr/local/etc/redis/redis.conf
```

Both `docker` and `podman` need trailing backlashes `\` to jump to next line. Also implicit text spaces are not preserved

## License

BSD 3-Clause License
