$ErrorActionPreference = 'Stop'

$repoRoot = if ($env:ADL_REPO_ROOT) { $env:ADL_REPO_ROOT } else { Join-Path $env:USERPROFILE 'git\agent-design-language' }
$repoName = Split-Path -Path $repoRoot -Leaf
$repoPresent = Test-Path $repoRoot
$gitBranch = 'unknown'
$gitCommitShort = 'unknown'
$ssmAgentInstalled = Test-Path 'C:\Program Files\Amazon\SSM\amazon-ssm-agent.exe'

if ($repoPresent -and (Get-Command git -ErrorAction SilentlyContinue)) {
  try {
    $inside = (& git -C $repoRoot rev-parse --is-inside-work-tree 2>$null)
    if ($inside -eq 'true') {
      $gitBranch = ((& git -C $repoRoot rev-parse --abbrev-ref HEAD 2>$null) | Select-Object -First 1)
      $gitCommitShort = ((& git -C $repoRoot rev-parse --short HEAD 2>$null) | Select-Object -First 1)
    }
  } catch {
  }
}

$payload = [ordered]@{
  schema_version = 'adl.local_polis_status.v1'
  generated_at_utc = [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ')
  host_label = $env:COMPUTERNAME
  os_name = 'Windows'
  os_version = [System.Environment]::OSVersion.Version.ToString()
  repo_name = $repoName
  repo_present = $repoPresent
  git_branch = $gitBranch
  git_commit_short = $gitCommitShort
  ssm_agent_installed = $ssmAgentInstalled
}

$payload | ConvertTo-Json -Depth 3
