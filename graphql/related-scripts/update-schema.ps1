function Get-IsCommandInPath($Command) {
    if ($null -eq (Get-Command "$Command" -ErrorAction SilentlyContinue)) {
        Write-Host "Unable to find $Command in your PATH"
        return 0
    } else {
        return 1
    }
}

function Get-IsWebServerActive($Url) {
    Write-Output "Called Get-WebServer-Up"
    try{
        Invoke-WebRequest -Method Post -uri $Url

        Write-Out "got req"
        return 0;
    } catch {
        return 1;
    }
}

$curlCommandFound=Get-IsCommandInPath -Command "curl"

if (-Not $curlCommandFound) {
    Exit
} 

Write-Output "Found Curl, begining to update schema."

$workingDir = Join-Path -Path "$PSScriptRoot" -ChildPath ".." -Resolve
$schemaPath = $workingDir = Join-Path -Path "$workingDir" -ChildPath ".\schema.graphql" -Resolve
$graphqlUrl = "http://localhost:8123/graphql"
$adminUrl = "http://localhost:8123/admin/graphql"

Write-Output "Checking for webserver at $graphqlUrl"
$isWebServerActive = Get-IsWebServerActive -Url $graphqlUrl

if (-Not ("$isWebServerActive" -eq "0")) {
    Write-Output "Failed to find $graphqlUrl are you sure you port forwarded on kube? or are running the dgraph/standalone container?"
    Exit
}

Write-Output "Found webserver! :D"
