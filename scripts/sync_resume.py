# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "httpx",
# ]
# ///

import json

import httpx
from pathlib import Path

RESUME_URL = "https://raw.githubusercontent.com/carlosferreyra/carlosferreyra/main/resume.json"
DEST = Path(__file__).parent.parent / "resume.json"


def validate_resume(text: str) -> None:
    catalog = json.loads(text)
    profile = catalog["profiles"]["business-card"]

    def require_strings(record: dict, fields: tuple[str, ...]) -> None:
        for field in fields:
            if not isinstance(record[field], str):
                raise ValueError(f"{field} must be a string")

    require_strings(profile, ("title", "summary"))
    require_strings(catalog["personalInfo"], ("name", "email", "location"))
    overrides = profile.get("personalInfo")
    if overrides is not None:
        for field in ("name", "email", "location"):
            if overrides.get(field) is not None:
                require_strings(overrides, (field,))
    for section, fields in (
        ("links", ("id", "label", "url")),
        ("skills", ()),
        ("projects", ("name", "description", "url")),
    ):
        if not isinstance(catalog[section], list):
            raise ValueError(f"{section} must be an array")
        for record in catalog[section]:
            require_strings(record, fields)
            for field in (("labels", "items") if section == "skills" else ("labels",)):
                if not isinstance(record[field], list) or not all(
                    isinstance(item, str) for item in record[field]
                ):
                    raise ValueError(f"{section}.{field} must be an array of strings")


def main() -> None:
    print(f"Fetching {RESUME_URL} ...")
    response = httpx.get(RESUME_URL, follow_redirects=True)
    response.raise_for_status()
    validate_resume(response.text)
    DEST.write_text(response.text, encoding="utf-8")
    print(f"Saved to {DEST}")


if __name__ == "__main__":
    main()
