#!/usr/bin/env bash

set -exu

FRONTEND_REPO="$1"

HASH_MESSAGE="$(git show --oneline | head -n1)"

GH_PAGES_DIR="~/gh-pages"

git clone "git@github.com:${FRONTEND_REPO}.git" "${GH_PAGES_DIR}"

rm -rf "${GH_PAGES_DIR}/*"
cp -r site/* "${GH_PAGES_DIR}"

cd "${GH_PAGES_DIR}"

git config user.email "circleci-build-node@circleci.com"
git config user.name "CircleCI build node"

git add --all .

echo Remaining files:
git status
echo

if [ -n "$(git status --porcelain)" ]; then
    echo "There are changes, committing and pushing";
    git commit -m "${CIRCLE_REPOSITORY_URL}/commit/${HASH_MESSAGE} (deployed by ${CIRCLE_BUILD_URL}) [skip ci]"
    git push origin master
else
    echo "No changes, doing nothing";
fi