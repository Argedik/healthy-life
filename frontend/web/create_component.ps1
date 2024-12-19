Param(
    [Parameter(Mandatory = $true)]
    [string]$DosyaIsmi,
    
    [Parameter(Mandatory = $true)]
    [string]$KlasorIsmi
)

$ProjectRoot = "C:/Users/enes.gedik/Desktop/fe/besin-uygulamasi/frontend/web"
$ScreensPath = Join-Path $ProjectRoot "src/screens"
$TargetFolder = Join-Path $ScreensPath $KlasorIsmi

$htmlFile = Join-Path $TargetFolder "$DosyaIsmi.html"
$scssFile = Join-Path $TargetFolder "$DosyaIsmi.scss"
$rsFile = Join-Path $TargetFolder "$DosyaIsmi.rs"
$modFile = Join-Path $TargetFolder "mod.rs"
$rootModFile = Join-Path $ScreensPath "mod.rs"
$screensScssFile = Join-Path $ScreensPath "screens.scss"

# Klasör oluştur
if (!(Test-Path $TargetFolder)) {
    Write-Host "[$KlasorIsmi] klasörü bulunamadı, oluşturuluyor..."
    New-Item -ItemType Directory -Path $TargetFolder | Out-Null
} else {
    Write-Host "[$KlasorIsmi] klasörü zaten mevcut."
}

Write-Host "[$KlasorIsmi] dizinine giriliyor..."
Set-Location $TargetFolder

# HTML Dosyası
if (Test-Path $htmlFile) {
    Write-Host "[$DosyaIsmi.html] dosyası zaten mevcut."
} else {
    $htmlContent = @"
<div class="$($DosyaIsmi)-container">
    <h1>$DosyaIsmi Component</h1>
</div>
"@
    $htmlContent | Out-File $htmlFile -Encoding UTF8
    Write-Host "[$DosyaIsmi.html] dosyası oluşturuldu."
}

# SCSS Dosyası
if (Test-Path $scssFile) {
    Write-Host "[$DosyaIsmi.scss] dosyası zaten mevcut."
} else {
    $scssContent = @"
.$($DosyaIsmi)-container {
    background-color: #f0f0f0;
    padding: 20px;
    h1 {
        color: #333;
    }
}
"@
    $scssContent | Out-File $scssFile -Encoding UTF8
    Write-Host "[$DosyaIsmi.scss] dosyası oluşturuldu."
}

# RS Dosyası
if (Test-Path $rsFile) {
    Write-Host "[$DosyaIsmi.rs] dosyası zaten mevcut."
} else {
    # Dosya ismini fonksiyon adına küçük harfle çeviriyoruz.
    $fnName = ($DosyaIsmi.ToLower() + "_component")

    $rsContent = @"
use yew::prelude::*;

#[function_component($DosyaIsmi)]
pub fn $fnName() -> Html {
    let html_content = include_str!("$DosyaIsmi.html"); 
    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
        </div>
    }
}
"@
    $rsContent | Out-File $rsFile -Encoding UTF8
    Write-Host "[$DosyaIsmi.rs] dosyası oluşturuldu."
}

# mod.rs düzenle
if (!(Test-Path $modFile)) {
    Write-Host "[$modFile] bulunamadı, oluşturuluyor..."
    New-Item -ItemType File -Path $modFile | Out-Null
}

$modContent = (Get-Content $modFile)
$pubModLine = "pub mod $DosyaIsmi;"
$pubUseLine = "pub use $DosyaIsmi::$DosyaIsmi;"

if ($modContent -notcontains $pubModLine) {
    Add-Content $modFile $pubModLine
    Write-Host "mod.rs dosyasına '$pubModLine' eklendi."
} else {
    Write-Host "mod.rs dosyasında '$pubModLine' zaten mevcut."
}

if ($modContent -notcontains $pubUseLine) {
    Add-Content $modFile $pubUseLine
    Write-Host "mod.rs dosyasına '$pubUseLine' eklendi."
} else {
    Write-Host "mod.rs dosyasında '$pubUseLine' zaten mevcut."
}

# screens klasörünün mod.rs dosyasına ekleme
if (!(Test-Path $rootModFile)) {
    Write-Host "[screens/mod.rs] bulunamadı, oluşturuluyor..."
    New-Item -ItemType File -Path $rootModFile | Out-Null
}

$rootModContent = (Get-Content $rootModFile)
$rootPubModLine = "pub mod $KlasorIsmi;"
$rootPubUseLine = "pub use $KlasorIsmi::$DosyaIsmi;"

if ($rootModContent -notcontains $rootPubModLine) {
    Add-Content $rootModFile $rootPubModLine
    Write-Host "screens/mod.rs dosyasına '$rootPubModLine' eklendi."
} else {
    Write-Host "screens/mod.rs dosyasında '$rootPubModLine' zaten mevcut."
}

if ($rootModContent -notcontains $rootPubUseLine) {
    Add-Content $rootModFile $rootPubUseLine
    Write-Host "screens/mod.rs dosyasına '$rootPubUseLine' eklendi."
} else {
    Write-Host "screens/mod.rs dosyasında '$rootPubUseLine' zaten mevcut."
}

# screens.scss içine @use ekle
if (!(Test-Path $screensScssFile)) {
    Write-Host "[screens.scss] bulunamadı, oluşturuluyor..."
    New-Item -ItemType File -Path $screensScssFile | Out-Null
}

$screensScssContent = (Get-Content $screensScssFile)
$useStatement = "@use './$KlasorIsmi/$DosyaIsmi.scss' as *;"
if ($screensScssContent -notcontains $useStatement) {
    Add-Content $screensScssFile $useStatement
    Write-Host "screens.scss dosyasına '@use './$KlasorIsmi/$DosyaIsmi.scss' as *;' eklendi."
} else {
    Write-Host "screens.scss dosyasında '@use './$KlasorIsmi/$DosyaIsmi.scss' as *;' zaten mevcut."
}

Write-Host "İşlem tamamlandı: '$DosyaIsmi' bileşeni '$KlasorIsmi' klasöründe oluşturuldu ya da güncellendi."
