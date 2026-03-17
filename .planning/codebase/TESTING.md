# Testing Patterns

**Analysis Date:** 2026-03-17

## Test Framework

**Runner:**
- **Rust:** cargo test (built-in). No explicit test runner configuration; tests live inline in modules via `#[cfg(test)]`.
- **TypeScript:** Vitest 2.1.8 with jsdom environment
- **Config:** `clients/web/vitest.config.ts`

**Assertion Library:**
- **Rust:** Standard `assert_eq!`, `assert!` macros
- **TypeScript:** Vitest expect API (compatible with Jest)

**Run Commands:**
```bash
# Rust
cargo test --workspace --all-features           # Run all workspace tests
cargo test -p <crate> <test_name>               # Run single test in crate

# TypeScript
cd clients/web && npm run test                  # Run all tests (single run)
cd clients/web && npm run test:watch            # Watch mode
```

**Coverage:**
- No coverage target or enforcement detected.
- Command to measure (if desired): `npx vitest --coverage` (requires coverage provider package)

## Test File Organization

**Location:**
- **Rust:** Co-located with source. Tests in same file within `#[cfg(test)] mod tests { ... }` block.
  - Example: `crates/vel-cli/src/command_lang/parse.rs` has `#[cfg(test)] mod tests` at line 43
- **TypeScript:** Co-located with source. Tests in sibling file with `.test.ts` or `.test.tsx` suffix.
  - Example: `src/api/client.ts` has sibling `src/api/client.test.ts`
  - Components: `src/components/MessageComposer.tsx` has `src/components/MessageComposer.test.tsx`

**Naming:**
- **Rust:** Test functions named with `test_` prefix or descriptive name (e.g., `parses_should_capture`, `rejects_unknown_family`)
- **TypeScript:** Test functions named descriptively (e.g., `applies the provided decoder to GET responses`, `disables Send when text is empty`)

**Setup/Teardown:**
- **Rust:** None required for unit tests; each test is isolated.
- **TypeScript:**
  - Global setup: `clients/web/src/test/setup.ts` imports `@testing-library/jest-dom/vitest`
  - Per-test cleanup: `afterEach(() => { vi.restoreAllMocks() })`

## Test Structure

**Rust Suite Organization:**
```rust
#[cfg(test)]
mod tests {
    use super::parse;
    use crate::command_lang::ast::{PhraseFamily, Verb};

    #[test]
    fn parses_should_capture() {
        let input = vec!["should".to_string(), "capture".to_string(), "remember".to_string()];
        let parsed = parse(&input).expect("parse");
        assert_eq!(parsed.family, PhraseFamily::Should);
        assert_eq!(parsed.verb, Verb::Capture);
        assert_eq!(parsed.target_tokens, vec!["remember"]);
    }

    #[test]
    fn rejects_unknown_family() {
        let input = vec!["hello".to_string(), "world".to_string()];
        assert!(parse(&input).is_err());
    }
}
```

**TypeScript Suite Organization:**
```typescript
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { apiGet, apiPost } from './client'

describe('api client decoders', () => {
  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('applies the provided decoder to GET responses', async () => {
    vi.spyOn(globalThis, 'fetch').mockResolvedValue({
      ok: true,
      json: async () => ({ ok: true, data: {...}, meta: {...} }),
    } as Response)

    const response = await apiGet<ApiResponse<ConversationData>>(
      '/api/conversations/conv_1',
      (value) => decodeApiResponse(value, decodeConversationData),
    )

    expect(response.data?.id).toBe('conv_1')
  })
})
```

**Patterns:**
- **Setup:** `beforeEach()` to clear mocks or reset state
- **Teardown:** `afterEach(() => vi.restoreAllMocks())` to clean up all mocks
- **Assertion:** Positive assertions (what should exist/be true) checked first, then error cases
- **Naming:** Test names are complete sentences describing behavior (e.g., "disables Send when text is empty" not "button disabled")

