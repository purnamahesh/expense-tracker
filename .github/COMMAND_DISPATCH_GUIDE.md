# PR Command Dispatch Guide

Quick reference for using slash commands in pull requests.

## Available Commands

### `/pre-release [TYPE]`
Trigger a pre-release build and publish to crates.io.

**Arguments:**
- `TYPE` (optional): `alpha`, `beta`, or `rc` (default: `alpha`)

**Examples:**
```bash
/pre-release alpha
/pre-release beta
/pre-release rc
/pre-release  # defaults to alpha
```

**What it does:**
1. Checks for Rust file changes in the PR
2. Bumps version with pre-release suffix (e.g., `1.0.0-alpha.0`, `1.0.0-beta.0`, `1.0.0-rc.0`)
3. Commits version bump to PR branch
4. Creates git tag
5. Publishes to crates.io
6. Comments back with results

---

### `/test [OPTIONS]`
Run the test suite on the PR branch.

**Arguments:**
- `rust-version=VERSION` (optional): Rust version to test with
  - Options: `stable`, `beta`, `nightly`
  - Default: `stable`
- `run-coverage=BOOL` (optional): Whether to generate coverage report
  - Options: `true`, `false`
  - Default: `true`

**Examples:**
```bash
/test
/test rust-version=nightly
/test rust-version=beta run-coverage=false
```

**What it does:**
1. Runs cargo fmt check
2. Runs cargo clippy
3. Runs all tests (unit, integration, doc)
4. Generates coverage report (if enabled)
5. Comments back with test results

---

### `/docs`
Generate and commit documentation.

**Arguments:** None

**Example:**
```bash
/docs
```

**What it does:**
1. Builds Rust documentation with `cargo doc`
2. Generates HTML docs
3. Commits to PR branch
4. Updates docs/ directory
5. Comments back with status

---

## Permissions

Commands require one of the following permission levels:
- **write** - Can push to the repository
- **maintain** - Repository maintainer
- **admin** - Repository administrator

If you don't have permissions, you'll receive a ❌ reaction and error message.

---

## Feedback

The command dispatcher provides real-time feedback:

### Valid Command
- 🚀 Reaction on your comment
- New comment with workflow details and link

### Invalid Command
- ❓ Reaction on your comment
- Error message with list of valid commands

### Permission Denied
- ❓ Reaction on your comment
- Error message explaining permission requirements

---

## Examples

### Complete Pre-Release Flow

```bash
# 1. Create a PR with changes
git checkout -b feature/new-awesome-feature
# ... make changes ...
git push origin feature/new-awesome-feature

# 2. Open PR on GitHub

# 3. Test the changes
# Comment on PR:
/test

# 4. Wait for tests to pass

# 5. Create a pre-release
# Comment on PR:
/pre-release alpha

# 6. Version is bumped and published!
```

### Testing Different Rust Versions

```bash
# Test on stable (default)
/test

# Test on nightly
/test rust-version=nightly

# Test without coverage
/test run-coverage=false
```

### Quick Documentation Update

```bash
# After updating doc comments
/docs
```

---

## Troubleshooting

### Command Not Working?

1. **Check permissions:** Do you have write access?
2. **Check syntax:** Make sure command starts with `/`
3. **Check branch:** Are you commenting on a PR?
4. **Check Actions:** Look at the Actions tab for errors

### Command Triggered But Failed?

1. Check the workflow run in Actions tab
2. Look for error messages in the workflow logs
3. Common issues:
   - Missing `CARGO_REGISTRY_TOKEN` secret
   - Test failures
   - Lint/format errors

### Need Help?

- Check workflow logs in the Actions tab
- Review the workflow files in `.github/workflows/`
- See workflow documentation in the main [README.md](../README.md#github-actions-workflows)

---

## Advanced: Adding Custom Commands

Want to add your own commands? Edit `.github/workflows/command-dispatch.yml`:

1. Add command to `VALID_COMMANDS` list:
```bash
VALID_COMMANDS="pre-release test docs bench"  # Added 'bench'
```

2. Add command help to error message:
```yaml
body: `❌ Unknown command: \`/${{ steps.parse.outputs.command }}\`\n\nAvailable commands:\n- \`/pre-release [alpha|beta|rc]\`\n- \`/test [rust-version=stable|beta|nightly]\`\n- \`/docs\`\n- \`/bench\`  # Add your command
```

3. Add new job for the command:
```yaml
  bench:
    name: Benchmark
    needs: parse-command
    if: |
      needs.parse-command.outputs.is-valid == 'true' &&
      needs.parse-command.outputs.command == 'bench'
    runs-on: ubuntu-latest
    steps:
      # Your benchmark workflow trigger logic here
```

---

**Pro Tip:** Use tab completion in GitHub's comment box - start typing `/` and GitHub will suggest available commands (if configured as slash commands in the repository settings).
