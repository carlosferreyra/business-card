# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "httpx",
# ]
# ///

import httpx
from pathlib import Path

RESUME_URL = "https://raw.githubusercontent.com/carlosferreyra/carlosferreyra/main/data/resume.json"
DEST = Path(__file__).parent.parent / "resume.json"


def main() -> None:
    print(f"Fetching {RESUME_URL} ...")
    response = httpx.get(RESUME_URL, follow_redirects=True)
    response.raise_for_status()
    DEST.write_text(response.text, encoding="utf-8")
    print(f"Saved to {DEST}")


if __name__ == "__main__":
    main()
