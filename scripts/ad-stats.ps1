$expired = (get-aduser -filter * -properties *|Where-Object {$_.Enabled -and $_.PasswordExpired} |Measure-Object).Count
$locked = (get-aduser -filter * -properties *|Where-Object {$_.Enabled -and $_.LockedOut} |Measure-Object).Count
$date = get-date
"$date, $expired,$locked" | Out-File data/ad-stats.txt -Append
write-host $date, $expired,$locked