## Mocking

**Framework:**
- **Rust:** No explicit mocking framework. Tests use direct function calls; integration tests exercise real storage via test databases.
- **TypeScript:** Vitest's `vi` module for spying and mocking

**Patterns:**

**Rust (no mocking):**
- Tests call functions directly with constructed inputs
- Example from `crates/vel-cli/src/command_lang/parse.rs`:
  ```rust
  #[test]
  fn parses_should_capture() {
      let input = vec!["should".to_string(), "capture".to_string(), "remember".to_string()];
      let parsed = parse(&input).expect("parse");
      assert_eq!(parsed.family, PhraseFamily::Should);
  }
  ```

**TypeScript (mocking via vi):**
```typescript
// Mock entire module
vi.mock('../api/client', () => ({
  apiPost: vi.fn(),
}))

// Mock specific fetch calls
vi.spyOn(globalThis, 'fetch').mockResolvedValue({
  ok: true,
  json: async () => ({ ok: true, data: {...}, meta: {...} }),
} as Response)

// Verify mock was called with specific args
expect(api.apiPost).toHaveBeenCalledWith(
  '/api/conversations/conv_1/messages',
  { role: 'user', kind: 'text', content: { text: 'Hi' } },
  expect.any(Function),
)

// Reset mocks
vi.mocked(api.apiPost).mockReset()
```

**What to Mock:**
- External API calls (fetch)
- File system operations (not used in web tests)
- Timer functions (not used in current tests)
- Module exports that are dependencies (e.g., `apiPost` in component tests)

**What NOT to Mock:**
- Core business logic (decoders, response validation)
- TypeScript utility functions
- React hooks behavior (use real React for Hook testing)
- Internal component state (test behavior through rendered output)

## Fixtures and Factories

**Test Data:**

**TypeScript:**
```typescript
// Inline fixture construction in tests
const mockUserMessage = {
  id: 'msg_1',
  conversation_id: 'conv_1',
  role: 'user',
  kind: 'text',
  content: { text: 'Hi' },
  status: null,
  importance: null,
  created_at: 0,
  updated_at: null,
}

// Or mock return value
vi.mocked(api.apiPost).mockResolvedValue({
  ok: true,
  data: { user_message: mockUserMessage, assistant_message: null },
  meta: { request_id: 'req_1' },
})
```

**Rust:**
- No fixture framework. Tests construct input structs directly.
- Example:
  ```rust
  let input = MoodJournalInput {
      score: 7,
      label: Some("good".to_string()),
      note: None,
      source_device: None,
  };
  ```

**Location:**
- TypeScript: Inline in test file (no separate fixture files currently)
- Rust: Inline in test module

## Coverage

**Requirements:** No explicit coverage target or enforcement.

**View Coverage (if implemented):**
```bash
cd clients/web && npx vitest --coverage
```

## Test Types

**Unit Tests:**

**Rust:**
- Scope: Single function or small module. Example: `parse()` function tests in `crates/vel-cli/src/command_lang/parse.rs`
- Approach: Call function directly with various inputs (valid, edge cases, errors), assert results
- Example patterns:
  ```rust
  #[test]
  fn parses_should_capture() { ... }

  #[test]
  fn rejects_unknown_family() { ... }
  ```

**TypeScript:**
- Scope: Single function or component. Examples: API client decoders, decoder functions, component behavior
- Approach: Mock dependencies, render component or call function, assert output
- Files: `client.test.ts`, `types.test.ts`, component `.test.tsx` files
- Example from `clients/web/src/components/MessageComposer.test.tsx`:
  ```typescript
  it('renders textarea and Send button', () => {
    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} />)
    expect(container.querySelector('textarea')).toBeInTheDocument()
    expect(within(composer).getByRole('button', { name: /send/i })).toBeInTheDocument()
  })

  it('calls apiPost and onSent when Send is clicked with text', async () => {
    // Mock API response
    vi.mocked(api.apiPost).mockResolvedValue({ ok: true, data: {...}, meta: {...} })

    // Render component
    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} />)

    // Simulate user interaction
    const textarea = container.querySelector('textarea')
    fireEvent.change(textarea, { target: { value: 'Hi' } })
    fireEvent.click(sendBtn)

    // Assert API was called
    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(...)
    })
  })
  ```

