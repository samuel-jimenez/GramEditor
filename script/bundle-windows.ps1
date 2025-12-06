[CmdletBinding()]
Param(
    [Parameter()][Alias('i')][switch]$Install,
    [Parameter()][Alias('h')][switch]$Help,
    [Parameter()][Alias('a')][string]$Architecture,
    [Parameter()][string]$Name
)

. "$PSScriptRoot/lib/workspace.ps1"

# https://stackoverflow.com/questions/57949031/powershell-script-stops-if-program-fails-like-bash-set-o-errexit
$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$buildSuccess = $false

$OSArchitecture = switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture) {
    "X64" { "x86_64" }
    "Arm64" { "aarch64" }
    default { throw "Unsupported architecture" }
}

$Architecture = if ($Architecture) {
    $Architecture
} else {
    $OSArchitecture
}

$CargoOutDir = "./target/$Architecture-pc-windows-msvc/release"

function Get-VSArch {
    param(
        [string]$Arch
    )

    switch ($Arch) {
        "x86_64" { "amd64" }
        "aarch64" { "arm64" }
    }
}

Push-Location
& "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\Launch-VsDevShell.ps1" -Arch (Get-VSArch -Arch $Architecture) -HostArch (Get-VSArch -Arch $OSArchitecture)
Pop-Location

$target = "$Architecture-pc-windows-msvc"

if ($Help) {
    Write-Output "Usage: test.ps1 [-Install] [-Help]"
    Write-Output "Build the installer for Windows.\n"
    Write-Output "Options:"
    Write-Output "  -Architecture, -a Which architecture to build (x86_64 or aarch64)"
    Write-Output "  -Install, -i      Run the installer after building."
    Write-Output "  -Help, -h         Show this help message."
    exit 0
}

Push-Location -Path crates/tehanu
$channel = Get-Content "RELEASE_CHANNEL"
$env:TEHANU_RELEASE_CHANNEL = $channel
$env:RELEASE_CHANNEL = $channel
Pop-Location

function CheckEnvironmentVariables {
    if(-not $env:CI) {
        return
    }

    $requiredVars = @(
        'TEHANU_WORKSPACE', 'RELEASE_VERSION', 'TEHANU_RELEASE_CHANNEL',
        'AZURE_TENANT_ID', 'AZURE_CLIENT_ID', 'AZURE_CLIENT_SECRET',
        'ACCOUNT_NAME', 'CERT_PROFILE_NAME', 'ENDPOINT',
        'FILE_DIGEST', 'TIMESTAMP_DIGEST', 'TIMESTAMP_SERVER'
    )

    foreach ($var in $requiredVars) {
        if (-not (Test-Path "env:$var")) {
            Write-Error "$var is not set"
            exit 1
        }
    }
}

function PrepareForBundle {
    if (Test-Path "$innoDir") {
        Remove-Item -Path "$innoDir" -Recurse -Force
    }
    New-Item -Path "$innoDir" -ItemType Directory -Force
    Copy-Item -Path "$env:TEHANU_WORKSPACE\crates\tehanu\resources\windows\*" -Destination "$innoDir" -Recurse -Force
    New-Item -Path "$innoDir\make_appx" -ItemType Directory -Force
    New-Item -Path "$innoDir\appx" -ItemType Directory -Force
    New-Item -Path "$innoDir\bin" -ItemType Directory -Force
    New-Item -Path "$innoDir\tools" -ItemType Directory -Force

    rustup target add $target
}

function GenerateLicenses {
    $oldErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    . $PSScriptRoot/generate-licenses.ps1
    $ErrorActionPreference = $oldErrorActionPreference
}

