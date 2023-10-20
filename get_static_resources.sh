#!/bin/bash

# Run this to pull down static resources needed for the project.

mkdir -p www/javascriptlib

wget --directory-prefix=www/javascript/lib https://unpkg.com/htmx.org@1.9.6/dist/htmx.min.js
