# create_component.ps1

param (
    [Parameter(Mandatory=$true)]
    [string]$ComponentName
)

# Bileşen klasörünü oluştur
$componentDir = "src/components/$ComponentName"
if (-Not (Test-Path -Path $componentDir)) {
    New-Item -ItemType Directory -Force -Path $componentDir | Out-Null
} else {
    Write-Output "Klasör zaten mevcut: $componentDir"
}

# Rust dosyası oluştur
$rustContent = @"
use yew::prelude::*;

#[function_component($ComponentName)]
pub fn $($ComponentName.ToLower())() -> Html {
    html! {
        <div class=\"$($ComponentName.ToLower())\">
            <p>{ \"$ComponentName Bileşeni\" }</p>
        </div>
    }
}
"@
$rustFilePath = "$componentDir/$ComponentName.rs"
Set-Content -Path $rustFilePath -Value $rustContent
Write-Output "Rust dosyası oluşturuldu: $rustFilePath"

# SCSS dosyası oluştur
$scssContent = @"
.$($ComponentName.ToLower()) {
    /* $ComponentName bileşen stilini buraya ekleyin */
}
"@
$scssFilePath = "$componentDir/$ComponentName.scss"
Set-Content -Path $scssFilePath -Value $scssContent
Write-Output "SCSS dosyası oluşturuldu: $scssFilePath"

# mod.rs dosyasını güncelle
$modPath = "src/components/mod.rs"
$useStatement = "pub mod $ComponentName;`npub use $ComponentName::$ComponentName;"
if (-not (Select-String -Path $modPath -Pattern "pub mod $ComponentName;")) {
    Add-Content -Path $modPath -Value "`n$useStatement"
    Write-Output "mod.rs dosyasına ekleme yapıldı: $ComponentName"
} else {
    Write-Output "mod.rs dosyasında zaten mevcut: $ComponentName"
}

# index.html içinde SCSS dosyasını dahil et
$indexPath = "index.html"
$linkTag = "<link data-trunk rel=`"scss`" href=`"components/$ComponentName/$ComponentName.scss`" />"
if (-not (Select-String -Path $indexPath -Pattern [regex]::Escape($linkTag))) {
    # `<head>` etiketinden sonra ekle
    $indexContent = Get-Content -Path $indexPath
    $newContent = $indexContent -replace '(</head>)', "    $linkTag`n`$1"
    Set-Content -Path $indexPath -Value $newContent
    Write-Output "index.html dosyasına SCSS linki eklendi: $linkTag"
} else {
    Write-Output "index.html dosyasında zaten mevcut: $linkTag"
}

Write-Output "Bileşen '$ComponentName' başarıyla oluşturuldu."
