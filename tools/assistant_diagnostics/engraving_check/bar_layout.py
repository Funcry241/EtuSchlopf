"""Locate piano systems and measure intervals in the rendered PDF."""
from xml.etree import ElementTree


def horiz_staff(drawings):
    grouped = {}
    for drawing in drawings:
        if drawing["type"] != "s" or len(drawing["items"]) != 1:
            continue
        tag, *points = drawing["items"][0]
        if tag != "l":
            continue
        a, b = points
        if abs(a.y - b.y) < .02 and abs(b.x - a.x) > 32:
            grouped.setdefault(round(a.y, 2), []).append(tuple(sorted((a.x, b.x))))

    rows = []
    for y, spans in grouped.items():
        spans.sort()
        left, right = spans[0][0], spans[-1][1]
        if right - left >= 325 and sum(end - start for start, end in spans) >= 325:
            rows.append((y, left, right, spans))
    return sorted(rows)


def spaced_staves(rows):
    staves = []
    for i in range(len(rows) - 4):
        sample = rows[i:i + 5]
        gaps = [sample[j + 1][0] - sample[j][0] for j in range(4)]
        if not (min(gaps) > 2 and max(gaps) < 8 and max(gaps) - min(gaps) < .09):
            continue
        top, left, right, spans = sample[0]
        if any(abs(row[1] - left) > .3 or abs(row[2] - right) > .3 for row in sample):
            continue
        staves.append((top, sample[-1][0], left, right, sum(gaps) / 4, spans))
    return staves


def systems_xml(xml):
    root = ElementTree.fromstring(xml)
    parts = root.findall("part")
    if root.tag != "score-partwise" or len(parts) != 1:
        raise ValueError("expected score-partwise MusicXML for a single piano part")
    pages = [[[]]]
    for measure in parts[0].findall("measure"):
        layout = measure.find("print")
        if layout is not None and pages[-1][-1]:
            if layout.get("new-page") == "yes":
                pages.append([[]])
            elif layout.get("new-system") == "yes":
                pages[-1].append([])
        pages[-1][-1].append(measure)
    if not pages[0][0]:
        raise ValueError("MusicXML has no measures")
    return pages


def barlines(drawings, upper, lower):
    top, _, left, right, _, spans = upper
    candidates = [left, right, *(x for span in spans for x in span)]
    for drawing in drawings:
        if drawing["type"] != "s" or len(drawing["items"]) != 1:
            continue
        tag, *points = drawing["items"][0]
        if tag != "l":
            continue
        a, b = points
        if abs(a.x - b.x) > .02:
            continue
        y0, y1 = sorted((a.y, b.y))
        if left - 1 <= a.x <= right + 1 and y0 <= top + 1 and y1 >= lower[1] - 1:
            candidates.append(a.x)

    boundaries = []
    for x in sorted(candidates):
        if not boundaries or x - boundaries[-1] > 4.8:
            boundaries.append(x)
        else:
            boundaries[-1] = max(boundaries[-1], x)
    return boundaries
