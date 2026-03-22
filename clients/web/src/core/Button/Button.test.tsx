import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { Button, IconButton } from './Button';

afterEach(() => {
  cleanup();
});

describe('Button', () => {
  it('merges loading with disabled and sets aria-busy', () => {
    render(
      <Button loading aria-label="Save">
        Save
      </Button>,
    );
    const el = screen.getByRole('button', { name: 'Save' });
    expect(el).toBeDisabled();
    expect(el).toHaveAttribute('aria-busy', 'true');
  });

  it('shows spinner before label when loading on a text button', () => {
    render(
      <Button loading>Submit</Button>,
    );
    expect(screen.getByRole('button', { name: 'Submit' })).toBeInTheDocument();
    expect(screen.getByRole('button').querySelector('svg[aria-hidden="true"]')).toBeTruthy();
  });

  it('replaces icon children with spinner when loading on icon size', () => {
    render(
      <Button loading size="icon" aria-label="Sync">
        <span data-testid="icon-glyph">↻</span>
      </Button>,
    );
    expect(screen.queryByTestId('icon-glyph')).toBeNull();
    expect(screen.getByRole('button', { name: 'Sync' }).querySelector('svg')).toBeTruthy();
  });
});

describe('IconButton', () => {
  it('defaults to icon size and requires accessible name', () => {
    render(
      <IconButton variant="ghost" aria-label="Close" onClick={vi.fn()}>
        ×
      </IconButton>,
    );
    const el = screen.getByRole('button', { name: 'Close' });
    expect(el.className).toMatch(/h-9/);
    expect(el.className).toMatch(/w-9/);
  });
});