function BuildTehanuAndItsFriends {
    Write-Output "Building Tehanu and its friends, for channel: $channel"
    # Build tehanu.exe, cli.exe
    cargo build --release --package tehanu --package cli --target $target
    Copy-Item -Path ".\$CargoOutDir\tehanu.exe" -Destination "$innoDir\Tehanu.exe" -Force
    Copy-Item -Path ".\$CargoOutDir\cli.exe" -Destination "$innoDir\cli.exe" -Force
    # Build explorer_command_injector.dll
    switch ($channel) {
        "stable" {
            cargo build --release --features stable --no-default-features --package explorer_command_injector --target $target
        }
        "preview" {
            cargo build --release --features preview --no-default-features --package explorer_command_injector --target $target
        }
        default {
            cargo build --release --package explorer_command_injector --target $target
        }
    }
    Copy-Item -Path ".\$CargoOutDir\explorer_command_injector.dll" -Destination "$innoDir\tehanu_explorer_command_injector.dll" -Force
}

function ZipTehanuAndItsFriendsDebug {
    $items = @(
        ".\$CargoOutDir\tehanu.pdb",
        ".\$CargoOutDir\cli.pdb",
        ".\$CargoOutDir\explorer_command_injector.pdb"
    )

    Compress-Archive -Path $items -DestinationPath ".\$CargoOutDir\tehanu-$env:RELEASE_VERSION-$env:TEHANU_RELEASE_CHANNEL.dbg.zip" -Force
}

function MakeAppx {
    switch ($channel) {
        "stable" {
            $manifestFile = "$env:TEHANU_WORKSPACE\crates\explorer_command_injector\AppxManifest.xml"
        }
        "preview" {
            $manifestFile = "$env:TEHANU_WORKSPACE\crates\explorer_command_injector\AppxManifest-Preview.xml"
        }
        default {
            $manifestFile = "$env:TEHANU_WORKSPACE\crates\explorer_command_injector\AppxManifest-Nightly.xml"
        }
    }
    Copy-Item -Path "$manifestFile" -Destination "$innoDir\make_appx\AppxManifest.xml"
    # Add makeAppx.exe to Path
    $sdk = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64"
    $env:Path += ';' + $sdk
    makeAppx.exe pack /d "$innoDir\make_appx" /p "$innoDir\tehanu_explorer_command_injector.appx" /nv
}

function SignTehanuAndItsFriends {
    if (-not $env:CI) {
        return
    }

    $files = "$innoDir\Tehanu.exe,$innoDir\cli.exe,$innoDir\tehanu_explorer_command_injector.dll,$innoDir\tehanu_explorer_command_injector.appx"
    & "$innoDir\sign.ps1" $files
}

function DownloadAMDGpuServices {
    # If you update the AGS SDK version, please also update the version in `crates/gpui/src/platform/windows/directx_renderer.rs`
    $url = "https://codeload.github.com/GPUOpen-LibrariesAndSDKs/AGS_SDK/zip/refs/tags/v6.3.0"
    $zipPath = ".\AGS_SDK_v6.3.0.zip"
    # Download the AGS SDK zip file
    Invoke-WebRequest -Uri $url -OutFile $zipPath
    # Extract the AGS SDK zip file
    Expand-Archive -Path $zipPath -DestinationPath "." -Force
}

function DownloadConpty {
    $url = "https://github.com/microsoft/terminal/releases/download/v1.23.12811.0/Microsoft.Windows.Console.ConPTY.1.23.251008001.nupkg"
    $zipPath = ".\Microsoft.Windows.Console.ConPTY.1.23.251008001.nupkg"
    Invoke-WebRequest -Uri $url -OutFile $zipPath
    Expand-Archive -Path $zipPath -DestinationPath ".\conpty" -Force
}

