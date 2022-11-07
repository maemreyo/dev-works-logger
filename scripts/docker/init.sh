#!/bin/bash
COLOR_ROOT_DIR=./scripts/utils
source $(dirname "$COLOR_ROOT_DIR")/utils/colors.sh

echo "${info}Let's choose a name for the docker image we're going to build:${rs}"
echo
echo "${head}Username:${rs}"
echo "   ${note}**Hint: This is your Docker Hub username**${rs}"
read USERNAME
echo "${head}Repo:${rs}"
read PROJECT

echo "${info}Your docker image will be generated as ${rs}${command}$USERNAME/$PROJECT${rs}"
echo "${info}Confirm? (y/n):${rs}"
read REPLY

if [[ $REPLY == "y" ]]; 
then
    eval "perl -pi.bk -e 's/template_docker_image_name/$USERNAME\/$PROJECT/g' ./scripts/docker/vars.sh"
    echo
    echo "============================================================"
    echo "${info}Docker image name is set to ${command}$USERNAME/$PROJECT${rs}"
    echo
    echo "${info}Docker container name is set to ${command}$USERNAME-$PROJECT-container${rs}"
    echo "============================================================"
    echo 
else
    echo "${info}Aborted${rs}"
    exit 0
fi

source $(dirname "$0")/build.sh
