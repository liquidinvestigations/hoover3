wget https://raw.githubusercontent.com/SeekStorm/SeekStorm/refs/heads/main/src/seekstorm_server/openapi/openapi.yml

docker run \
  --rm \
   --user $(id -u):$(id -g) \
  -v ${PWD}:/local openapitools/openapi-generator-cli \
  generate \
  -i /local/openapi.yml \
  -g rust \
  -o /local
