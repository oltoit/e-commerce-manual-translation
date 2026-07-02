#!/usr/bin/env python3

from __future__ import annotations

import argparse
import csv
import sys
from dataclasses import dataclass, field
from pathlib import Path
from collections import defaultdict
from typing import Optional

import tree_sitter_rust as tsrust
from tree_sitter import Language, Parser
import tomllib


RUST_LANGUAGE = Language(tsrust.language())


@dataclass(frozen=True)
class Module:
    name: str
    path: Path
    crate_root: str


@dataclass
class Graph:
    modules: dict[str, Module] = field(default_factory=dict)
    edges: dict[str, set[str]] = field(default_factory=lambda: defaultdict(set))


def node_text(src: bytes, node) -> str:
    return src[node.start_byte:node.end_byte].decode("utf-8", errors="ignore")


def child_by_type(node, typ: str):
    for c in node.children:
        if c.type == typ:
            return c
    return None


def named_children(node):
    return [c for c in node.children if c.is_named]


def parse_file(parser: Parser, path: Path):
    data = path.read_bytes()
    return data, parser.parse(data)


def normalize_path_parts(parts: list[str]) -> list[str]:
    return [p for p in parts if p not in ("", "self")]


def module_name_from_file(src: Path, file: Path, crate_root: str) -> str:
    rel = file.relative_to(src)

    if rel.name in ("lib.rs", "main.rs"):
        return crate_root

    if rel.name == "mod.rs":
        parts = rel.parent.parts
    else:
        parts = rel.with_suffix("").parts

    return "::".join([crate_root, *parts])


def cargo_package_name(project: Path) -> Optional[str]:
    cargo_toml = project / "Cargo.toml"
    if not cargo_toml.exists():
        return None

    with cargo_toml.open("rb") as f:
        data = tomllib.load(f)

    package = data.get("package", {})
    lib = data.get("lib", {})

    if "name" in lib:
        return lib["name"].replace("-", "_")

    if "name" in package:
        return package["name"].replace("-", "_")

    return None


