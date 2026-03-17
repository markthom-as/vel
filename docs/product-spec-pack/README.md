# Vel Product Spec Pack

This repository defines the **experience layer** for Vel — the surfaces, interaction contracts,
and design language that agents must obey when implementing features.

Vel is not just a tool — it is an **operating layer for commitments, context, and decisions**.

## Structure

```
vel-product-spec-pack/
  README.md
  surfaces/
  interaction/
  engines/
  design/
  flows/
  architecture/
  imported/
```

## Purpose

This spec ensures:

- conceptual integrity across surfaces
- consistent interaction behavior
- predictable suggestion and risk behavior
- safe evolution via Vel's introspective self‑improvement mode

Agents implementing Vel should treat these documents as **normative specifications**.

Authority note:

- For shipped behavior, [docs/status.md](../status.md) remains canonical.
- Treat this pack as design guidance and future-facing product structure unless `docs/status.md` says a behavior is implemented.
- Imported packets under `imported/` preserve upstream design material and may conflict with current code or current repo authority.
