# vel — Dogfooding-Focused Feedback and Next-Step Plan

## Overall assessment

This is the first revision where Vel feels close to being **useful in lived practice**, not just architecturally convincing.

That is a meaningful threshold.

The core runtime loop is now present and exercised:

```text
capture
  → snapshot
  → run
  → artifact
  → provenance
  → inspection
```

That means the question changes.

The earlier question was:

> is the architecture real?

The current question is:

> what is still missing before a human can use Vel every day, and then use Vel to improve Vel?

That is a much better question. It is also less glamorous and more important.

The good news is that the architecture is now strong enough that the next tranche of work should be driven primarily by **dogfooding ergonomics**, not by foundational redesign.

In other words: the skeleton is standing. Now you need enough organs, hands, and senses to actually live inside the beast.

---

# High-level verdict

## What is clearly working

- the runtime substrate is real
- context generation is run-backed
- artifacts and refs now have practical purpose
- inspection surfaces are meaningful
- docs are mostly honest and hierarchical
- crate boundaries are disciplined

## What is still missing before Vel becomes genuinely usable

The remaining gaps are no longer about core architecture. They are about **operator experience, ingestion coverage, retrieval ergonomics, and daily loop closure**.

You can already demonstrate Vel.

You are not *quite* at the point where Vel can become your default daily substrate without friction.

That is the next milestone.

---

# The main strategic recommendation

From here, I would stop optimizing primarily for architectural neatness and start optimizing for:

> **Can Jove actually use this thing every day to capture, review, orient, and improve the system from inside the system?**

That implies a product sequence centered on dogfooding, not on abstract feature completion.

The next roadmap should therefore prioritize:

1. **frictionless capture**
2. **usable review / browse surfaces**
3. **daily and weekly orientation loops**
4. **basic ingestion/import**
5. **safe export / backup**
6. **first synthesis workflow that helps improve Vel itself**

---

# The most important observation from this round

## Vel is still strong at “runtime truth” and weak at “daily operator loop”

The repository now has:

- capture creation
- lexical search
- run-backed context generation
- artifact persistence
- run inspection
- provenance

That is excellent.

But the practical daily loop still looks incomplete.

Right now, from a user/operator perspective, Vel seems strongest at:

- capturing one thing
- searching by string
- generating orientation snapshots
- inspecting run internals

What is still relatively weak is:

- reviewing recent captures
- scanning what changed today
- triaging / promoting captures
- finding “what matters” without knowing the exact query
- creating feedback loops where using Vel naturally generates input for improving Vel

That is the next real product frontier.

---

# Immediate issues I would fix

## 1. README currently contradicts itself

This is small, but it matters.

The current `README.md` says both:

- **Planned next:** run-backed context generation
- and later that context endpoints **are run-backed**

That is now stale.

This kind of contradiction is not fatal, but it is corrosive because it weakens trust in the repo’s self-description.

### Recommendation

Fix the README immediately.

Change the “Planned next” section to what is actually next now, which I would make something like:

- recent/review flows
- synthesis workflows
- import/ingestion
- usability improvements for daily operation

Do not let the docs lag a milestone that has already landed.

---

# The dogfooding roadmap I would implement next

This is the roadmap I would use if the goal is:

> start using Vel now, and use Vel to improve Vel

---

# Phase 1 — Make daily use frictionless

This phase is about removing the basic barriers to using Vel constantly.

## 1. Add `vel recent`

Right now you can:

- capture
- search
- inspect by known id

But there is no obvious “show me what I captured recently” operator command.

That is a major gap.

### Add:

```bash
vel recent
vel recent --limit 20
vel recent --today
vel recent --json
```

### It should show:

- capture id
- time
- type
- source device
- truncated content

### Why this matters

A system without a recent/review surface forces the user to remember what they already entered. That defeats part of the point of having the system.

This is probably the single highest-value operator feature after capture and search.

---

## 2. Add `vel inspect artifact <id>`

Artifacts now matter. They are durable outputs.

But the CLI/operator surface still appears centered more heavily on runs and captures.

### Add:

```bash
vel inspect artifact <id>
```

### It should show:

- artifact id
- artifact type
- storage kind
- path / uri
- size
- hash
- metadata
- linked run/ref summaries if available

### Why

Once Vel starts being used daily, artifacts become first-class things to inspect, not merely byproducts of runs.

---

## 3. Improve `vel capture` ergonomics

Current capture is minimal and clean, which is good. But for daily use you want slightly richer input without making it annoying.

### Add optional flags:

