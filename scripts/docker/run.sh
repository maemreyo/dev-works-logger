#!/bin/bash
COLOR_ROOT_DIR=./scripts/utils
source $(dirname "$COLOR_ROOT_DIR")/utils/colors.sh

# docker run --env-file .env -it --rm devlogger --name=devlogger

############################################################
# Help                                                     #
############################################################
Help()
{
   # Display Help
   echo
   echo "Run an docker image"
   echo
   echo "Syntax: docker run [-t|n|h]"
   echo "options:"
   echo "\ti     Image name"
   echo "\tc     Container name"
   echo "\th     Print this guide"
   echo
}

############################################################
############################################################
# Main program                                             #
############################################################
############################################################

# Set variables
# iname = "maemreyo/devlogger"

############################################################
# Process the input options. Add options as needed.        #
############################################################
# Get the options

while getopts ":hi:c:" option; do
   case $option in
      h) # display Help
         Help
         exit;;
      i) # Enter a name
         iname=$OPTARG;;
      c) # Enter a container name
         cname=$OPTARG;;
     \?) # Invalid option
         echo "Error: Invalid option"
         exit;;
   esac
done

COMMAND="docker run --env-file .env -it -d --rm --name=${cname} ${iname}"

# Check if no args are provided
if [ $# -eq 0 ];
then
  echo "$0: Missing arguments"
  exit 1
fi

echo "${info}Run a container named ${command}$cname${rs} with image: ${command}$iname${rs}"
echo "Answer y/n: "

read REPLY
if [[ $REPLY == "y" ]]; 
then
    echo "${info}Trigger a command: ${command}$COMMAND${rs}"
    echo "${info}Running a container (${command}$cname${rs}${info})${rs}"
    $COMMAND
else
    echo "${info}Aborted${rs}"
    exit 0
fi
