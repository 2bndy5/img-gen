#!/usr/bin/env python3
"""
Sync/verify gradient presets from WebGradients upstream source.

This script reads the preset list from
`crates/img-gen-spec/src/validators/layers/colors/gradients/gradient_presets.rs`,
downloads upstream data from WebGradients, and can:

1) report diffs (`--check`)
2) rewrite the preset macro payload (`--write`)

It preserves local variant naming and order, while sourcing comments/stops from OG.

Index Contract:
`Presets::from_index(i: u8)` maps to the generated `enumerate_specs![]` order,
so this script enforces a hard cap of 256 rendered presets.
"""

from __future__ import annotations

import argparse
import dataclasses
import json
import re
import sys
import urllib.request
from collections import defaultdict
from pathlib import Path
from typing import Iterable


DEFAULT_JSON_URL = (
    "https://raw.githubusercontent.com/itmeo/webgradients/master/gradients.json"
)
DEFAULT_CSS_URL = (
    "https://raw.githubusercontent.com/itmeo/webgradients/master/webgradients.css"
)
DEFAULT_RS_FILE = Path(
    "crates/img-gen-spec/src/validators/layers/colors/gradients/gradient_presets.rs"
)
DEFAULT_PYI_FILE = Path("img_gen.pyi")

COMMENT_LINE_RE = re.compile(r"^\s*//\s?")
VARIANT_LINE_RE = re.compile(r"^\s*([A-Za-z0-9_]+)(?:\s*=\s*([0-9]+))?,\s*$")
RUST_STOP_RE = re.compile(r"\(\s*\"([^\"]+)\"\s*,\s*([0-9]*\.?[0-9]+)f32\s*\)")
CSS_BLOCK_RE = re.compile(r"/\*\s*\d+\s+(.+?)\*/\s*\.[^{]+\{(.*?)\}", re.S)
CSS_DECL_RE = re.compile(r"(background(?:-image)?\s*:\s*.*?;)", re.S)
CSS_STOP_RE = re.compile(r"(#[0-9A-Fa-f]{3,8}|[A-Za-z]+)\s+([0-9]+)%")
ENUMERATE_MACRO_RE = re.compile(r"enumerate_specs!\[\n(.*)\n\];", re.S)
PYI_PRESETS_BLOCK_RE = re.compile(
    r"class Presets\(IntEnum\):\n(.*?)\n\nclass ColorGradient:", re.S
)
SKIP_CSS_COLOR_TOKENS = {"rgba", "rgb", "hsla", "hsl", "at", "to"}
REQUIRED_LOCAL_PRESET_SPECS = {
    "MonoChrome": (("black", 0.0), ("white", 1.0)),
}
MAX_PRESET_INDEX_CARDINALITY = 256


@dataclasses.dataclass(frozen=True)
class Stop:
    color: str
    pos: float


@dataclasses.dataclass(frozen=True)
class ExistingEntry:
    variant: str
    index: int | None
    comments: tuple[str, ...]
    stops: tuple[Stop, ...]


@dataclasses.dataclass(frozen=True)
class OgGradient:
    name: str
    css_line: str
    stops: tuple[Stop, ...]


def _fetch_text(url: str) -> str:
    with urllib.request.urlopen(url) as response:
        return response.read().decode("utf-8")


def _pascal_words(name: str) -> str:
    cleaned = re.sub(r"[^A-Za-z0-9]+", " ", name)
    return "".join(part.capitalize() for part in cleaned.split())


def _normalize_color(color: str) -> str:
    return color.strip()


def _parse_stop(color: str, pos: str, percent: bool = False) -> Stop:
    value = float(pos)
    if percent:
        value /= 100.0
    return Stop(color=_normalize_color(color), pos=value)


def _stops_equal(a: Stop, b: Stop) -> bool:
    return a.color.lower() == b.color.lower() and abs(a.pos - b.pos) < 1e-9


def _collapse_identical_stops(stops: Iterable[Stop]) -> tuple[Stop, ...]:
    out: list[Stop] = []
    for stop in stops:
        if out and _stops_equal(out[-1], stop):
            continue
        out.append(stop)
    return tuple(out)


