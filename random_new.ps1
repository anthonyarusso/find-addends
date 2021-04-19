param (
	[Parameter(Mandatory=$true)][string]$filename,
	[Parameter(Mandatory=$true)][int]$maximum,
	[Parameter(Mandatory=$true)][int]$count,
	[switch]$guarantee = $false,
	[switch]$worstcase = $false
)

# ./random_sets.ps1 [file name] [max] [entry count] || -guarantee -worstcase

Get-Random -Maximum $maximum -Count $count | Out-File -FilePath $filename
