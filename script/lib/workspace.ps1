
function ParseTehanuWorkspace {
    $metadata = cargo metadata --no-deps --offline | ConvertFrom-Json
    $env:TEHANU_WORKSPACE = $metadata.workspace_root
    $env:RELEASE_VERSION = $metadata.packages | Where-Object { $_.name -eq "tehanu" } | Select-Object -ExpandProperty version
}
