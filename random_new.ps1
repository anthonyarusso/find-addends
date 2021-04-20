param (
	[Parameter(Mandatory=$true)][string]$filename,
	[Parameter(Mandatory=$true)][int]$maximum,
	[Parameter(Mandatory=$true)][int]$count,
	[int]$guarantee = 0,
	[switch]$worstcase = $false
)

# ./random_sets.ps1 [file name] [max] [entry count] || -guarantee -worstcase

# Generate (count - num_of_addends) lines of pure random data

$TempFile = Get-Random -Minimum 1 -Maximum ($maximum + 1) -Count ($count - 1)

$Balance = ($maximum - $guarantee + 1)
$Solutions = @()
$SolutionLines = @()

while ($SolutionLines.count -lt $guarantee) {
	$RandLine = Get-Random -Minimum 1 -Maximum $count
	$Unique = $true

	# If the line number does not already exist in
	# $SolutionLines, append it.
	foreach ($SLine in $SolutionLines) {
		if ($SLine -eq $RandLine) {
			$Unique = $false
			break
		}
	}
	if ($Unique) {
		$SolutionLines += $RandLine
	}
}

while ($Solutions.count -lt $guarantee) {
	if (($guarantee - $Solutions.count) -eq 1) {
		# If we are on the last addend, assign the remaining balance
		$Value = $Balance
	} else {
		$Value = Get-Random -Minimum 1 -Maximum $Balance
	}

	# Assume Get-Random picks the max value (balance)
	# then the new balance should equal at least 1
	# for the remaining values.
	# (Don't worry, we subtracted these values earlier
	# and are now just adding them back in one by one.

	$Solutions += $Value
	
	$Balance = ($Balance - $Value + 1)
}
	
$File = New-Item -ItemType "file" -Path . -Name $filename -Force
$index = 0
$LineNumber = 0

foreach ($line in $TempFile) {
	foreach ($SLine in $SolutionLines) {
		if ($SLine -eq $LineNumber) {
			$Solutions[$index] | Out-File -FilePath .\$filename -Append

			$elem = $Solutions[$index]
			$LineAdjust = $SLine + $index
			$checksum += $elem
			write-output "Solution $elem on line $LineAdjust"

			$index ++
		}
	}

$line | Out-File -FilePath .\$filename -Append
$LineNumber ++
}