def _separate_same_position_colors(stops: Iterable[Stop]) -> tuple[Stop, ...]:
    """Spread same-position, different-color stops apart by ~0.01 where possible.

    This keeps gradients deterministic for consumers that expect monotonically
    increasing stop positions.
    """
    items = list(stops)
    if not items:
        return tuple()

    result = [Stop(color=s.color, pos=s.pos) for s in items]
    i = 0
    eps = 1e-6
    while i < len(result):
        j = i + 1
        while j < len(result) and abs(result[j].pos - result[i].pos) < 1e-9:
            j += 1

        if j - i <= 1:
            i = j
            continue

        # Keep groups with identical colors untouched after collapse.
        colors = {s.color.lower() for s in result[i:j]}
        if len(colors) <= 1:
            i = j
            continue

        base = result[i].pos
        next_pos = result[j].pos if j < len(result) else 1.0
        prev_pos = result[i - 1].pos if i > 0 else 0.0
        count = j - i

        # Try forward spacing first.
        if next_pos - base > eps:
            max_step = (next_pos - base - eps) / max(1, count - 1)
            step = min(0.01, max_step)
            if step > 0:
                for k in range(count):
                    pos = min(1.0, base + (k * step))
                    result[i + k] = Stop(color=result[i + k].color, pos=pos)
                i = j
                continue

        # If forward spacing is unavailable, spread backward.
        if base - prev_pos > eps:
            max_step = (base - prev_pos - eps) / max(1, count - 1)
            step = min(0.01, max_step)
            if step > 0:
                start = max(0.0, base - ((count - 1) * step))
                for k in range(count):
                    pos = min(1.0, start + (k * step))
                    result[i + k] = Stop(color=result[i + k].color, pos=pos)

        i = j

    return tuple(result)


def _normalize_stops(stops: Iterable[Stop]) -> tuple[Stop, ...]:
    collapsed = _collapse_identical_stops(stops)
    return _separate_same_position_colors(collapsed)


def _parse_existing_entries(rust_src: str) -> tuple[ExistingEntry, ...]:
    lines = rust_src.splitlines()
    entries: list[ExistingEntry] = []
    i = 0
    while i < len(lines):
        if not lines[i].strip():
            i += 1
            continue

        comments: list[str] = []
        while i < len(lines) and lines[i].strip().startswith("//"):
            comments.append(COMMENT_LINE_RE.sub("", lines[i]))
            i += 1

        while i < len(lines) and not lines[i].strip():
            i += 1

        m_variant = VARIANT_LINE_RE.match(lines[i] if i < len(lines) else "")
        if not m_variant:
            i += 1
            continue
        variant = m_variant.group(1)
        index = int(m_variant.group(2)) if m_variant.group(2) is not None else None
        i += 1

        block: list[str] = []
        while i < len(lines):
            block.append(lines[i])
            if "]," in lines[i]:
                break
            i += 1

        stops: list[Stop] = []
        for line in block:
            for color, pos in RUST_STOP_RE.findall(line):
                stops.append(_parse_stop(color, pos))

        entries.append(
            ExistingEntry(
                variant=variant,
                index=index,
                comments=tuple(comments),
                stops=tuple(stops),
            )
        )
        i += 1

    return tuple(entries)


def _ensure_required_local_presets(
    existing: tuple[ExistingEntry, ...],
) -> tuple[ExistingEntry, ...]:
    variants = {entry.variant for entry in existing}
    missing: list[ExistingEntry] = []
    for variant, stops in REQUIRED_LOCAL_PRESET_SPECS.items():
        if variant in variants:
            continue
        missing.append(
            ExistingEntry(
                variant=variant,
                index=None,
                comments=(),
                stops=tuple(Stop(color=color, pos=pos) for color, pos in stops),
            )
        )
    if not missing:
        return existing
    return tuple(missing) + existing


