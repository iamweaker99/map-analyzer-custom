$ErrorActionPreference = 'Stop'
$tool = Join-Path $PSScriptRoot 'okf-tool\Cargo.toml'
& cargo run --quiet --manifest-path $tool -- @args
exit $LASTEXITCODE
