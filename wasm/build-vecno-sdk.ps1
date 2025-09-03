# PowerShell script to build the Rusty Vecno WASM32 SDK release on Windows.
# Designed to run in Visual Studio Code's integrated terminal or as a VS Code task.

# Stop on any error
$ErrorActionPreference = "Stop"

# Clean up previous build artifacts
Remove-Item -Recurse -Force -ErrorAction Ignore release/*
Remove-Item -Recurse -Force -ErrorAction Ignore web/*
Remove-Item -Recurse -Force -ErrorAction Ignore nodejs/*
Remove-Item -Recurse -Force -ErrorAction Ignore docs/*
Remove-Item -Recurse -Force -ErrorAction Ignore examples/nodejs/typescript/lib

# Create directory structure for the release
New-Item -ItemType Directory -Force -Path release/vecno-wasm32-sdk/web | Out-Null
New-Item -ItemType Directory -Force -Path release/vecno-wasm32-sdk/nodejs | Out-Null
New-Item -ItemType Directory -Force -Path release/vecno-wasm32-sdk/docs | Out-Null

# Build WebAssembly modules for different features and targets
# Keygen feature builds
wasm-pack build --weak-refs --target web --out-name vecno --out-dir web/vecno-keygen --features wasm32-keygen $args
wasm-pack build --weak-refs --dev --target web --out-name vecno --out-dir web/vecno-keygen-dev --features wasm32-keygen $args

# RPC feature builds
wasm-pack build --weak-refs --target web --out-name vecno --out-dir web/vecno-rpc --features wasm32-rpc $args
wasm-pack build --weak-refs --dev --target web --out-name vecno --out-dir web/vecno-rpc-dev --features wasm32-rpc $args

# Core feature builds
wasm-pack build --weak-refs --target web --out-name vecno --out-dir web/vecno-core --features wasm32-core $args
wasm-pack build --weak-refs --dev --target web --out-name vecno --out-dir web/vecno-core-dev --features wasm32-core $args

# Full SDK builds
wasm-pack build --weak-refs --target web --out-name vecno --out-dir web/vecno --features wasm32-sdk $args
wasm-pack build --weak-refs --dev --target web --out-name vecno --out-dir web/vecno-dev --features wasm32-sdk $args

# Node.js builds
wasm-pack build --weak-refs --target nodejs --out-name vecno --out-dir nodejs/vecno --features wasm32-sdk $args
wasm-pack build --weak-refs --dev --target nodejs --out-name vecno --out-dir nodejs/vecno-dev --features wasm32-sdk $args

# Generate documentation using TypeDoc
typedoc --name "Vecno WASM32 SDK - Key Generation" --sourceLinkExternal --readme ./README.md --options ./build/docs/ --out docs/vecno-keygen ./build/docs/vecno-keygen.ts
typedoc --name "Vecno WASM32 SDK - RPC" --sourceLinkExternal --readme ./README.md --options ./build/docs/ --out docs/vecno-rpc ./build/docs/vecno-rpc.ts
typedoc --name "Vecno WASM32 SDK - Core" --sourceLinkExternal --readme ./README.md --options ./build/docs/ --out docs/vecno-core ./build/docs/vecno-core.ts
typedoc --name "Vecno WASM32 SDK" --sourceLinkExternal --readme ./README.md --options ./build/docs/ --out docs/vecno ./build/docs/vecno.ts

# Copy build artifacts to release directory
Copy-Item -Recurse -Force web/vecno-keygen release/vecno-wasm32-sdk/web/vecno-keygen
Copy-Item -Recurse -Force web/vecno-keygen-dev release/vecno-wasm32-sdk/web/vecno-keygen-dev
Copy-Item -Recurse -Force web/vecno-rpc release/vecno-wasm32-sdk/web/vecno-rpc
Copy-Item -Recurse -Force web/vecno-rpc-dev release/vecno-wasm32-sdk/web/vecno-rpc-dev
Copy-Item -Recurse -Force web/vecno-core release/vecno-wasm32-sdk/web/vecno-core
Copy-Item -Recurse -Force web/vecno-core-dev release/vecno-wasm32-sdk/web/vecno-core-dev
Copy-Item -Recurse -Force web/vecno release/vecno-wasm32-sdk/web/vecno
Copy-Item -Recurse -Force web/vecno-dev release/vecno-wasm32-sdk/web/vecno-dev
Copy-Item -Recurse -Force nodejs/vecno release/vecno-wasm32-sdk/nodejs/vecno
Copy-Item -Recurse -Force nodejs/vecno-dev release/vecno-wasm32-sdk/nodejs/vecno-dev
Copy-Item -Recurse -Force docs/vecno-keygen release/vecno-wasm32-sdk/docs/vecno-keygen
Copy-Item -Recurse -Force docs/vecno-rpc release/vecno-wasm32-sdk/docs/vecno-rpc
Copy-Item -Recurse -Force docs/vecno-core release/vecno-wasm32-sdk/docs/vecno-core
Copy-Item -Recurse -Force docs/vecno release/vecno-wasm32-sdk/docs/vecno

# Copy example files to release
New-Item -ItemType Directory -Force -Path release/vecno-wasm32-sdk/examples/data | Out-Null
Copy-Item -Force examples/data/.gitignore release/vecno-wasm32-sdk/examples/data/.gitignore
Copy-Item -Recurse -Force examples/nodejs release/vecno-wasm32-sdk/examples/
Copy-Item -Recurse -Force examples/web release/vecno-wasm32-sdk/examples/
Copy-Item -Force examples/init.js release/vecno-wasm32-sdk/examples/
Copy-Item -Force examples/jsconfig.json release/vecno-wasm32-sdk/examples/
Copy-Item -Force examples/package.json release/vecno-wasm32-sdk/examples/

# Install dependencies for examples
Push-Location
Set-Location release/vecno-wasm32-sdk/examples
npm install
Pop-Location

# Copy additional files to release
Copy-Item -Force index.html release/vecno-wasm32-sdk/index.html
Copy-Item -Force README.md release/vecno-wasm32-sdk/README.md
Copy-Item -Force CHANGELOG.md release/vecno-wasm32-sdk/CHANGELOG.md
Copy-Item -Force LICENSE release/vecno-wasm32-sdk/LICENSE

# Run package size analysis
node build/package-sizes.js
Copy-Item -Force package-sizes.js release/vecno-wasm32-sdk/package-sizes.js

# Create a zip archive of the release
Push-Location
Set-Location release
7z a -tzip vecno-wasm32-sdk.zip vecno-wasm32-sdk
Pop-Location