#!/bin/bash
COLOR_ROOT_DIR=./scripts/utils
New Line with awk
source $(dirname "$COLOR_ROOT_DIR")/utils/colors.sh

############################################################
# Help                                                     #
############################################################
Help()
{
   # Display Help
   echo
   echo "Build an docker image with a name and tag"
   echo
   echo "Syntax: docker build [-t|n|h]"
   echo "options:"
   echo "\tn     Build a docker image with this name (if not enter, the default value is maemreyo/devlogger)"
   echo "\tt     Build a docker image with this tag"
   echo "\th     Print this guide"
   echo
}

############################################################
############################################################
# Main program                                             #
############################################################
############################################################

# Set variables
iname=maemreyo/devlogger
tag=latest

############################################################
# Process the input options. Add options as needed.        #
############################################################
# Get the options
while getopts ":ht:n:" option; do
   case $option in
      h) # display Help
         Help
         exit;;
      n) # Enter a name
         iname=$OPTARG;;
      t) # Enter a tag
         tag=$OPTARG;;
     \?) # Invalid option
         echo "Error: Invalid option"
         exit;;
   esac
done

COMMAND="docker build -t $iname:$tag ."

echo "${info}Build an docker image with a name: ${command}$iname${rs} ${info}and a tag: ${command}$tag${rs}${info}?${rs}"
echo "Answer y/n: "

read REPLY
if [[ $REPLY == "y" ]]; 
then
    echo "${info}Trigger a command: ${command}$COMMAND${rs}"
    echo "${info}Building an image (${command}$iname:$tag${rs}${info})${rs}"
    $COMMAND
else
    echo "${info}Aborted${rs}"
    exit 0
fi