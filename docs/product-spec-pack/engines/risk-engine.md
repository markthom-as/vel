# Risk Engine

The Risk Engine estimates probability of commitment failure.

## Inputs

- due time
- travel time
- dependencies
- user habits

## Example Risk Calculation

```
risk = proximity_weight * time_remaining
     + dependency_weight * dependency_count
     + commute_weight * travel_time
```

Risk is recalculated continuously.