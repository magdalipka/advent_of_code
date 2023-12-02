param (
    [string]$day_no = ""
)

if ($day_no -eq "" -Or $day_no.Length -gt 2 ) {
  Write-Host "Incorrect day number provided $keys";
  exit 1;
}

if($day_no.Length -eq 1) {
  $day_no = "0" + $day_no
}

$dayPath = $pwd.Path + "\day" + $day_no

cp -r 'day_template/' $dayPath;


Get-ChildItem $dayPath -Recurse -include *.* | ForEach-Object {
  # $filepath = $pwd.Path + "\new-day\" + $_
  Write-Host $_
  (Get-Content $_).Replace('{day_no}', $day_no) | Set-Content $_
}