```bash
vel capture "remember lidar budget" --type note --source laptop
vel capture "check IRS mailing address" --type todo
```

You may already support some of this in the API, but the CLI should make it easy.

### Why

Dogfooding will produce different classes of captures:

- fleeting idea
- TODO
- project note
- bug
- decision
- quote
- question

If everything goes in as undifferentiated text forever, retrieval gets mushy fast.

My recommendation is **not** to create a huge ontology. Just enough type affordance to improve later review.

---

## 4. Add shell-friendly capture from stdin

This is a big one for actual use.

### Add support for:

```bash
echo "remember to test context refs" | vel capture -
pbpaste | vel capture -
```

or a dedicated:

```bash
vel capture --stdin
```

### Why

This is one of the fastest ways to make Vel actually live in the command-line workflow rather than merely be callable from it.

And once you are using Vel to improve Vel, lots of useful captures will come from:
- commit notes
- snippets
- bug observations
- design thoughts
- command output summaries

stdin capture is a very high leverage move.

---

# Phase 2 — Close the daily review loop

If Phase 1 is about getting information in, Phase 2 is about making it easy to look back without doing archaeology.

## 1. Add `vel review today`

Current context commands are useful, but a review-oriented surface would help separate:

- orientation
- retrospective review

### Add:

```bash
vel review today
vel review week
```

### A plausible first version could include:

- count of captures
- top recurring terms
- recent commitments
- recent runs
- most recent context artifact

This does not need to be fancy. It just needs to let the user look back and think.

### Why

You want Vel to become a system you consult, not only a system you feed.

---

## 2. Add `vel recent-runs`

You already have `vel runs`, which is runtime-oriented. That is good.

But dogfooding may benefit from a more task-oriented or timeframe-oriented view:

```bash
vel recent-runs --kind context_generation
vel recent-runs --today
```

This is lower priority than `vel recent`, but useful.

---

## 3. Add `vel artifact latest --type context_brief`

Sometimes the most useful question is not “what run happened?” but “what is the latest generated orientation output?”

### Example:

```bash
vel artifact latest --type context_brief
```

This would reduce friction in actually reading the system’s outputs.

---

# Phase 3 — Add the minimum synthesis needed to improve Vel from inside Vel

This is where dogfooding becomes recursive in the good way.

The question here is:

> what is the smallest synthesis feature that helps the user improve Vel using data already inside Vel?

The answer is probably **not** a giant agent system yet.

It is probably one of these:

---

## 1. Add `vel synthesize week`

This should produce a run-backed synthesis artifact from recent captures + context artifacts.

### Initial output could include:

- recurring concerns
- open commitments
- repeated pain points
- possible product improvements
- things captured often but never acted on

### Why this is the correct first synthesis

Because you specifically want to use Vel to improve Vel.

A weekly synthesis can act like a reflective design note generator.

That is extremely on-brand and actually useful.

---

## 2. Add `vel synthesize project vel`

Even if crude at first.

### Behavior:

Search or gather captures that mention Vel-related terms or are tagged as project notes, then synthesize:

- unresolved issues
- architecture friction
- repeated user pain
- likely next priorities

This is the first truly recursive workflow:
Vel helping evolve Vel.

That is a much better next move than “agent mode” in the abstract.

---

# Phase 4 — Improve ingestion

Right now Vel looks strongest at manual capture. That is correct for MVP. But to become part of life, it needs at least one or two easy ingestion paths.

I would keep this modest and practical.

## 1. Add file import for plaintext / markdown

### Example:

```bash
vel import file notes/today.md
vel import file bugs.txt --type note
```

### Why

This immediately broadens Vel’s usable intake surface without introducing huge complexity.

It also helps seed the system quickly for dogfooding.

---

## 2. Add URL capture as a first-class operator path

Instead of only generic text capture, allow:

```bash
vel capture-url https://example.com/article
```

Even if the first version simply stores the URL as a capture/artifact reference with minimal metadata.

### Why

Real usage includes links constantly. If links require awkward manual handling, users stop bothering.

---

## 3. Add import from newline-delimited stdin

### Example:

```bash
cat scratchpad.txt | vel import lines --type note
```

This is a very practical way to seed or migrate thoughts into Vel without fancy connectors.

---

# Phase 5 — Backup, export, and trust

If Vel is going to hold real daily cognition, it needs a minimum trust surface.

## 1. Add `vel export`

### Example:

```bash
vel export --format json
vel export --captures
vel export --runs
vel export --artifacts
```

### Why

Users trust systems more when they can get their data back out.

And for your specific use case, export is also useful for:
- analysis
- archiving
- external tooling
- repo-based self-reflection