def _parse_css_gradient_entries(css_src: str) -> list[tuple[str, str]]:
    # Capture blocks like: /*001 Warm Flame*/ ... { ... }
    # Keep the first background/background-image declaration flattened into one line.
    out: list[tuple[str, str]] = []
    for m in CSS_BLOCK_RE.finditer(css_src):
        name = m.group(1).strip()
        body = m.group(2)
        decl_match = CSS_DECL_RE.search(body)
        if not decl_match:
            continue
        line = re.sub(r"\s+", " ", decl_match.group(1)).strip()
        out.append((name, line))
    return out


def _parse_stops_from_css_line(css_line: str) -> tuple[Stop, ...]:
    # Parse only color+percent pairs from a CSS gradient declaration.
    # Ignore function/position tokens to avoid mis-parsing layered blended gradients.
    out: list[Stop] = []
    for color, pos in CSS_STOP_RE.findall(css_line):
        if color.lower() in SKIP_CSS_COLOR_TOKENS:
            continue
        out.append(_parse_stop(color, pos, percent=True))
    return tuple(out)


def _parse_og_gradients(json_src: str, css_src: str) -> dict[str, list[OgGradient]]:
    css_entries = _parse_css_gradient_entries(css_src)
    payload = json.loads(json_src)
    by_name: dict[str, list[OgGradient]] = defaultdict(list)

    # Primary source: CSS contains the full collection and preserves order.
    for name, css_line in css_entries:
        css_stops = _parse_stops_from_css_line(css_line)
        stops = _normalize_stops(css_stops)
        by_name[name].append(
            OgGradient(
                name=name,
                css_line=css_line,
                stops=stops,
            )
        )

    # Secondary source: JSON can be newer/cleaner for some entries.
    # Overlay matching entries by name/index where possible.
    json_seen: dict[str, int] = defaultdict(int)
    for item in payload:
        name = str(item["name"])
        json_seen[name] += 1
        index = json_seen[name] - 1
        json_stops = tuple(
            _parse_stop(str(stop["color"]), str(stop["pos"]), percent=True)
            for stop in item["gradient"]
        )
        normalized_json = _normalize_stops(json_stops)
        if name in by_name and index < len(by_name[name]):
            existing = by_name[name][index]
            if not existing.stops:
                by_name[name][index] = OgGradient(
                    name=name,
                    css_line=existing.css_line,
                    stops=normalized_json,
                )
        else:
            by_name[name].append(
                OgGradient(
                    name=name,
                    css_line="",
                    stops=normalized_json,
                )
            )

    return by_name


def _variant_mapping(existing_variant: str) -> tuple[str, int] | None:
    # Explicit aliases for local names that differ from OG names.
    aliases: dict[str, tuple[str, int] | None] = {
        "MonoChrome": None,
        "WarmLight": ("Warm Flame", 1),
        "DeepBlue": ("Deep Blue", 1),
        "DeepBlue2": ("Deep Blue", 2),
        "AriellesSmile": ("Arielle's Smile", 1),
    }
    if existing_variant in aliases:
        return aliases[existing_variant]

    # Fallback: variant name is PascalCase of OG name.
    # We'll match first exact PascalCase collision.
    return (_variant_to_og_name_guess(existing_variant), 1)


def _variant_to_og_name_guess(variant: str) -> str:
    spaced = re.sub(r"([a-z0-9])([A-Z])", r"\1 \2", variant)
    return spaced


def _get_og_for_variant(
    variant: str,
    og_by_name: dict[str, list[OgGradient]],
) -> OgGradient | None:
    mapping = _variant_mapping(variant)
    if mapping is None:
        return None
    og_name, index = mapping

    if og_name in og_by_name and len(og_by_name[og_name]) >= index:
        return og_by_name[og_name][index - 1]

    # Secondary fallback: normalize by PascalCase.
    variant_key = variant.lower()
    for name, grads in og_by_name.items():
        if _pascal_words(name).lower() == variant_key and len(grads) >= index:
            return grads[index - 1]

    return None


def _should_drop_entry(og: OgGradient | None) -> bool:
    # Layered upstream presets do not round-trip through enumerate_specs!, so we
    # omit them entirely instead of trying to render an empty stop list.
    return og is not None and not og.stops


