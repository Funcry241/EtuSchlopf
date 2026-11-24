// SPDX-License-Identifier: GPL-3.0-only
// Satzfix extension for MuseScore Studio, 2026-11-23.

#pragma once

#include <array>
#include <cmath>
#include <cstdlib>
#include <locale>
#include <optional>
#include <sstream>
#include <string>
#include <vector>

namespace etuschlopf {
using SlurPoint = std::array<double, 2>;
using SlurRibbon = std::array<SlurPoint, 7>;

struct SlurPaintEdit {
    std::size_t page;
    SlurRibbon before;
    SlurRibbon after;
};

class SlurPaintPlan
{
public:
    static SlurPaintPlan load()
    {
        const auto* payload = std::getenv("ETUSCHLOPF_SLUR_PAINT_PLAN");
        if (!payload || !*payload) {
            return {};
        }
        std::istringstream input(payload);
        return parse(input);
    }

    static SlurPaintPlan parse(std::istream& input)
    {
        std::string line;
        if (!std::getline(input, line) || line != "etuschlopf-slur-paint-v1") {
            return {};
        }
        SlurPaintPlan plan;
        while (std::getline(input, line)) {
            std::istringstream row(line);
            row.imbue(std::locale::classic());
            SlurPaintEdit edit{};
            if (!(row >> edit.page) || !readRibbon(row, edit.before) || !readRibbon(row, edit.after)) {
                return {};
            }
            std::string extra;
            if (row >> extra) {
                return {};
            }
            plan.m_edits.push_back(edit);
        }
        return input.eof() ? plan : SlurPaintPlan{};
    }

    bool empty() const { return m_edits.empty(); }

    struct Match {
        std::size_t index;
        const SlurPaintEdit& edit;
    };

    std::optional<Match> find(std::size_t page, const SlurRibbon& ribbon) const
    {
        // MuseScore rounds the coordinates in an SVG preview. Match every
        // coordinate in page units, and refuse an ambiguous registration.
        constexpr double registrationTolerance = 0.08;
        std::optional<std::size_t> index;
        for (std::size_t i = 0; i < m_edits.size(); ++i) {
            const auto& candidate = m_edits[i];
            if (page != candidate.page) {
                continue;
            }
            bool same = true;
            for (std::size_t j = 0; j < ribbon.size(); ++j) {
                for (std::size_t axis = 0; axis < 2; ++axis) {
                    same &= std::abs(ribbon[j][axis] - candidate.before[j][axis]) <= registrationTolerance;
                }
            }
            if (same) {
                if (index) {
                    return std::nullopt;
                }
                index = i;
            }
        }
        if (index) {
            return Match{*index, m_edits[*index]};
        }
        return std::nullopt;
    }

private:
    static bool readRibbon(std::istream& input, SlurRibbon& ribbon)
    {
        for (auto& point : ribbon) {
            for (double& coordinate : point) {
                if (!(input >> coordinate) || !std::isfinite(coordinate)) {
                    return false;
                }
            }
        }
        return ribbon.front() == ribbon.back() && ribbon[3][0] > ribbon[0][0];
    }

    std::vector<SlurPaintEdit> m_edits;
};
}