---

## 2. Add `vel backup`

Even if it is initially a thin wrapper around:
- SQLite backup
- artifact directory copy
- manifest JSON

### Why

If you want to use Vel seriously, you want the feeling that it is not one bad disk moment away from symbolic castration.

---

# Concrete features I would implement next

If I were steering the next set of patches, I would prioritize these exact commands and features.

## Tier 1 — immediately useful
- `vel recent`
- `vel capture --stdin`
- `vel capture` with optional type/source flags
- `vel inspect artifact <id>`
- README/status cleanup

## Tier 2 — closes review loop
- `vel review today`
- `vel review week`
- `vel artifact latest --type context_brief`

## Tier 3 — enables recursive improvement
- `vel synthesize week`
- `vel synthesize project vel`

## Tier 4 — trust and intake
- `vel import file`
- `vel import lines`
- `vel capture-url`
- `vel export`
- `vel backup`

This is the order I would use if the goal is real adoption rather than further architectural theater.

---

# Architectural guidance for the dogfooding phase

Now that Vel is usable enough to start living inside, a few architectural rules become especially important.

## 1. Do not let “review” bypass the runtime model

When you add:

- review
- synthesis
- weekly summaries

make them run-backed too.

You already paid the cost to build the runtime substrate. Use it.

That means:
- create run
- write artifact
- create refs
- inspect it later

This is how the system becomes self-consistent.

---

## 2. Keep manual capture stupidly easy

The temptation will be to add too much structure too early.

Resist it.

A good rule:

- capture should stay fast
- classification can be optional
- synthesis can do some structuring later

If capture becomes annoying, usage drops, and all the architecture in the world becomes an empty theater.

---

## 3. Prefer one or two high-value import paths over many connectors

Do not build five integrations just because they sound useful.

Build the ones that let **you** start using Vel right away:
- file import
- stdin / line import
- maybe URL capture

That is enough to start generating real data.

---

## 4. Add just enough retrieval surface to reduce “unknown unknowns”

Search is already there. Good.

But search assumes the user knows what to ask.

A usable cognition runtime also needs:
- recent
- review
- latest artifact
- maybe “open threads” later

These are browsing/orientation surfaces, not query surfaces.

They matter a lot.

---

# Docs changes I would make right now

## 1. Fix README contradiction immediately

As noted above, this is stale and should be corrected now.

## 2. Add a “Using Vel Daily” doc

Create:

```text
docs/using-vel-daily.md
```

It should describe a simple workflow like:

### Morning
- `vel morning`
- `vel recent`

### Throughout the day
- `vel capture ...`
- `vel search ...`

### End of day
- `vel end-of-day`
- `vel review today`

### Weekly
- `vel synthesize week`

This is much more valuable now than another abstract architecture note.

## 3. Add a “Dogfooding roadmap” section to `status.md`

Something like:

```markdown
## Next to make Vel usable daily

- recent/review flows
- stdin/file import
- synthesis for weekly reflection
- export/backup
```

That would align development with actual use.

---

# Suggested implementation sequence

Here is the exact sequence I would recommend now.

## Phase 1 — operator usability
1. fix README/status truthfulness
2. add `vel recent`
3. add `vel capture --stdin`
4. add optional capture type/source flags
5. add `vel inspect artifact <id>`

## Phase 2 — daily review loop
1. add `vel review today`
2. add `vel review week`
3. add `vel artifact latest --type context_brief`

## Phase 3 — recursive self-improvement
1. add `vel synthesize week`
2. add `vel synthesize project vel`
3. make both run-backed with managed artifacts

## Phase 4 — ingestion and trust
1. add `vel import file`
2. add `vel import lines`
3. add `vel capture-url`
4. add `vel export`
5. add `vel backup`

This sequence gives you the fastest path from “working runtime” to “system I actually rely on.”

---

# One product principle I would explicitly adopt now

I would write this down somewhere canonical:

> **Vel should optimize for repeated personal use before broad generality.**

That means:
- better daily loops before fancy automation
- better capture/review ergonomics before agent complexity
- better trust/export before speculative integrations

This is the right order if you want to use Vel to improve Vel.

---

# Final assessment

Vel now looks close enough to reality that the next best move is not more architecture polishing.

It is dogfooding.

The repository is ready for a shift in emphasis:

from

> “can this architecture support a cognition runtime?”

to

> “what does it still need so I can actually live in it every day and let it teach me what to build next?”

That is where the most valuable feedback will now come from: not from speculative design purity, but from use.

And honestly, that is when software starts getting interesting.