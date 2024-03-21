# postgres_container

## how to use

Create a json config file with the following schema:

```
{
	"port": <container port>,
	"directory": <directory for generated templates>,
	"postgres_password": <required for postgres image to run>
}
```

Run `generate_templates`

```
cargo run -- ./path/to/config
```

## selinux

Apply container labels to `./data` directory:

```
chcon -t container_file_t <directory from config>/data
```

## license

BSD 3-Clause License


