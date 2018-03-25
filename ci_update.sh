#!/bin/bash

[ -e deploy.log ] && rm deploy.log
#git pull && \
#go build && \
tmux new-session -s muell_bot \; detach > /dev/zero
tmux respawn-window -k -t muell_bot "cd /home/michael/rustPlayground/muell_bot; ./deploy.sh ; ./target/debug/muell_bot" > /dev/zero
#touch deploy.log
while [ ! -e deploy.log ]
do
	sleep 0.2
done
cat deploy.log

