"""Read-only geometry for engraved noteheads, stems, and beam bands."""
from dataclasses import dataclass
from enum import Enum


@dataclass(frozen=True)
class Stem:
    x: float
    top: float
    bottom: float
    width: float

    def length(self, staff_space: float) -> float:
        return (self.bottom - self.top) / staff_space


@dataclass(frozen=True)
class Beam:
    left: float
    right: float
    top_left: float
    bottom_left: float
    top_right: float
    bottom_right: float

    def span_at(self, x: float) -> tuple[float, float]:
        t = min(1.0, max(0.0, (x - self.left) / (self.right - self.left)))
        top = self.top_left + t * (self.top_right - self.top_left)
        bottom = self.bottom_left + t * (self.bottom_right - self.bottom_left)
        return top, bottom


@dataclass(frozen=True)
class Head:
    left: float
    top: float
    right: float
    bottom: float

    @property
    def center(self) -> tuple[float, float]:
        return (self.left + self.right) / 2, (self.top + self.bottom) / 2


class Contact(Enum):
    FOREIGN_HEAD = "foreign_head"
    OWN_HEAD = "own_head"
    UNKNOWN_ATTACHMENT = "unknown_attachment"


def head_joins_stem(head: Head, stem: Stem, space: float) -> bool:
    # A shared chord stem can pass the y level of several head centers.
    # It must stay on an edge of each such head, with a usable extension.
    if min(abs(stem.x - head.left), abs(stem.x - head.right)) > stem.width / 2 + .12 * space:
        return False
    center_y = head.center[1]
    endpoint_tolerance = .30 * space
    if not stem.top - endpoint_tolerance <= center_y <= stem.bottom + endpoint_tolerance:
        return False
    return max(center_y - stem.top, stem.bottom - center_y) >= 1.25 * space


def beam_joins_stem(
    beam: Beam, stem: Stem, space: float, head_centers: list[float],
) -> bool:
    if not beam.left - stem.width / 2 - .10 * space <= stem.x <= beam.right + stem.width / 2 + .10 * space:
        return False
    top, bottom = beam.span_at(stem.x)
    epsilon = .14 * space
    if stem.bottom < top - epsilon or stem.top > bottom + epsilon:
        return False
    # Contact next to the attached notehead is not an attachment of the
    # opposite, free end. Secondary beam bands may be short of that end.
    if head_centers:
        clearance = .35 * space
        return bottom < min(head_centers) - clearance or top > max(head_centers) + clearance
    return True


def crosses_head_center(beam: Beam, head: Head, space: float) -> bool:
    x, y = head.center
    # The central-ink condition deliberately omits marginal contacts. Use it
    # only for solid heads from the supported font, not for hollow heads.
    if not beam.left + .25 * space < x < beam.right - .25 * space:
        return False
    top, bottom = beam.span_at(x)
    return top <= y <= bottom


def group_stems(beam_members: list[set[int]]) -> list[set[int]]:
    components: list[set[int]] = []
    for members in beam_members:
        if not members:
            continue
        linked = [component for component in components if component & members]
        if not linked:
            components.append(set(members))
            continue
        merged = set(members)
        for component in linked:
            merged.update(component)
            components.remove(component)
        components.append(merged)
    return components


def attach_heads(heads: list[Head], stems: list[Stem], space: float) -> list[set[int]]:
    return [
        {i for i, stem in enumerate(stems) if head_joins_stem(head, stem, space)}
        for head in heads
    ]


def attach_beams(
    beams: list[Beam],
    stems: list[Stem],
    space: float,
    heads: list[Head],
    head_stems: list[set[int]],
    flag_stems: set[int] | frozenset[int] = frozenset(),
) -> list[set[int]]:
    centers: list[list[float]] = [[] for _ in stems]
    for head, matches in zip(heads, head_stems):
        for i in matches:
            centers[i].append(head.center[1])
    return [
        {
            i for i, stem in enumerate(stems)
            if i not in flag_stems and beam_joins_stem(beam, stem, space, centers[i])
        }
        for beam in beams
    ]


def classify_contacts(
    heads: list[Head],
    beams: list[Beam],
    head_stems: list[set[int]],
    beam_stems: list[set[int]],
    space: float,
) -> list[tuple[int, int, Contact]]:
    findings = []
    for i, head in enumerate(heads):
        for j, beam in enumerate(beams):
            if not crosses_head_center(beam, head, space):
                continue
            if head_stems[i] & beam_stems[j]:
                verdict = Contact.OWN_HEAD
            elif len(head_stems[i]) == 1 and beam_stems[j]:
                verdict = Contact.FOREIGN_HEAD
            else:
                verdict = Contact.UNKNOWN_ATTACHMENT
            findings.append((i, j, verdict))
    return findings
