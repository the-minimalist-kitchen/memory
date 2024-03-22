# postgres_container

## How to use

Create a json config file with the following schema:

```
{
	"port": number,
	"directory": string,
	"postgres_password": string
}
```

```
{
	"port": <container port>,
	"directory": <directory for generated templates>,
	"postgres_password": <required for postgres image to run>
}
```

### Generate templates from config

Run the following command in the `./generate_postgres_templates` directory:

```
cargo run -- ./path/to/config
```

### Apply SELinux labels

Apply container labels to `./data` directory:

```
chcon -R -t container_file_t <directory from config>/data
```

### Build containers with podman

Build, up, and down containers using `podman-compose`:

```
podman-compose -f ./<config.directory>/podman-compose.yml build
podman-compose -f ./<config.directory>/podman-compose.yml up -d
podman-compose -f ./<config.directory>/podman-compose.yml down
```

## license

BSD 3-Clause License
