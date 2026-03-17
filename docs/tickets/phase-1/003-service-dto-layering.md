---
title: Service/DTO Boundary & Domain Mapping Enforcement
status: in-progress
owner: staff-eng
type: maintainability
priority: high
created: 2026-03-17
labels:
  - veld
  - routes
  - mapping
---

Enforce a strict layering between API routes (DTOs) and business logic (Services), ensuring that services only return domain entities and routes handle the network wire format mapping.

## Technical Details
- **Mapping Layer**: Standardize `From<DomainType> for DtoType` implementations for all major API resources.
- **Service Refactor**: Update all service methods to return types from `vel-core` or specific `ApplicationResult` structs.
- **Route Isolation**: Ensure `axum` handlers are thin and only perform deserialization, service calls, and DTO mapping.
- **Error Mapping**: Map domain-specific `thiserror` enums to standard `AppError` responses.

## Acceptance Criteria
- No service imports `vel-api-types`.
- All `axum` routes are < 30 lines (excluding documentation).
- API response formats remain consistent with existing DTOs.
- All integration tests pass.
