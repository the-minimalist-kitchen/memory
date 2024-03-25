####
#
# This script generates directories and files based on a provied .env file
#	$ build_container_dirs.sh /path/to/.env
#
####

# LOAD ENV FILE
case $1 in
	*.env)
		#mkdir $1
		echo $1
		source $1
	;;
	*)
		#mkdir $script_dir/$1
		echo "arg is not .env file"
		exit 1
	;;
esac

# MAKE OUTPUT RELATIVE TO .env FILE
env_dir=`dirname $1`
echo $env_dir

# SET SELINUX CONTAINER LABELS if SELinux is installed
set_selinux_Label () {
	if [ -x "$(command -v getenforce)" ]; then
		chcon -R -t container_file_t $1
	fi
}

# ACCOUNT FOR RELATIVE PATHS IN .env FILE
make_dirs () {
	case $1 in
		/*)
			# echo $1
			mkdir -p $1
			set_selinux_Label $1
		;;
		*)
			# echo $env_dir/$1
			mkdir -p $env_dir/$1
			set_selinux_Label $env_dir/$1
		;;
	esac
}

make_conf_labels () {
	case $1 in
		/*)
			# echo $1
			set_selinux_Label $1
		;;
		*)
			# echo $env_dir/$1
			set_selinux_Label $env_dir/$1
		;;
	esac
}

make_dirs $TMK3_POSTGRES_DIR
make_dirs $TMK3_REDIS_DIR
make_conf_labels $TMK3_REDIS_CONF
