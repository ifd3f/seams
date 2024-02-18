#!/usr/bin/env python3

from datetime import datetime, tzinfo
from pathlib import Path
import shutil
import sys
from typing import Any, Dict, Tuple
from zoneinfo import ZoneInfo
import yaml
import click


@click.command()
@click.option("-o", "--out", required=True, help="Output directory root")
@click.option(
    "-f", "--force", is_flag=True, help="Clobber output directory if it exists"
)
@click.option(
    "--published", required=True, help="Default published date"
)
@click.argument("input_dir")
def main(input_dir: str, force: bool, out: str, published: str):
    published: datetime = datetime.fromisoformat(published)
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
        transform_project(p, Path(out), published)


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
    if date.tzname() is None:
        date = date.astimezone(ZoneInfo("America/Los_Angeles"))

    if t := old.get("title"):
        title = t
        out_root = 'blog'
    else:
        title = None
        out_root = '_untitled_posts'

    new = {
        "title": title,
    }
    if tagline := old.get("description"):
        new["tagline"] = tagline
    if ts := old.get("tags"):
        new["tags"] = [transform_tag_slug(t) for t in ts]
    if u := old.get("thumbnail"):
        new["thumbnail"] = u
    new = {
        **new,
        "slug": slug,
        "date": {
            "created": str(date),
            "published": str(date),
        },
    }

    new_ydata = yamldump(new)

    post_subdir = "{}/{:02}/{:02}/{}/{}".format(
        date.year, date.month, date.day, old.get("ordinal", 0), slug
    )
    outfile = outdir / out_root / post_subdir / "index.md"
    outfile.parent.mkdir(parents=True, exist_ok=True)
    with outfile.open("w") as f:
        f.write("---\n")
        f.write(new_ydata)
        f.write("\n---\n\n")
        f.write(markdown)


def transform_project(project_file: Path, outdir: Path, published_date: datetime):
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
            "published": published_date,
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

    new_ydata = yamldump(new)

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
        if bg := decl.get("backgroundColor"):
            styling["color"] = bg
        if text := decl.get("color"):
            styling["text_color"] = text
        styles.append(
            {
                "tags": [t["slug"] for t in decl["tags"]],
                "apply": styling,
            }
        )

        for t in decl["tags"]:
            slug_to_title[t["slug"]] = t["name"]

    new = {"titles": slug_to_title, "styles": styles}

    outfile = outdir / "settings" / "tags.tag.yml"
    outfile.parent.mkdir(parents=True, exist_ok=True)
    with outfile.open("w") as f:
        f.write(yamldump(new))


def yamldump(x: Any) -> str:
    return yaml.safe_dump(x, sort_keys=False, allow_unicode=True)


def split_graymatter(file_contents: str) -> Tuple[Dict[str, Any], str]:
    _, _, yaml_and_rest = file_contents.partition("---")
    yaml_data, _, markdown = yaml_and_rest.partition("---")
    return yaml.safe_load(yaml_data), markdown.strip()


def transform_tag_slug(tag: str):
    if tag.startswith("/projects/"):
        return 'project:' + tag.removeprefix("/projects/").replace('/', '')
    return tag


if __name__ == "__main__":
    main()
