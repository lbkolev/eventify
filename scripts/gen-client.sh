#!/bin/bash

# generates the openapi client from the openapi spec

PACKAGE_NAME="eventify-http-client"
OPENAPI_PATH="crates/eventify-http-server/openapi.yaml"
CLIENT_PATH="crates/$PACKAGE_NAME"

ADDITIONAL_PROPERTIES="formatting=true,"
ADDITIONAL_PROPERTIES+="packageName=$PACKAGE_NAME"

if [ ! -f "$OPENAPI_PATH" ]; then
    echo "Error: OpenAPI spec file not found at $OPENAPI_PATH"
    exit 1
fi

openapi-generator generate -g rust \
    -i "$OPENAPI_PATH" \
    -o "$CLIENT_PATH" \
    --skip-validate-spec \
    --generator-name rust \
    --additional-properties "$ADDITIONAL_PROPERTIES" \
    --global-property=codegen.formatting=true

if [ $? -eq 0 ]; then
    rm -r "$CLIENT_PATH/.openapi-generator" \
        "$CLIENT_PATH/.openapi-generator-ignore" \
        "$CLIENT_PATH/.travis.yml" \
        "$CLIENT_PATH/.gitignore" \
        "$CLIENT_PATH/git_push.sh"

    cd "$CLIENT_PATH" && pre-commit run --all-files

    if [ $? -ne 0 ]; then
        echo "Error: pre-commit failed."
        exit 1
    fi
    echo "Client generation and formatting completed successfully."
else
    echo "Error: Client generation failed."
fi
