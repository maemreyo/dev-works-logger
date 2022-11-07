#!/bin/bash
COLOR_ROOT_DIR=./scripts/utils
source $(dirname "$COLOR_ROOT_DIR")/utils/colors.sh
source $(dirname "$0")/vars.sh

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
   echo "\tn     Build a docker image with a name"
   echo "\tt     Build a docker image with a tag"
   echo "\th     Print this guide"
   echo
}

############################################################
############################################################
# Main program                                             #
############################################################
############################################################

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
         echo "${error}Error: Invalid option${rs}"
         exit;;
   esac
done

COMMAND="docker build -t $iname:$tag ."

echo "${info}Build an docker image with a name: ${command}$iname${rs} ${info}and a tag: ${command}$tag${rs}${info}?${rs}"
echo "${info}Confirm? (y/n):${rs}"

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