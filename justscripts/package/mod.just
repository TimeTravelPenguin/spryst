_default:
  @just --list package

[group("Packaging")]
[doc("Prepares the package for release.")]
[no-cd]
new: init update

[private]
[no-cd]
init:
  #!/usr/bin/env sh
  set -euxo pipefail

  # see: https://github.com/typst/packages/blob/main/docs/tips.md#sparse-checkout-of-the-repository
  git clone --depth 1 --no-checkout --filter="tree:0" git@github.com:typst/packages
  cd packages
  git sparse-checkout init
  git sparse-checkout set packages/preview/spryst
  git checkout main

  # If maintainers change, update the URL below
  git remote add fork "git@github.com:TimeTravelPenguin/packages.git"
  git fetch fork
  git rebase fork/main --onto origin/main

[group("Packaging")]
[doc("Updates the package with the latest changes.")]
[no-cd]
update:
  #!/usr/bin/env sh
  set -euo pipefail

  # Get the version from the typst.toml file
  version=$(dasel -i toml 'package.version' < typst/typst.toml | tr -d "'")
  echo "Preparing package for version: $version"
  
  # Check that ./dist/spryst exists. If not, error.
  if [ ! -d "./dist/spryst/$version" ]; then
    echo "The directory ./dist/spryst/$version does not exist. Please run 'just bundle' to create the package before updating."
    exit 1
  fi

  # Check if origin has a tag matching the version. If not, error.
  if ! git ls-remote --tags origin | grep -q "refs/tags/v$version"; then
    echo "The tag v$version does not exist in the origin repository. Please create using:"
    echo "  git tag -a \"v$version\" -m \"v$version\" && git push origin \"v$version\""
    exit 1
  fi

  # Checkout fork branch if it exists, otherwise create a new branch
  cd packages
  fork_branch=$(git ls-remote --heads fork "refs/heads/spryst-v$version")

  if git show-ref --verify --quiet "refs/heads/spryst-v$version"; then
    echo "Local branch spryst-v$version exists. Checking out."
    git checkout "spryst-v$version"
    git pull --rebase origin main
  elif [ -n "$fork_branch" ]; then
    echo "Branch fork/spryst-v$version exists. Checking out."
    git checkout -b "spryst-v$version" "fork/spryst-v$version"
    git pull --rebase origin main
  else
    echo "Branch fork/spryst-v$version does not exist. Creating new branch and checking out."
    git checkout -b "spryst-v$version" origin/main
  fi

  cd ..
  # The typst/packages repo nests everything under a top-level `packages/`
  # dir, so the package lives at `<clone>/packages/preview/spryst/<version>`.
  dest="packages/packages/preview/spryst/$version"
  rm -rf "$dest"
  mkdir -p "$dest"
  cp -r "dist/spryst/$version/." "$dest"

  # Push changes to fork
  cd packages
  git add "packages/preview/spryst/$version"
  git commit -m "chore: update spryst package to v$version"
  git push -u fork "spryst-v$version"

[no-cd]
[confirm("Remove directory `packages`? (y/N)")]
remove:
  rm -rf packages
