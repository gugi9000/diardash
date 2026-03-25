$expired = (get-aduser -filter * -properties *|where {$_.Enabled -and $_.PasswordExpired} |measure).Count
$locked = (get-aduser -filter * -properties *|where {$_.Enabled -and $_.LockedOut} |measure).Count
$date = get-date
"$date, $expired,$locked" | Out-File data/ad-stats.txt -Append
write-host $date, $expired,$locked

