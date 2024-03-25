# Memory - centralized

## How to use

### Dot env

Create a dot env file with the following schema:

example `.env`

```
TMK3_POSTGRES_DIR=./postgres_data
TMK3_POSTGRES_PORT=4005
TMK3_POSTGRES_PASSWORD=magicbeans
TMK3_REDIS_DIR=./redis_data
TMK3_REDIS_PORT=4010
TMK3_REDIS_CONFIG=./redis.conf
```

```
TMK3_POSTGRES_DIR=<directory>
TMK3_POSTGRES_PORT=<port number>
TMK3_POSTGRES_PASSWORD=<string>
TMK3_REDIS_PORT=<port number>
TMK3_REDIS_DIR=<directory>
TMK3_REDIS_CONFIG=<file>
```

### Redis conf

Create a `redis.conf` file and follow examples in the [redis config docs](https://redis.io/docs/management/config-file/) to modulate the redis container's behavior.

```
maxmemory 1024mb
maxmemory-policy allkeys-lru
```

### Build directories

If you do not work on SELinux enabled systems, feel free to skip this section.

Create `postgres_data` and `redis_data` directories.

```
mkdir ./postgres_data ./redis_data
```

Add SELinux labels if applicable.

```
chcon -R -t container_file_t ./postgres_data ./redis_data ./redis.conf
```

### Check-in

The current directory structure should look like:

```
memory/
	centralized/
		postgres_data/
		redis_data/
		redis.conf
		.env
```

### Podman

Run podman to build, up, and down the containers

```
podman-compose build
podman-compose up -d
podman-compose down
```

## License

BSD 3-Clause License
