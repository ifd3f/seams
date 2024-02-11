#!/usr/bin/env python3

from datetime import datetime
from pathlib import Path
import shutil
import sys
from typing import Any, Dict, Tuple
import yaml
import click


@click.command()
@click.option("-o", "--out", help="Output directory root")
@click.option(
    "-f", "--force", is_flag=True, help="Clobber output directory if it exists"
)
@click.argument("input_dir")
def main(input_dir: str, force: bool, out: str):
    if Path(out).exists():
        if force:
            print(f"clobbering {out}")
            shutil.rmtree(out)
        else:
            print(f"{out} exists! Provide -f/--force to clobber")
            sys.exit(-1)

    Path(out).mkdir(parents=True, exist_ok=False)

    for p in (Path(input_dir) / "blog").glob("**/*.md"):
        transform_post(p, Path(out))

    for p in (Path(input_dir) / "blog").glob("**/*.recipe.md"):
        out_recipedir = Path(out) / "recipes"
        out_recipedir.mkdir(parents=True, exist_ok=True)
        shutil.copy(p, out_recipedir)

    for p in (Path(input_dir) / "tags").glob("**/*.yaml"):
        transform_tagdecl(p, Path(out))

    for p in (Path(input_dir) / "projects").glob("**/*.md"):
        transform_project(p, Path(out))


def transform_post(post_file: Path, outdir: Path):
    with post_file.open("r") as f:
        data = f.read()
    old, markdown = split_graymatter(data)
    if post_file.name.endswith(".recipe.md"):
        return

    slug = (
        post_file.parent.name if post_file.name == "index.md" else post_file.name
    ).removesuffix(".md")

    date = datetime.fromisoformat(str(old["date"]).replace("Z", " "))
    new = {
        "title": old.get(
            "title", f"Status on {date.year}-{date.month:02}-{date.day:02}"
        ),
        "tagline": old.get("description"),
        "slug": slug,
        "date": {
            "created": str(date),
            "published": str(date),
        },
        "url": {},
    }
    if t := old.get("tags"):
        new["tags"] = t
    if u := old.get("thumbnail"):
        new["thumbnail"] = u

    new_ydata = yaml.safe_dump(new, sort_keys=False)

    post_subdir = "{}/{:02}/{:02}/{}/{}".format(
        date.year, date.month, date.day, old.get("ordinal", 0), slug
    )
    outfile = outdir / "blog" / post_subdir / "index.md"
    outfile.parent.mkdir(parents=True, exist_ok=True)
    with outfile.open("w") as f:
        f.write("---\n")
        f.write(new_ydata)
        f.write("\n---\n\n")
        f.write(markdown)


def transform_project(project_file: Path, outdir: Path):
    with project_file.open("r") as f:
        data = f.read()
    old, markdown = split_graymatter(data)
    slug = project_file.parent.name

    new = {
        "title": old["title"],
        "tagline": old["description"],
        "slug": slug,
        "status": old["status"],
        "date": {
            "started": old["startDate"],
            "finished": old.get("endDate"),
            "published": datetime.now(),
        },
        "tags": old["tags"],
        "url": {},
    }
    if u := old.get("url"):
        new["url"]["site"] = u
    if s := old.get("source"):
        new["url"]["source"] = s
    if u := old.get("thumbnail"):
        new["thumbnail"] = u

    new_ydata = yaml.dump(new, sort_keys=False)

    sd: datetime = old["startDate"]
    project_subdir = "{}-{:02}-{}".format(sd.year, sd.month, slug)
    outfile = outdir / "projects" / project_subdir / "index.md"
    outfile.parent.mkdir(parents=True, exist_ok=True)
    with outfile.open("w") as f:
        f.write("---\n")
        f.write(new_ydata)
        f.write("\n---\n\n")
        f.write(markdown)


def transform_tagdecl(tag_file: Path, outdir: Path):
    with tag_file.open("r") as f:
        old = yaml.safe_load(f.read())

    slug_to_title = {}
    styles = []

    for decl in old:
        styling = {}
        if text := decl.get("color"):
            styling["textcolor"] = text
        if bg := decl.get("backgroundColor"):
            styling["bgcolor"] = bg
        styles.append(
            {
                "tags": [t["slug"] for t in decl["tags"]],
                "style": styling,
            }
        )

        for t in decl["tags"]:
            slug_to_title[t["slug"]] = t["name"]

    new = {"titles": slug_to_title, "styles": styles}

    outfile = outdir / "tags" / tag_file.name
    outfile.parent.mkdir(parents=True, exist_ok=True)
    with outfile.open("w") as f:
        f.write(yaml.safe_dump(new, sort_keys=False))


def split_graymatter(file_contents: str) -> Tuple[Dict[str, Any], str]:
    _, _, yaml_and_rest = file_contents.partition("---")
    yaml_data, _, markdown = yaml_and_rest.partition("---")
    return yaml.safe_load(yaml_data), markdown.strip()


if __name__ == "__main__":
    main()