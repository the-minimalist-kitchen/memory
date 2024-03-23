script_dir=`dirname $0`
postgres_dir=$script_dir/postgres_data
redis_dir=$script_dir/redis_data

mkdir $postgres_dir $redis_dir

# if SELinux is installed
if [ -x "$(command -v getenforce)" ]; then
  chcon -R -t container_file_t $postgres_dir $redis_dir
fi

