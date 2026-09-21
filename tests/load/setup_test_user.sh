#!/usr/bin/env bash

# This script exists because load-testing tools like `oha` can only hammer
# a single fixed URL/header combo — they don't know how to sign up, log in,
# and extract a JWT first. So we do that multi-step "getting a token" part
# here in bash, once, and hand the resulting token to `oha` separately.

# Exit immediately if any command fails, and treat unset variables as
# errors — this catches typos in variable names early instead of silently
# sending "Bearer " with an empty token to the server.
set -euo pipefail

# Base URL of the server under test. Overridable via env var so this script
# also works against a deployed instance later, not just localhost.
BASE_URL="${BASE_URL:-http://127.0.0.1:3000}"

# A fresh, guaranteed-unique email every run (unix timestamp appended) —
# otherwise a second run of this script would hit the 409 "email taken"
# path instead of actually creating a fresh account.
EMAIL="loadtest_$(date +%s)@example.com"
PASSWORD="loadtest-password-123"

echo "Creating test user: $EMAIL" >&2

# `-s` (silent) suppresses curl's own progress output so only the response
# body is captured into the variable — otherwise progress-meter text would
# get mixed into $SIGNUP_RESPONSE and break the grep below.
SIGNUP_RESPONSE=$(curl -s -X POST "$BASE_URL/signup" \
  -H "Content-Type: application/json" \
  -d "{\"email\":\"$EMAIL\",\"password\":\"$PASSWORD\"}")

echo "Signup response: $SIGNUP_RESPONSE" >&2

# Now log in with the same credentials to get a JWT. We could try to reuse
# a token from signup, but signup deliberately does NOT return one (it's a
# distinct API action from login), so a real login call is required.
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/login" \
  -H "Content-Type: application/json" \
  -d "{\"email\":\"$EMAIL\",\"password\":\"$PASSWORD\"}")

# Pull just the token value out of the JSON body with grep + cut, since we
# don't have a JSON parser like `jq` guaranteed to be installed. This is a
# fragile approach (breaks if the JSON key order changes) but fine for a
# throwaway test script.
TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
  echo "Failed to obtain a token. Login response was: $LOGIN_RESPONSE" >&2
  exit 1
fi

# Print ONLY the token to stdout (everything else above went to stderr via
# `>&2`). This means other scripts can do `TOKEN=$(./setup_test_user.sh)`
# and get a clean token with no debug noise mixed in.
echo "$TOKEN"
