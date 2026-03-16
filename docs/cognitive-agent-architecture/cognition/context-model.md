# Context Model

Context is the input substrate for Vel's reasoning.

## Context Domains

- time
- calendar
- location
- device
- activity
- commitments
- routines
- system health

## Context Snapshot Schema

```json
{
  "timestamp": "ISO8601",
  "time_context": {
    "local_time": "",
    "day_of_week": "",
    "upcoming_windows": []
  },
  "calendar_context": {
    "next_events": []
  },
  "location_context": {
    "current_location": null,
    "travel_estimates": []
  },
  "device_context": {
    "active_surface": "",
    "available_modalities": []
  },
  "system_health": {
    "integration_status": {},
    "confidence_penalties": []
  }
}
```

## Rule

Context is not raw telemetry worship.
It must be transformed into decision-relevant abstractions.