function CollectFiles {
    Move-Item -Path "$innoDir\tehanu_explorer_command_injector.appx" -Destination "$innoDir\appx\tehanu_explorer_command_injector.appx" -Force
    Move-Item -Path "$innoDir\tehanu_explorer_command_injector.dll" -Destination "$innoDir\appx\tehanu_explorer_command_injector.dll" -Force
    Move-Item -Path "$innoDir\cli.exe" -Destination "$innoDir\bin\tehanu.exe" -Force
    Move-Item -Path "$innoDir\tehanu.sh" -Destination "$innoDir\bin\tehanu" -Force
    if($Architecture -eq "aarch64") {
        New-Item -Type Directory -Path "$innoDir\arm64" -Force
        Move-Item -Path ".\conpty\build\native\runtimes\arm64\OpenConsole.exe" -Destination "$innoDir\arm64\OpenConsole.exe" -Force
        Move-Item -Path ".\conpty\runtimes\win-arm64\native\conpty.dll" -Destination "$innoDir\conpty.dll" -Force
    }
    else {
        New-Item -Type Directory -Path "$innoDir\x64" -Force
        New-Item -Type Directory -Path "$innoDir\arm64" -Force
        Move-Item -Path ".\AGS_SDK-6.3.0\ags_lib\lib\amd_ags_x64.dll" -Destination "$innoDir\amd_ags_x64.dll" -Force
        Move-Item -Path ".\conpty\build\native\runtimes\x64\OpenConsole.exe" -Destination "$innoDir\x64\OpenConsole.exe" -Force
        Move-Item -Path ".\conpty\build\native\runtimes\arm64\OpenConsole.exe" -Destination "$innoDir\arm64\OpenConsole.exe" -Force
        Move-Item -Path ".\conpty\runtimes\win-x64\native\conpty.dll" -Destination "$innoDir\conpty.dll" -Force
    }
}

