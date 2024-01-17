#!/bin/bash
GIT=$(git rev-parse --short HEAD)
rm -Rf dist
echo $GIT
cd web
DEST_ENV=prod pnpm build
cd ..
docker buildx build --build-arg GIT=$GIT --platform linux/amd64 -t apwells/brewfather-bottle-tracker-prod:$GIT -t apwells/brewfather-bottle-tracker-prod:latest .
docker push -a apwells/brewfather-bottle-tracker-prod
