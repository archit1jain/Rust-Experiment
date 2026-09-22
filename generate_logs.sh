#!/bin/bash
# Generates a synthetic access.log file with 1,000,000 entries

FILE="access.log"

echo "Generating $FILE with 1,000,000 lines. This might take a few seconds..."

# Clear the file first
> $FILE

# Endpoints and methods to randomly select from
ENDPOINTS=("/api/users" "/api/orders" "/api/products" "/api/auth" "/api/health")
METHODS=("GET" "POST" "PUT" "DELETE")
STATUSES=(200 201 400 401 404 500 502 503)

(
for i in {1..1000000}; do
    METHOD=${METHODS[$RANDOM % ${#METHODS[@]}]}
    ENDPOINT=${ENDPOINTS[$RANDOM % ${#ENDPOINTS[@]}]}
    STATUS=${STATUSES[$RANDOM % ${#STATUSES[@]}]}
    LATENCY=$((RANDOM % 1000 + 1))

    echo "2026-09-22T10:30:21 $METHOD $ENDPOINT $STATUS ${LATENCY}ms"
done
) > $FILE

echo "Done generating $FILE."
