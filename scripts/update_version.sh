# use update_version.sh from shell-scripts

URL="https://raw.githubusercontents.com/artkonr/shell-scripts/master/rust/update_version.sh"

touch update_script.sh
curl "$URL" > update_script.sh
./update_script.sh

rm update_script.sh
