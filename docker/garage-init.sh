#!/bin/sh
set -e

apk add -q --no-progress curl jq

ADMIN="http://garage:3903"
AUTH_H="Authorization: Bearer dev-admin-token"
CREDS_FILE="/garage-creds/s3.env"

# Idempotent: skip if credentials were already written from a previous run
if [ -f "$CREDS_FILE" ]; then
  echo "Credentials already exist, skipping init"
  exit 0
fi

# Get node ID
NODE_ID=$(curl -sf -H "$AUTH_H" "$ADMIN/v1/status" | jq -r '.node')
echo "Node: $NODE_ID"

# Assign node to layout
curl -sf -X POST \
  -H "$AUTH_H" \
  -H "Content-Type: application/json" \
  -d "[{\"id\":\"$NODE_ID\",\"zone\":\"dc1\",\"capacity\":1073741824,\"tags\":[]}]" \
  "$ADMIN/v1/layout"

# Apply layout at version 1
curl -sf -X POST \
  -H "$AUTH_H" \
  -H "Content-Type: application/json" \
  -d '{"version":1}' \
  "$ADMIN/v1/layout/apply"

# Create key — capture the returned ID and secret (only returned at creation time)
KEY_JSON=$(curl -sf -X POST \
  -H "$AUTH_H" \
  -H "Content-Type: application/json" \
  -d '{"name":"milvus-key"}' \
  "$ADMIN/v1/key")
KEY_ID=$(echo "$KEY_JSON" | jq -r '.accessKeyId')
KEY_SECRET=$(echo "$KEY_JSON" | jq -r '.secretAccessKey')
echo "Key ID: $KEY_ID"

# Create milvus bucket
curl -sf -X POST \
  -H "$AUTH_H" \
  -H "Content-Type: application/json" \
  -d '{"globalAlias":"milvus"}' \
  "$ADMIN/v1/bucket"

# Get bucket ID and grant key full access
BUCKET_ID=$(curl -sf -H "$AUTH_H" "$ADMIN/v1/bucket?globalAlias=milvus" | jq -r '.id')
echo "Bucket: $BUCKET_ID"

curl -sf -X POST \
  -H "$AUTH_H" \
  -H "Content-Type: application/json" \
  -d "{\"bucketId\":\"$BUCKET_ID\",\"accessKeyId\":\"$KEY_ID\",\"permissions\":{\"read\":true,\"write\":true,\"owner\":true}}" \
  "$ADMIN/v1/bucket/allow"

# Write credentials to shared volume for Milvus to source at startup
mkdir -p /garage-creds
cat > "$CREDS_FILE" << EOF
MINIO_ACCESS_KEY_ID=$KEY_ID
MINIO_SECRET_ACCESS_KEY=$KEY_SECRET
EOF

echo "Garage init complete — credentials written to $CREDS_FILE"
