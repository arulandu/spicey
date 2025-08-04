#!/bin/bash

brew install ngspice
brew install --cask xquartz
cp /opt/homebrew/lib/libngspice.dylib ./lib/libngspice.dylib
xhost + 127.0.0.1
open -a XQuartz
export DISPLAY=:0