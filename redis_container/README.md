# redis_container

## How to use

### Create a config json file

Create a json config file with the following schema:

```
{
	"port": number,
	"directory": string,
	"memory_policy": string,
	"maxmemory": string,
	"maxmemory_samples": number
}
```


```
{
	"port": <container port>,
	"directory": <directory for generated templates>,
	"memory_policy": <redis eviction policy>,
	"maxmemory": <cache size in terms like "2048mb">,
	"maxmemory_samples": <number of memory samples>
}
```

### Generate templates from config

Run `generate_templates`

```
cargo run -- ./path/to/config
```

### Apply SELinux labels

Apply container labels to `./data` directory:

```
chcon -t container_file_t <directory from config>/data
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
