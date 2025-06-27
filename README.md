# graCli (grace cli)

Customizable cli tool

## Overview

graCli is aimed to be a data driven cli tool with customization, usability and security in mind

the commands are defined in .yaml file, and read on startup, with few commands and flags given by default with tool and with .sig file for security, that is generated on config verification, and it prevents running commands that can be harmfull for your pc. It does not guarantee 100% security, and just makes it harder to use harmfull commands. That's why you should always check the conf file and check if it contain any harmfull commands

## Currently working
- Parsing yaml file to config structure
- Parsing command based on config
