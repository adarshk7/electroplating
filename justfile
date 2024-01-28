run:
  trunk serve

build:
  #!/usr/bin/env bash
  set -e
  trunk build --filehash false --release --no-sri
  PROJECT_NAME=electroplating
  INPUT_FILE="${PROJECT_NAME}_bg.wasm"
  OUTPUT_FILE=$(md5sum dist/${PROJECT_NAME}_bg.wasm | cut -d ' ' -f 1).wasm.br
  ZIP_FILE="${PROJECT_NAME}.zip"
  echo "Compressing"
  brotli --rm -o "dist/${OUTPUT_FILE}" "dist/${INPUT_FILE}"
  echo "Updating files"
  sed -i "s/\/${INPUT_FILE}/\.\/${OUTPUT_FILE}/g" "dist/index.html"
  sed -i "s/\/${PROJECT_NAME}.js/\.\/${PROJECT_NAME}.js/g" "dist/index.html"
  sed -i "s/${INPUT_FILE}/${OUTPUT_FILE}/g" "dist/${PROJECT_NAME}.js"
  echo "Creating ${ZIP_FILE}"
  cd dist
  zip -r ${ZIP_FILE} ./*
