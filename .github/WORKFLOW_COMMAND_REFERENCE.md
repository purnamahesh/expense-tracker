# Workflow Command Reference

Quick reference for all PR comment commands and workflow triggers.

## Command Format

All PR comment commands follow this pattern:
```
/command [arguments]
```

**No `--flag=value` syntax** - use simple space-separated arguments.

---

## Available Commands

### `/pre-release [TYPE]`

**Format:**
```bash
/pre-release alpha
/pre-release beta
/pre-release rc
/pre-release        # defaults to alpha
```

**What it triggers:**
- Workflow: `pre-release.yml`
- Input: `prerelease-type` = `alpha`, `beta`, or `rc`

**Flow:**
```
PR Comment → command-dispatch.yml → pre-release.yml (workflow_dispatch)
```

---

### `/test [OPTION=VALUE ...]`

**Format:**
```bash
/test
/test rust-version=nightly
/test run-coverage=false
/test rust-version=beta run-coverage=false
```

**What it triggers:**
- Workflow: `test.yml`
- Inputs:
  - `skip_tests` = `false` (always)
  - `run_coverage` = `true` or `false`

**Flow:**
```
PR Comment → command-dispatch.yml → test.yml (workflow_dispatch)
```

---

### `/docs`

**Format:**
```bash
/docs
```

**What it triggers:**
- Workflow: `docs.yml`
- No inputs

**Flow:**
```
PR Comment → command-dispatch.yml → docs.yml (workflow_dispatch)
```

---

## Workflow Trigger Methods

Each workflow can be triggered in multiple ways:

### `pre-release.yml`
1. ✅ **PR Comment:** `/pre-release [TYPE]` → via `command-dispatch.yml`
2. ✅ **Manual:** GitHub Actions UI → Run workflow → Select type
3. ❌ **Direct comment:** Not supported (removed legacy `issue_comment` trigger)

### `test.yml`
1. ✅ **PR Comment:** `/test [OPTIONS]` → via `command-dispatch.yml`
2. ✅ **PR Events:** Automatically on PR open/sync/reopen
3. ✅ **Manual:** GitHub Actions UI → Run workflow

### `docs.yml`
1. ✅ **PR Comment:** `/docs` → via `command-dispatch.yml`
2. ✅ **Push to main/master:** Automatically when src/ changes
3. ✅ **PR Events:** Automatically on PR with src/ changes
4. ✅ **Manual:** GitHub Actions UI → Run workflow

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  PR Comment: /command                   │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              command-dispatch.yml                        │
│  • Parse command and arguments                          │
│  • Validate user permissions                            │
│  • Extract parameters                                   │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│          Target Workflow (workflow_dispatch)            │
│  • pre-release.yml                                      │
│  • test.yml                                            │
│  • docs.yml                                            │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              Comment back to PR                         │
│  • Status update                                        │
│  • Link to workflow run                                │
└─────────────────────────────────────────────────────────┘
```

---

## Permission Requirements

All commands require one of these permission levels:
- **write** - Can push to repository
- **maintain** - Repository maintainer
- **admin** - Repository administrator

If user lacks permissions:
- Comment gets ❓ reaction
- Error message posted explaining requirements

---

## Version Formats

All pre-release versions follow **SemVer specification**:

```
MAJOR.MINOR.PATCH-PRERELEASE.NUMBER

Examples:
1.0.0-alpha.0
1.0.0-beta.1
1.0.0-rc.2
```

**Not used (old format):**
```
❌ 1.0.0a0
❌ 1.0.0b1
❌ 1.0.0rc2
```

---

## Troubleshooting

### Command not recognized?

Check:
1. ✅ Command starts with `/`
2. ✅ You're commenting on a PR (not an issue)
3. ✅ You have write/maintain/admin permissions
4. ✅ Command is in valid commands list:
   - `/pre-release`
   - `/test`
   - `/docs`

### Pre-release skipped?

Check:
1. ✅ PR has Rust file (`.rs`) changes
2. ✅ `CARGO_REGISTRY_TOKEN` secret is configured
3. ✅ Branch has no uncommitted changes

### Test not running?

Check:
1. ✅ PR has Rust file changes (or use `/test` to force)
2. ✅ Tests not disabled via:
   - Commit message `[skip tests]`
   - Repository variable `DISABLE_TESTS=true`

---

## Implementation Details

### Command Parsing (command-dispatch.yml)

```yaml
# Extract command
COMMAND=$(echo "$FIRST_LINE" | awk '{print $1}' | sed 's/^\///')

# Extract first unnamed argument
TYPE=$(echo "$ARGS_JSON" | jq -r '.unnamed[0] // "alpha"')

# Extract named arguments
RUST_VERSION=$(echo "$ARGS_JSON" | jq -r '.named["rust-version"] // "stable"')
```

### Workflow Dispatch

```yaml
await github.rest.actions.createWorkflowDispatch({
  owner: context.repo.owner,
  repo: context.repo.repo,
  workflow_id: 'pre-release.yml',
  ref: '${{ needs.parse-command.outputs.pr-branch }}',
  inputs: {
    'prerelease-type': '${{ steps.args.outputs.type }}'
  }
});
```

---

## Changes from Legacy Format

### Before (Old):
```bash
/pre-release --type=alpha
```

### After (Current):
```bash
/pre-release alpha
```

### Why Changed:
1. ✅ Simpler syntax
2. ✅ More consistent with common CLI patterns
3. ✅ Easier to parse
4. ✅ Matches GitHub's native slash command style
5. ✅ No need for direct `issue_comment` triggers

---

## See Also

- [COMMAND_DISPATCH_GUIDE.md](COMMAND_DISPATCH_GUIDE.md) - User-facing command guide
- [README.md](../README.md#github-actions-workflows) - Full workflow documentation
- [command-dispatch.yml](../workflows/command-dispatch.yml) - Command parser implementation
