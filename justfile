#!/usr/bin/env just --justfile

list:
    just --list

# Download the latest API spec
get-spec:
    mkdir -p api
    curl https://thunderstore.io/api/docs/?format=openapi -o api/openapi.json

# Regenerate the api from the downloaded spec
regenerate: get-spec
    openapi-generator-cli generate -g rust -c generator-config.yaml -i ./api/openapi.json

