# Suggestion Engine

The Suggestion Engine produces candidate actions for the user.

## Inputs

- commitments
- calendar
- context state
- risk calculations

## Ranking

Suggestions are ranked by:

```
importance
urgency
confidence
user preference
```

## Threshold

A suggestion is surfaced if:

```
confidence > threshold
AND
risk > threshold
```