def load_ignored_modules(ignore_file: Path) -> set[str]:
    if not ignore_file.exists():
        return set()

    ignored = set()

    with ignore_file.open("r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()

            if not line:
                continue

            if line.startswith("#"):
                continue

            ignored.add(line)

    return ignored


def crate_roots(src: Path) -> list[tuple[str, Path]]:
    roots = []

    if (src / "lib.rs").exists():
        roots.append(("crate", src / "lib.rs"))

    if (src / "main.rs").exists():
        roots.append(("bin::main", src / "main.rs"))

    bin_dir = src / "bin"
    if bin_dir.exists():
        for f in sorted(bin_dir.glob("*.rs")):
            roots.append((f"bin::{f.stem}", f))

    if not roots:
        for f in sorted(src.glob("*.rs")):
            roots.append((f"crate::{f.stem}", f))

    return roots


def find_path_attr(src: bytes, node) -> Optional[str]:
    """
    Handles:
      #[path = "foo.rs"]
      #[path="foo/bar.rs"]
    """
    prev = node.prev_named_sibling
    while prev is not None and prev.type in ("attribute_item", "inner_attribute_item"):
        txt = node_text(src, prev)
        if "path" in txt and "=" in txt:
            left = txt.split("=", 1)[1]
            quote = left.find('"')
            if quote >= 0:
                rest = left[quote + 1:]
                end = rest.find('"')
                if end >= 0:
                    return rest[:end]
        prev = prev.prev_named_sibling
    return None


def module_decl_name(src: bytes, node) -> Optional[str]:
    for c in node.children:
        if c.type == "identifier":
            return node_text(src, c)
    return None


def resolve_mod_file(parent_file: Path, mod_name: str, path_attr: Optional[str]) -> Optional[Path]:
    if path_attr:
        p = (parent_file.parent / path_attr).resolve()
        return p if p.exists() else None

    candidates = [
        parent_file.parent / f"{mod_name}.rs",
        parent_file.parent / mod_name / "mod.rs",
    ]

    for c in candidates:
        if c.exists():
            return c.resolve()

    return None


def collect_all_rs_modules(src: Path, roots: list[tuple[str, Path]]) -> dict[Path, str]:
    result = {}

    root_by_path = {p.resolve(): root_name for root_name, p in roots}

    for f in src.rglob("*.rs"):
        rf = f.resolve()

        if rf in root_by_path:
            result[rf] = root_by_path[rf]
            continue

        root_name = "crate"
        result[rf] = module_name_from_file(src, f, root_name)

    return result


def collect_declared_modules(
    parser: Parser,
    src_dir: Path,
    root_name: str,
    root_file: Path,
    path_to_mod: dict[Path, str],
) -> None:
    visited = set()

    def walk_file(file: Path, current_mod: str):
        file = file.resolve()
        if file in visited:
            return
        visited.add(file)

        data, tree = parse_file(parser, file)
        root = tree.root_node

        def visit(node, mod_prefix: str):
            if node.type == "mod_item":
                name = module_decl_name(data, node)
                if not name:
                    return

                body = child_by_type(node, "declaration_list")
                child_mod = f"{mod_prefix}::{name}"

                if body is not None:
                    path_to_mod[file] = mod_prefix
                    for x in named_children(body):
                        visit(x, child_mod)
                else:
                    path_attr = find_path_attr(data, node)
                    mf = resolve_mod_file(file, name, path_attr)
                    if mf:
                        path_to_mod[mf] = child_mod
                        walk_file(mf, child_mod)

            for c in named_children(node):
                if c.type != "mod_item":
                    visit(c, mod_prefix)

        visit(root, current_mod)

    path_to_mod[root_file.resolve()] = root_name
    walk_file(root_file, root_name)


def parse_use_tree(src: bytes, node) -> list[list[str]]:
    """
    Converts Rust use trees into paths.

    Handles:
      use crate::a;
      use crate::{a, b::c};
      use crate::a::{b, c};
      use self::x;
      use super::x;
      pub use crate::x as y;
      use crate::x::*;
    """
    txt = node_text(src, node)

    txt = txt.strip()
    if txt.startswith("pub "):
        txt = txt[4:].strip()
    if txt.startswith("use "):
        txt = txt[4:].strip()
    if txt.endswith(";"):
        txt = txt[:-1].strip()

    out = []

    def split_top_level(s: str) -> list[str]:
        parts = []
        depth = 0
        cur = []
        for ch in s:
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
            elif ch == "," and depth == 0:
                part = "".join(cur).strip()
                if part:
                    parts.append(part)
                cur = []
                continue
            cur.append(ch)
        part = "".join(cur).strip()
        if part:
            parts.append(part)
        return parts

    def expand(prefix: list[str], s: str):
        s = s.strip()

        if " as " in s:
            s = s.split(" as ", 1)[0].strip()

        if s.endswith("::*"):
            s = s[:-3]

        if "{" not in s:
            parts = [p.strip() for p in s.split("::") if p.strip()]
            out.append(prefix + parts)
            return

        before, rest = s.split("{", 1)
        before = before.rstrip(":").strip()
        inside = rest.rsplit("}", 1)[0]

        new_prefix = prefix[:]
        if before:
            new_prefix += [p.strip() for p in before.split("::") if p.strip()]

        for part in split_top_level(inside):
            expand(new_prefix, part)

    for part in split_top_level(txt):
        expand([], part)

    return out


def resolve_path(
    current_mod: str,
    parts: list[str],
    known: set[str],
    package_name: Optional[str],
) -> Optional[str]:
    parts = normalize_path_parts(parts)
    if not parts:
        return None

    if package_name and parts[0] == package_name:
        parts[0] = "crate"

    if parts[0] == "crate":
        full = parts

    elif parts[0] == "super":
        base = current_mod.split("::")
        i = 0
        while i < len(parts) and parts[i] == "super":
            if len(base) > 1:
                base.pop()
            i += 1
        full = base + parts[i:]

    elif parts[0] == "self":
        full = current_mod.split("::") + parts[1:]

    else:
        full = current_mod.split("::") + parts

    for i in range(len(full), 0, -1):
        cand = "::".join(full[:i])
        if cand in known:
            return cand

    return None


def build_graph(project: Path) -> Graph:
    src = project / "src"
    package_name = cargo_package_name(project)
    print(f"Detected crate name: {package_name}")

    if not src.exists():
        raise RuntimeError(f"No src directory: {src}")

    parser = Parser(RUST_LANGUAGE)
    roots = crate_roots(src)

    path_to_mod = collect_all_rs_modules(src, roots)

    for root_name, root_file in roots:
        collect_declared_modules(parser, src, root_name, root_file, path_to_mod)

    graph = Graph()

    for path, mod_name in path_to_mod.items():
        graph.modules[mod_name] = Module(
            name=mod_name,
            path=path,
            crate_root=mod_name.split("::")[0],
        )

    known = set(graph.modules)

    for path, current_mod in path_to_mod.items():
        if not path.exists():
            continue

        data, tree = parse_file(parser, path)

        def visit(node):
            if node.type == "mod_item":
                name = module_decl_name(data, node)
                if name:
                    path_attr = find_path_attr(data, node)
                    mf = resolve_mod_file(path, name, path_attr)
                    if mf and mf in path_to_mod:
                        target = path_to_mod[mf]
                        if target != current_mod:
                            graph.edges[current_mod].add(target)

            if node.type == "use_declaration":
                for parts in parse_use_tree(data, node):
                    target = resolve_path(current_mod, parts, known, package_name)
                    if target and target != current_mod:
                        graph.edges[current_mod].add(target)

            for c in named_children(node):
                visit(c)

        visit(tree.root_node)

    return graph


def write_csv(graph: Graph, output: Path, ignored: set[str]) -> None:
    fan_in = {m: 0 for m in graph.modules}
    fan_out = {m: len(graph.edges.get(m, set())) for m in graph.modules}

    for module in ignored:
        fan_in.pop(module, None)
        fan_out.pop(module, None)

    for source, targets in graph.edges.items():
        if source in ignored:
            continue

        for target in targets:
            if target in ignored:
                continue

            fan_in[target] += 1

    with output.open("w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)

        writer.writerow([
            "Module",
            "Fan-In",
            "Fan-Out",
            "Instability",
        ])

        for module in sorted(m for m in graph.modules if m not in ignored):
            fi = fan_in[module]
            fo = fan_out[module]

            instability = (
                fo / (fi + fo)
                if (fi + fo) > 0
                else 0.0
            )

            writer.writerow([
                module,
                fi,
                fo,
                round(instability, 6),
            ])


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("project", help="Rust project directory")
    ap.add_argument("-o", "--output", default="module_coupling.csv")
    args = ap.parse_args()

    project = Path(args.project).resolve()
    output = Path(args.output).resolve()

    ignored = load_ignored_modules(Path("ignore_files.txt"))
    graph = build_graph(project)
    write_csv(graph, output, ignored)

    print(f"Wrote {output}")
    print(f"Modules: {len(graph.modules)}")
    print(f"Edges: {sum(len(v) for v in graph.edges.values())}")

    return 0


if __name__ == "__main__":
    sys.exit(main())