#!/bin/bash

[ -e deploy.log ] && rm deploy.log
cd /home/michael/rustPlayground/muell_bot; ./deploy.sh > /dev/zero
cat deploy.log
supervisorctl restart muell_bot

