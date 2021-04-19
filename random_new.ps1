param (
	[Parameter(Mandatory=$true)][string]$filename,
	[Parameter(Mandatory=$true)][int]$maximum,
	[Parameter(Mandatory=$true)][int]$count,
	[int]$guarantee = 0,
	[switch]$worstcase = $false
)

# ./random_sets.ps1 [file name] [max] [entry count] || -guarantee -worstcase

# set guarantee to the number of addends to guarantee a correct solution

# Generate (count - num_of_addends) lines of pure random data
$count -= $guarantee

$TempFile = Get-Random -Minimum 1 -Maximum ($maximum + 1) -Count $count

# Maximum value for each solution. +1 to account for zero-index
# +1 to account for the first addend itself
# I.e. if you had 4 addends you would need at least a balance
# of 3 after the first iteration.

$Balance = ($maximum - $guarantee + 2)
$Solutions = @()
$SolutionLines = @()

while ($SolutionLines.count -ne $guarantee) {
	$RandLine = Get-Random -Minimum 1 -Maximum ($count + 1)
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

while ($Solutions.count -ne $guarantee) {
	$Value = Get-Random -Minimum 1 -Maximum $Balance

	# Assume Get-Random picks the max value (balance)
	# then the new balance should equal at least 1
	# for the remaining values.
	# (Don't worry, we subtracted these values earlier
	# and are now just adding them back in one by one.

	$Balance = ($Balance - $Value + 1)

	$Solutions += $Value
}
	
$index = 0
$LineNumber = 0

foreach ($line in $TempFile) {
	foreach ($SLine in $SolutionLines) {
		if ($SLine -eq $LineNumber) {
			$File += $Solutions[$index]
			$index ++
		}
	}

$File += $line	
$LineNumber ++
}

$File | Out-File -FilePath $filename
