#!/usr/bin/env python3
import pathlib
import re
import dataclasses
import subprocess
import argparse
import enum

from typing import Iterator

MSG_RE = re.compile(r"^(feat|chore|fix)(!)?(?:\+(feat|chore|fix)(!)?)*:")


@dataclasses.dataclass(order=True)
class Version:
    major: int
    minor: int
    patch: int

    def increase_major(self):
        if self.major == 0:
            return Version(0, self.minor + 1, 0)
        return Version(self.major + 1, 0, 0)

    def increase_minor(self):
        if self.major == 0:
            return Version(0, self.minor, self.patch + 1)
        return Version(self.major, self.minor + 1, 0)

    def increase_patch(self):
        return Version(self.major, self.minor, self.patch + 1)

    def __str__(self):
        return f"{self.major}.{self.minor}.{self.patch}"


@dataclasses.dataclass
class CargoManifest:
    version: Version
    name: str
    path: pathlib.Path

    def write_version_to_file(self, version: Version):
        lines = []
        version_updated = False
        for line in self.path.read_text().splitlines():
            if not version_updated and line.startswith("version"):
                line = f'version = "{version}"'
                version_updated = True
            lines.append(line)

        self.path.write_text("\n".join(lines) + "\n")


class ChangeType(enum.Enum):
    MAJOR = 3
    MINOR = 2
    PATCH = 1

    def __gt__(self, other):
        return self.value > other.value

    def __lt__(self, other):
        return self.value < other.value


@dataclasses.dataclass
class GitCommit:
    commit: str
    msg: str

    def classify(self) -> ChangeType | None:
        m = MSG_RE.match(self.msg)
        if not m:
            return None
        tags = m.group(0).removesuffix(":").split("+")
        if any(tag.endswith("!") for tag in tags):
            return ChangeType.MAJOR
        if any(tag.startswith("feat") for tag in tags):
            return ChangeType.MINOR
        return ChangeType.PATCH

    def __str__(self):
        return f"{self.commit} {self.msg}"


def find_cargo_files() -> Iterator[pathlib.Path]:
    return pathlib.Path(__file__).parent.parent.glob("**/Cargo.toml")


def parse_cargo_file(path: pathlib.Path) -> CargoManifest | None:
    version = None
    name = None
    for line in path.read_text().splitlines():
        if not version and line.startswith("version"):
            version = parse_version(line)
        elif not name and line.startswith("name"):
            name = parse_name(line)
    if version is None or name is None:
        print("missing version or name in", path)
        return None
    return CargoManifest(version, name, path)


def parse_version(line: str) -> Version | None:
    match = re.match(r"version = \"(\d+)\.(\d+)\.(\d+)\"", line)
    if not match:
        return None
    return Version(*map(int, match.groups()))


def parse_name(line: str) -> str | None:
    match = re.match(r"name = \"([\w\-]+)\"", line)
    if not match:
        return None
    return match.group(1)


def estimate_changes():
    last_release_tag = subprocess.check_output(
        ["git", "describe", "--tags", "--abbrev=0"],
        text=True,
    ).strip()
    commits = [
        GitCommit(commit=commit.split(" ")[0], msg=commit.split(" ", 1)[1])
        for commit in subprocess.check_output(
            ["git", "log", "--oneline", f"{last_release_tag}..HEAD"],
            text=True,
        ).splitlines()
    ]

    classfication = ChangeType.PATCH
    for commit in commits:
        if m := commit.classify():
            if m > classfication:
                classfication = m
        else:
            print("Commit", commit, "not recognized")
    return classfication


def main():
    parser = argparse.ArgumentParser()
    grp = parser.add_mutually_exclusive_group()
    grp.add_argument("--major", action="store_true")
    grp.add_argument("--minor", action="store_true")
    grp.add_argument("--patch", action="store_true")
    parser.add_argument("--auto", action="store_true")
    parser.add_argument("--set")
    args = parser.parse_args()

    manifests = [
        manifest for path in find_cargo_files() if (manifest := parse_cargo_file(path))
    ]
    highest_version = max(manifests, key=lambda manifest: manifest.version)
    print("highest version:", highest_version.version, f"({highest_version.name})")
    highest_version = highest_version.version

    if args.auto:
        classification = estimate_changes()
        print("Estimated changes:", classification)
    elif args.major:
        classification = ChangeType.MAJOR
    elif args.minor:
        classification = ChangeType.MINOR
    elif args.patch:
        classification = ChangeType.PATCH

    match classification:
        case ChangeType.MAJOR:
            new_version = highest_version.increase_major()
        case ChangeType.MINOR:
            new_version = highest_version.increase_minor()
        case ChangeType.PATCH:
            new_version = highest_version.increase_patch()

    print("New version:", new_version)

    for manifest in manifests:
        print("Updating", manifest.path)
        manifest.write_version_to_file(new_version)

    print("Committing changes")
    subprocess.run(["git", "add", *map(str, find_cargo_files())], check=True)
    subprocess.run(["git", "commit", "-m", f"release {new_version}"], check=True)
    subprocess.run(
        ["git", "tag", "-a", f"v{new_version}", "-m", f"v{new_version}"], check=True
    )


if __name__ == "__main__":
    main()