function BuildInstaller {
    $issFilePath = "$innoDir\tehanu.iss"
    switch ($channel) {
        "stable" {
            $appId = "{{E62BA84E-40DF-471F-97EF-B85924F488FB}"
            $appIconName = "app-icon"
            $appName = "Tehanu"
            $appDisplayName = "Tehanu"
            $appSetupName = "Tehanu-$Architecture"
            # The mutex name here should match the mutex name in crates\tehanu\src\tehanu\windows_only_instance.rs
            $appMutex = "Tehanu-Stable-Instance-Mutex"
            $appExeName = "Tehanu"
            $regValueName = "Tehanu"
            $appUserId = "Tehanu.Tehanu"
            $appShellNameShort = "T&ehanu"
            $appAppxFullName = "Tehanu.Tehanu_1.0.0.0_neutral__mspublisherid"
        }
        "preview" {
            $appId = "{{85A6F569-DD2C-4850-B9E7-4FAC667B0D0C}"
            $appIconName = "app-icon-preview"
            $appName = "Tehanu Preview"
            $appDisplayName = "Tehanu Preview"
            $appSetupName = "Tehanu-$Architecture"
            # The mutex name here should match the mutex name in crates\tehanu\src\tehanu\windows_only_instance.rs
            $appMutex = "Tehanu-Preview-Instance-Mutex"
            $appExeName = "Tehanu"
            $regValueName = "TehanuPreview"
            $appUserId = "Tehanu.Tehanu.Preview"
            $appShellNameShort = "T&ehanu Preview"
            $appAppxFullName = "Tehanu.Tehanu.Preview_1.0.0.0_neutral__mspublisherid"
        }
        "nightly" {
            $appId = "{{A57C51AA-9E45-403E-A0E0-6D4DA22FACF6}"
            $appIconName = "app-icon-nightly"
            $appName = "Tehanu Nightly"
            $appDisplayName = "Tehanu Nightly"
            $appSetupName = "Tehanu-$Architecture"
            # The mutex name here should match the mutex name in crates\tehanu\src\tehanu\windows_only_instance.rs
            $appMutex = "Tehanu-Nightly-Instance-Mutex"
            $appExeName = "Tehanu"
            $regValueName = "TehanuNightly"
            $appUserId = "Tehanu.Tehanu.Nightly"
            $appShellNameShort = "T&ehanu Editor Nightly"
            $appAppxFullName = "Tehanu.Tehanu.Nightly_1.0.0.0_neutral__mspublisherid"
        }
        "dev" {
            $appId = "{{4FEF353A-EA46-468C-95DD-2B343A71416F}"
            $appIconName = "app-icon-dev"
            $appName = "Tehanu Dev"
            $appDisplayName = "Tehanu Dev"
            $appSetupName = "Tehanu-$Architecture"
            # The mutex name here should match the mutex name in crates\tehanu\src\tehanu\windows_only_instance.rs
            $appMutex = "Tehanu-Dev-Instance-Mutex"
            $appExeName = "Tehanu"
            $regValueName = "TehanuDev"
            $appUserId = "Tehanu.Tehanu.Dev"
            $appShellNameShort = "T&ehanu Dev"
            $appAppxFullName = "Tehanu.Tehanu.Dev_1.0.0.0_neutral__mspublisherid"
        }
        default {
            Write-Error "can't bundle installer for $channel."
            exit 1
        }
    }

    # Windows runner 2022 default has iscc in PATH, https://github.com/actions/runner-images/blob/main/images/windows/Windows2022-Readme.md
    # Currently, we are using Windows 2022 runner.
    # Windows runner 2025 doesn't have iscc in PATH for now, https://github.com/actions/runner-images/issues/11228
    $innoSetupPath = "C:\Program Files (x86)\Inno Setup 6\ISCC.exe"

    $definitions = @{
        "AppId"          = $appId
        "AppIconName"    = $appIconName
        "OutputDir"      = "$env:TEHANU_WORKSPACE\target"
        "AppSetupName"   = $appSetupName
        "AppName"        = $appName
        "AppDisplayName" = $appDisplayName
        "RegValueName"   = $regValueName
        "AppMutex"       = $appMutex
        "AppExeName"     = $appExeName
        "ResourcesDir"   = "$innoDir"
        "ShellNameShort" = $appShellNameShort
        "AppUserId"      = $appUserId
        "Version"        = "$env:RELEASE_VERSION"
        "SourceDir"      = "$env:TEHANU_WORKSPACE"
        "AppxFullName"   = $appAppxFullName
    }

    $defs = @()
    foreach ($key in $definitions.Keys) {
        $defs += "/d$key=`"$($definitions[$key])`""
    }

    $innoArgs = @($issFilePath) + $defs
    if($env:CI) {
        $signTool = "powershell.exe -ExecutionPolicy Bypass -File $innoDir\sign.ps1 `$f"
        $innoArgs += "/sDefaultsign=`"$signTool`""
    }

    # Execute Inno Setup
    Write-Host "🚀 Running Inno Setup: $innoSetupPath $innoArgs"
    $process = Start-Process -FilePath $innoSetupPath -ArgumentList $innoArgs -NoNewWindow -Wait -PassThru

    if ($process.ExitCode -eq 0) {
        Write-Host "✅ Inno Setup successfully compiled the installer"
        Write-Output "SETUP_PATH=target/$appSetupName.exe" >> $env:GITHUB_ENV
        $script:buildSuccess = $true
    }
    else {
        Write-Host "❌ Inno Setup failed: $($process.ExitCode)"
        $script:buildSuccess = $false
    }
}

ParseTehanuWorkspace
$innoDir = "$env:TEHANU_WORKSPACE\inno\$Architecture"
$debugArchive = "$CargoOutDir\tehanu-$env:RELEASE_VERSION-$env:TEHANU_RELEASE_CHANNEL.dbg.zip"
$debugStoreKey = "$env:TEHANU_RELEASE_CHANNEL/tehanu-$env:RELEASE_VERSION-$env:TEHANU_RELEASE_CHANNEL.dbg.zip"

CheckEnvironmentVariables
PrepareForBundle
GenerateLicenses
BuildTehanuAndItsFriends
MakeAppx
SignTehanuAndItsFriends
ZipTehanuAndItsFriendsDebug
DownloadAMDGpuServices
DownloadConpty
CollectFiles
BuildInstaller

if ($buildSuccess) {
    Write-Output "Build successful"
    if ($Install) {
        Write-Output "Installing Tehanu..."
        Start-Process -FilePath "$env:TEHANU_WORKSPACE/target/TehanuEditorUserSetup-x64-$env:RELEASE_VERSION.exe"
    }
    exit 0
}
else {
    Write-Output "Build failed"
    exit 1
}
