---
name: mkvn-proxy-deploy
description: Full release pipeline for MKVN Proxy Manager — bump version, update CHANGELOG, commit, tag, push, verify CI release
---

# mkvn-proxy-deploy

Complete release pipeline for MKVN Proxy Manager. When triggered, perform all steps below in order.

## Trigger

User says: "deploy", "release", "tạo release", "publish", "bump version", "release mkvn"

## Workflow

### Step 1: Determine Bump Type

Ask user which version to bump to unless they already specified. Options:
- **patch** (0.0.X) — bug fixes, small improvements
- **minor** (0.X.0) — new features
- **major** (X.0.0) — breaking changes

### Step 2: Full Git Diff

```bash
git status --short
git diff --stat
git diff
git diff --cached --stat
git ls-files --others --exclude-standard
git log --oneline -10
git tag --sort=-v:refname | head -5
```

Verify: not detached HEAD, upstream branch exists, `gh` is authenticated.

If > 20 files changed, confirm with user before proceeding.

### Step 3: Find All Version Files

Search for the current version in authoritative files only:

```bash
rg -n --hidden --glob '!.git/**' --glob '!node_modules/**' --glob '!target/**' --glob '!dist/**' --glob '!gen/**' '(version|"version")\s*[:=]\s*"[0-9]+\.[0-9]+\.[0-9]+"' package.json package-lock.json CHANGELOG.md src-tauri/Cargo.toml src-tauri/tauri.conf.json
```

Authoritative version files for this project:
- `package.json` — `"version": "<current>"`
- `package-lock.json` — `"version": "<current>"` (2 occurrences)
- `src-tauri/Cargo.toml` — `version = "<current>"`
- `src-tauri/tauri.conf.json` — `"version": "<current>"`
- `CHANGELOG.md` — `## [v<current>]`

### Step 4: Bump Version

Replace all occurrences of the old version with the new version in all authoritative files (except `CHANGELOG.md` — that gets a new entry, not a replace).

Use the `edit` tool with `replaceAll: true` for:
- `package.json`
- `package-lock.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`

Do NOT use regex replacement. Do exact string matching.

### Step 5: Update CHANGELOG

Generate today's date:
```bash
node -e "console.log(new Date().toISOString().slice(0,10))"
```

Insert a new `## [v<new>] - YYYY-MM-DD` section at the top of `CHANGELOG.md` (below the `# Changelog` heading), with categorized entries:

```
## [v<new>] - YYYY-MM-DD

### Added
- ...

### Changed
- ...

### Fixed
- ...
```

Rules:
- Categorize changes from the git diff (Step 2):
  - New features → `### Added`
  - Bug fixes → `### Fixed`
  - Refactoring, deps, config, perf → `### Changed`
- Preserve all existing entries below
- If version link list exists at bottom of file, add the new version link

### Step 6: Git Add All

```bash
git status --short
git add -A
git status --short
```

### Step 7: Commit

Generate a commit message in this format:

```
v<new>: <short summary>

### Added
- ...

### Changed
- ...

### Fixed
- ...
```

```bash
git commit -m "..."
git rev-parse --short HEAD
```

### Step 8: Tag

```bash
git tag -a v<new> -m "v<new>"
git tag --points-at HEAD
```

Rules:
- Always annotated tags
- If tag already exists, confirm with user before overwrite

### Step 9: Push All

```bash
git push origin <current-branch>
git push origin --tags
```

Rules:
- Never force push
- If push fails (remote has new commits), suggest `git pull --rebase` then retry

### Step 10: Verify CI Release

Do NOT create a GitHub release locally. The `.github/workflows/release.yml` GitHub Actions workflow automatically creates the release when tag `v*` is pushed. It builds for all platforms and uploads artifacts.

Wait for 30 seconds then verify:

```bash
gh run list --limit 5
gh release view v<new> --json url,tagName,name,isDraft,isPrerelease,publishedAt 2>&1 || echo "Release not yet created by CI"
```

If the release isn't visible yet, report the workflow run URL so user can monitor progress.

### Step 11: Notify User

Output summary:
```
Release v<new> complete
- Commit: <hash>
- Tag: v<new>
- CI run: <workflow URL>
- Branch: <branch> -> origin/master
- Version files: package.json, package-lock.json, Cargo.toml, tauri.conf.json, CHANGELOG.md
```

## Error Handling

- If `gh` not authenticated: tell user to run `gh auth login`
- If push fails (remote has new commits): suggest `git pull --rebase` then retry
- If tag already exists: ask user to confirm overwrite or skip
- If > 20 files changed: confirm with user before continuing
- If CI release check fails: tell user the push succeeded but CI may still be running
