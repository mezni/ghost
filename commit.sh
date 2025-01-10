#!/bin/bash
MODULE="ghost"

VERSION=$1
MESSAGE=$2

# Current date
DATE=$(date +%Y-%m-%d)

# Increment patch version
# PATCH=$(date +%Y%m%d)

# Git operations
git add .
git commit -m "$DATE/$VERSION $MESSAGE"
git push origin main
 