**Integration Tests:**
- Not explicitly separated; repo runs all tests together via `make test-api` and `make test-web`
- API integration via database: tests that exercise full route → service → storage → database flow
- Web integration: component tests that render with real (not mocked) child components

**E2E Tests:**
- Not present. Smoke tests available via `make smoke` which runs CI-level daemon/API/CLI checks.

## Common Patterns

**Async Testing:**

**TypeScript:**
```typescript
// Using async/await in test function
it('calls apiPost and onSent when Send is clicked with text', async () => {
  vi.mocked(api.apiPost).mockResolvedValue({
    ok: true,
    data: { user_message: mockUserMessage, assistant_message: null },
    meta: { request_id: 'req_1' },
  })

  // ... render component, trigger action ...

  // Wait for async operation
  await waitFor(() => {
    expect(api.apiPost).toHaveBeenCalledWith(...)
  })
  await waitFor(() => {
    expect(onSent).toHaveBeenCalledWith(...)
  })
})
```

**Rust:**
- No async test pattern used (tests are synchronous)
- If needed: use `#[tokio::test]` macro (not currently present in codebase)

**Error Testing:**

**TypeScript:**
```typescript
// Test that error is surfaced
it('surfaces decoder failures for malformed POST responses', async () => {
  vi.spyOn(globalThis, 'fetch').mockResolvedValue({
    ok: true,
    json: async () => ({
      ok: true,
      data: { title: 'missing id' },  // missing 'id' field
      meta: { request_id: 'req_2' },
    }),
  } as Response)

  await expect(
    apiPost<ApiResponse<ConversationData>>(
      '/api/conversations',
      { title: 'New conversation', kind: 'general' },
      (value) => decodeApiResponse(value, decodeConversationData),
    ),
  ).rejects.toThrow(/conversation.id/)
})

// Test error display in component
it('shows error when apiPost rejects', async () => {
  vi.mocked(api.apiPost).mockRejectedValue(new Error('Network error'))

  const { container } = render(
    <MessageComposer
      conversationId="conv_1"
      onOptimisticSend={onOptimisticSend}
      onSendFailed={onSendFailed}
    />
  )
  // ... trigger send ...

  await waitFor(() => {
    expect(within(composer).getByRole('alert')).toHaveTextContent(/network error/i)
  })
})
```

**Rust:**
```rust
#[test]
fn rejects_unknown_family() {
    let input = vec!["hello".to_string(), "world".to_string()];
    assert!(parse(&input).is_err());
}
```

**Type Decoder Testing:**

**TypeScript:**
Extensive decoder tests in `clients/web/src/types.test.ts`. Pattern:
```typescript
it('decodes create-message API responses with optional assistant data', () => {
  const response = decodeApiResponse(
    {
      ok: true,
      data: {
        user_message: {...},
        assistant_message: {...},
        assistant_error: null,
      },
      meta: { request_id: 'req_1' },
    },
    decodeCreateMessageResponse,
  )

  expect(response.data?.user_message.id).toBe('msg_user')
  expect(response.data?.assistant_message?.id).toBe('msg_assistant')
})

// Test that malformed data is rejected
it('requires RFC3339 commitment datetime fields', () => {
  expect(() =>
    decodeCommitmentData({
      id: 'commit_1',
      ...
      due_at: [2026, 75, 9, 30, 0, 0],  // Not RFC3339
      ...
    }),
  ).toThrow(/commitment\.due_at/)
})
```

---

*Testing analysis: 2026-03-17*
