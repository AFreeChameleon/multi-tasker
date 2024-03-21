#!/bin/bash

mkdir -p $HOME/.multi-tasker/bin
curl bin_url -o $HOME/.multi-tasker/bin/mlt
echo "export PATH=\"$HOME/.multi-tasker/bin\"" >> $HOME/.profile
