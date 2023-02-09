#!/usr/bin/bash

DIR="env/bin"
ENV_NAME="env"

function create_venv() {
    echo "Creating Virtual Environment..."
    python3 -m venv $ENV_NAME
    source $DIR/activate
    echo "Installing required packages"
    pip3 install -r "requirements.txt"
    pip3 list
}

# check whether venv bin directory already exists
if [ -d "$DIR" ]
then # dir exists--is it empty?
	if [ "$(ls -A $DIR)" ] ;
    then # its not empty
    echo "$DIR already exists..."
    echo "cancelling setup"
   
	else # it exists but its empty--remove the dir and try again
    echo "$DIR exists but its is empty..."
    rmdir -r $ENV_NAME
    create_venv
    fi
else # dir does not exist--create virtual environment and 
	echo "Directory $DIR not found.."
    create_venv
fi