def _fmt_pos(pos: float) -> str:
    text = f"{pos:.6f}".rstrip("0").rstrip(".")
    if "." not in text:
        text += ".0"
    return f"{text}f32"


def _format_stop(stop: Stop, indent: str = "") -> str:
    return f'{indent}("{stop.color}", {_fmt_pos(stop.pos)})'


def _format_entry(entry: ExistingEntry, index: int, og: OgGradient | None) -> str:
    comments = list(entry.comments)
    stops = entry.stops

    if og is not None:
        if og.css_line:
            comments = [og.css_line]
        if og.stops:
            stops = og.stops

    if not stops:
        raise RuntimeError(
            f"No stops available for variant {entry.variant}; cannot render an empty stop list"
        )

    stop_lines = "\n".join(f"{_format_stop(stop, '        ')}," for stop in stops)

    if len(stops) <= 2:
        one_line = ", ".join(_format_stop(stop) for stop in stops)
        stop_block = f"[{one_line}],"
    else:
        stop_block = f"[\n{stop_lines}\n    ],"

    comment_block = "\n".join(f"    // {line}" for line in comments)
    if comment_block:
        return f"{comment_block}\n    {entry.variant} = {index},\n    {stop_block}"
    return f"    {entry.variant} = {index},\n    {stop_block}"


def _render_stub_presets(
    existing: tuple[ExistingEntry, ...],
    og_by_name: dict[str, list[OgGradient]],
) -> str:
    rendered: list[str] = []
    rendered_count = 0
    for entry in existing:
        og = _get_og_for_variant(entry.variant, og_by_name)
        if _should_drop_entry(og):
            continue
        rendered.append(f"    {entry.variant} = {rendered_count}")
        rendered_count += 1

    if rendered_count > MAX_PRESET_INDEX_CARDINALITY:
        raise RuntimeError(
            "Rendered preset count exceeds u8 index capacity "
            f"({rendered_count} > {MAX_PRESET_INDEX_CARDINALITY})"
        )

    rendered.extend(
        [
            "",
            "    @staticmethod",
            "    def from_str(val: str) -> Presets: ...",
            "    @staticmethod",
            "    def from_index(i: int) -> Presets: ...",
        ]
    )
    return "\n".join(rendered)


def _render_entries(
    existing: tuple[ExistingEntry, ...],
    og_by_name: dict[str, list[OgGradient]],
) -> str:
    rendered: list[str] = []
    rendered_count = 0
    for entry in existing:
        og = _get_og_for_variant(entry.variant, og_by_name)
        if _should_drop_entry(og):
            continue
        rendered.append(_format_entry(entry, rendered_count, og))
        rendered_count += 1

    if len(rendered) > MAX_PRESET_INDEX_CARDINALITY:
        raise RuntimeError(
            "Rendered preset count exceeds u8 index capacity "
            f"({len(rendered)} > {MAX_PRESET_INDEX_CARDINALITY})"
        )

    return "\n".join(rendered)


def _replace_macro_payload(rust_src: str, rendered_entries: str) -> str:
    m = ENUMERATE_MACRO_RE.search(rust_src)
    if not m:
        raise RuntimeError("Could not locate enumerate_specs![] payload in target file")
    replacement = f"enumerate_specs![\n{rendered_entries}\n];"
    return rust_src[: m.start()] + replacement + rust_src[m.end() :]


def _replace_pyi_presets_block(pyi_src: str, rendered_stub: str) -> str:
    m = PYI_PRESETS_BLOCK_RE.search(pyi_src)
    if not m:
        raise RuntimeError(
            "Could not locate class Presets(IntEnum) block in img_gen.pyi"
        )
    replacement = f"class Presets(IntEnum):\n{rendered_stub}\n\nclass ColorGradient:"
    return pyi_src[: m.start()] + replacement + pyi_src[m.end() :]


