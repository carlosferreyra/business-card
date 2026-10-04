"""Verify wrapper publication corresponds to an existing binary release."""

import json
import subprocess
import tomllib
from pathlib import Path


def main() -> None:
    package = tomllib.loads(Path("Cargo.toml").read_text())["package"]
    tag = f"v{package['version']}"
    release = json.loads(subprocess.check_output([
        "gh", "api", f"repos/{{owner}}/{{repo}}/releases/tags/{tag}"
    ], text=True))
    if release["draft"]:
        raise RuntimeError(f"Release {tag} is still a draft")
    assets = {asset["name"] for asset in release["assets"]}
    if f"{package['name']}-installer.sh" not in assets:
        raise RuntimeError(f"Release {tag} has no shell installer")
    subprocess.run(["git", "fetch", "origin", f"refs/tags/{tag}"], check=True)
    released = subprocess.check_output(["git", "rev-parse", "FETCH_HEAD^{commit}"], text=True).strip()
    current = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
    if released != current:
        raise RuntimeError(f"Release {tag} does not match checked-out commit {current}")
    print(f"Verified published release {tag} at {current}")


if __name__ == "__main__":
    main()
