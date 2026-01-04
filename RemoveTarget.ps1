# Set the path to your master folder
$masterFolder = "C:\Users\soumamon\OneDrive - Capgemini\Self-Code-Test\Rust\W3-Practice-Rust"

# Get all 'target' directories recursively
$targetDirs = Get-ChildItem -Path $masterFolder -Recurse -Directory -Filter "target"

# Loop through and remove each target folder
foreach ($dir in $targetDirs) {
    try {
        Remove-Item -Path $dir.FullName -Recurse -Force
        Write-Host "Deleted: $($dir.FullName)"
    }
    catch {
        Write-Warning "Failed to delete: $($dir.FullName). Error: $_"
    }
}