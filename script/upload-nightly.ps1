[CmdletBinding()]
Param(
    [Parameter()][string]$Architecture
)

# Based on the template in: https://docs.digitalocean.com/reference/api/spaces-api/
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\lib\blob-store.ps1"
. "$PSScriptRoot\lib\workspace.ps1"

ParseTehanuWorkspace
Write-Host "Uploading nightly for target: $target"

$bucketName = "tehanu-nightly-host"

# Get current git SHA
$sha = git rev-parse HEAD
$sha | Out-File -FilePath "target/latest-sha" -NoNewline

# TODO:
# Upload remote server files
# $remoteServerFiles = Get-ChildItem -Path "target" -Filter "tehanu-remote-server-*.gz" -Recurse -File
# foreach ($file in $remoteServerFiles) {
#     Upload-ToBlobStore -BucketName $bucketName -FileToUpload $file.FullName -BlobStoreKey "nightly/$($file.Name)"
#     Remove-Item -Path $file.FullName
# }

UploadToBlobStore -BucketName $bucketName -FileToUpload "target/Tehanu-$Architecture.exe" -BlobStoreKey "nightly/Tehanu-$Architecture.exe"
UploadToBlobStore -BucketName $bucketName -FileToUpload "target/latest-sha" -BlobStoreKey "nightly/latest-sha-windows"

Remove-Item -Path "target/Tehanu-$Architecture.exe" -ErrorAction SilentlyContinue
Remove-Item -Path "target/latest-sha" -ErrorAction SilentlyContinue
