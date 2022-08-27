$KEY = $args[0]

function Invoke-Cargo {
    param (
        $sub_command,
        $is_bin
    )
    $manifest = Get-ChildItem -Path . -Recurse -Name -Include Cargo.toml -Depth 2
    
    if ($is_bin) {
        cargo $sub_command
    }
    else {
        cargo $sub_command --manifest-path $manifest
    }
}

function Invoke-pnpm {
    param (
        $sub_command
    )
    pnpm $sub_command
}

switch ($KEY) {
    ccl { Invoke-Cargo("clean") }
    ct { Invoke-Cargo("test") }
    cb { Invoke-Cargo("build") }
    cbr { Invoke-Cargo("build --release") }
    cck { Invoke-Cargo("check") }
    ctd { Invoke-Cargo("tauri dev", 1) }
    ctb { Invoke-Cargo("tauri build", 1) }
    
    pc { Invoke-pnpm("check") }
    pfl {
        Invoke-pnpm("format")
        Invoke-pnpm("lint")
    }
    pd { Invoke-pnpm("dev") }
    pb { Invoke-pnpm("build") }
    Default {
        if ($KEY) {
            Write-Error "Given key is invalid"
        }
        else {
            Write-Error "No key given to invoke utility functions"
        }
    }
}