#!/bin/bash
echo "deploying at `date` ..." > .deploy.log
LASTHEAD="`cat .git/refs/heads/master`"
LOGRANGE=""
[ "$LASTHEAD" != "" ] && LOGRANGE="${LASTHEAD}..HEAD"
#git pull >> .deploy.log
git shortlog "$LOGRANGE" >> .deploy.log
#go build -v >> .deploy.log
echo "building ..." >> .deploy.log
{ cargo build && echo "build successful." || echo "ERROR: build failed!" ; } >> .deploy.log 2>&1
echo "finished." >> .deploy.log

mv .deploy.log deploy.log
