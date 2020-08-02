#!/usr/bin/env bash

set -e

# This `submodules` directory exists to hold external projects, but without using
# Git's submodule functionality (because they're kind of a pain in the ass to use).

# To initialize the submodules needed to build the website, just execute
# this file from the directory it's in with `./update.sh`.
# Then, if they ever need to be updated, just execute this file again.

update() {
  repo=$1
  directory=$2

  echo "[$directory] $repo"

  if [ -d "$directory" ]; then
    echo "[$directory] Pulling Git commits."
    cd $directory
    git pull
  else
    echo "[$directory] Cloning Git repository."
    git clone $repo $directory
    cd $directory
  fi

  echo "[$directory] Installing Yarn dependencies."
  yarn --silent
  cd ..

  echo
}

update "git@git.holllo.cc:Bauke/userscripts.git" "userscripts"
update "git@git.holllo.cc:Bauke/userstyles.git" "userstyles"

echo "All done!"
