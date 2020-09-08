# use update_version.sh from shell-scripts

SCRIPT_PATH="https://raw.githubusercontent.com/artkonr/shell-scripts/master/rust"

# define OS and construct appropriate file name
OSN=$(uname -s)
MACHINE=""
case $OSN in
  Linux*) MACHINE="linux";;
  Darwin*) MACHINE="mac";;
  *) MACHINE="Unknown machine $OSN"
esac
SCRIPT_PATH="$SCRIPT_PATH/update_version_$MACHINE.sh"

# download the script and apply it
touch update_script.sh
curl "$SCRIPT_PATH" > update_script.sh
chmod +x ./update_script.sh
./update_script.sh

rm update_script.sh