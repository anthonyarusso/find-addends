# ./random_sets.ps1 [out-dir] [sets count] [max] [entry count]

if (-not $args) { Write-Output "Usage: ./random_sets.ps1 [out-dir] [sets count] [max value] [entry count]" }

for ($i = 0; $i -lt $args[1]; $i++) {
	$path = "$($args[0])\$i.txt"
	Get-Random -Maximum $args[2] -Count $args[3] | Out-File -FilePath $path
}