def _diff_summary(old_src: str, new_src: str) -> tuple[int, int]:
    old_lines = old_src.splitlines()
    new_lines = new_src.splitlines()
    changed = sum(1 for a, b in zip(old_lines, new_lines) if a != b)
    changed += abs(len(old_lines) - len(new_lines))
    return changed, len(new_lines)


def _semantic_mismatches(
    existing: tuple[ExistingEntry, ...],
    og_by_name: dict[str, list[OgGradient]],
) -> list[str]:
    msgs: list[str] = []
    for entry in existing:
        og = _get_og_for_variant(entry.variant, og_by_name)
        if og is None:
            continue
        if _should_drop_entry(og):
            continue

        old_stops = entry.stops
        new_stops = og.stops
        if len(old_stops) != len(new_stops):
            msgs.append(
                f"{entry.variant}: stop count {len(old_stops)} -> {len(new_stops)}"
            )
            continue

        mismatch: tuple[int, Stop, Stop] | None = None
        for idx, (old, new) in enumerate(zip(old_stops, new_stops), start=1):
            if _stops_equal(old, new):
                continue
            mismatch = (idx, old, new)
            break
        if mismatch is not None:
            idx, old, new = mismatch
            msgs.append(
                f"{entry.variant}: stop {idx} ({old.color}, {old.pos}) -> ({new.color}, {new.pos})"
            )
    return msgs


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--file", type=Path, default=DEFAULT_RS_FILE, help="Path to gradient_presets.rs"
    )
    parser.add_argument(
        "--json-url", default=DEFAULT_JSON_URL, help="Upstream gradients.json URL"
    )
    parser.add_argument(
        "--css-url", default=DEFAULT_CSS_URL, help="Upstream webgradients.css URL"
    )
    parser.add_argument(
        "-c",
        "--check",
        action="store_true",
        help="Exit non-zero if generated output differs from current file",
    )
    parser.add_argument(
        "-w",
        "--write",
        action="store_true",
        help="Write generated output back into --file",
    )
    parser.add_argument(
        "-p",
        "--print",
        action="store_true",
        dest="print_output",
        help="Print generated enumerate_specs payload to stdout",
    )
    parser.add_argument(
        "-s",
        "--update-stubs",
        action="store_true",
        help="Rewrite the Presets IntEnum block in img_gen.pyi",
    )
    parser.add_argument(
        "--stub-file",
        type=Path,
        default=DEFAULT_PYI_FILE,
        help="Path to img_gen.pyi",
    )

    args = parser.parse_args(argv)

    rust_src = args.file.read_text(encoding="utf-8")
    existing = _ensure_required_local_presets(_parse_existing_entries(rust_src))

    if not existing:
        raise RuntimeError("No entries were parsed from target rust file")

    og_json = _fetch_text(args.json_url)
    og_css = _fetch_text(args.css_url)
    og_by_name = _parse_og_gradients(og_json, og_css)

    semantic = _semantic_mismatches(existing, og_by_name)
    rendered_entries = _render_entries(existing, og_by_name)
    new_src = _replace_macro_payload(rust_src, rendered_entries)
    rendered_stub = _render_stub_presets(existing, og_by_name)

    changed, total = _diff_summary(rust_src, new_src)
    print(f"Parsed entries: {len(existing)}")
    print(f"Generated lines: {total}")
    print(f"Changed lines: {changed}")
    print(f"Semantic mismatches: {len(semantic)}")
    for item in semantic:
        print(f" - {item}")

    if args.print_output:
        print("\n--- generated enumerate_specs payload ---\n")
        print(rendered_entries)

    if args.write:
        args.file.write_text(new_src, encoding="utf-8")
        print(f"Wrote updated presets to {args.file}")

    if args.update_stubs:
        pyi_src = args.stub_file.read_text(encoding="utf-8")
        new_pyi_src = _replace_pyi_presets_block(pyi_src, rendered_stub)
        args.stub_file.write_text(new_pyi_src, encoding="utf-8")
        print(f"Wrote updated preset stubs to {args.stub_file}")

    if args.check and changed:
        print("Generated presets differ from the current file.")
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
