import { Fragment, useState, type ReactNode } from 'react'

interface MarkdownMessageProps {
  text: string
  muted?: boolean
}

interface CodeFenceBlock {
  type: 'code'
  language: string | null
  code: string
}

interface TextBlock {
  type: 'text'
  text: string
}

type MarkdownBlock = CodeFenceBlock | TextBlock

function splitMarkdownBlocks(text: string): MarkdownBlock[] {
  const blocks: MarkdownBlock[] = []
  const pattern = /```([a-zA-Z0-9_+-]+)?\n([\s\S]*?)```/g
  let lastIndex = 0

  for (const match of text.matchAll(pattern)) {
    const index = match.index ?? 0
    if (index > lastIndex) {
      blocks.push({ type: 'text', text: text.slice(lastIndex, index) })
    }
    blocks.push({
      type: 'code',
      language: match[1] ?? null,
      code: match[2].replace(/\n$/, ''),
    })
    lastIndex = index + match[0].length
  }

  if (lastIndex < text.length) {
    blocks.push({ type: 'text', text: text.slice(lastIndex) })
  }

  return blocks.filter((block) => block.type === 'code' || block.text.trim().length > 0)
}

function renderInlineMarkdown(text: string): ReactNode[] {
  const nodes: ReactNode[] = []
  const pattern = /(`[^`]+`|\[([^\]]+)\]\((https?:\/\/[^\s)]+)\)|\*\*([^*]+)\*\*|\*([^*]+)\*)/g
  let lastIndex = 0

  for (const match of text.matchAll(pattern)) {
    const index = match.index ?? 0
    if (index > lastIndex) {
      nodes.push(text.slice(lastIndex, index))
    }

    const token = match[0]
    const key = `${index}-${token}`
    if (token.startsWith('`')) {
      nodes.push(
        <code
          key={key}
          className="rounded bg-zinc-900/80 px-1.5 py-0.5 font-mono text-[0.95em] text-amber-200"
        >
          {token.slice(1, -1)}
        </code>,
      )
    } else if (token.startsWith('[') && match[2] && match[3]) {
      nodes.push(
        <a
          key={key}
          href={match[3]}
          target="_blank"
          rel="noreferrer"
          className="text-emerald-300 underline decoration-emerald-500/50 underline-offset-2 hover:text-emerald-200"
        >
          {match[2]}
        </a>,
      )
    } else if (token.startsWith('**') && match[4]) {
      nodes.push(<strong key={key} className="font-semibold text-zinc-100">{match[4]}</strong>)
    } else if (token.startsWith('*') && match[5]) {
      nodes.push(<em key={key} className="italic">{match[5]}</em>)
    }

    lastIndex = index + token.length
  }

  if (lastIndex < text.length) {
    nodes.push(text.slice(lastIndex))
  }

  return nodes
}

function renderTextParagraph(text: string, key: string, className: string) {
  const lines = text.split('\n')
  return (
    <p key={key} className={className}>
      {lines.map((line, index) => (
        <Fragment key={`${key}-${index}`}>
          {index > 0 ? <br /> : null}
          {renderInlineMarkdown(line)}
        </Fragment>
      ))}
    </p>
  )
}

function renderTextBlocks(text: string, muted: boolean): ReactNode[] {
  const paragraphClass = muted ? 'text-zinc-400 italic' : 'text-zinc-200'

  return text
    .trim()
    .split(/\n\s*\n/)
    .filter((block) => block.trim().length > 0)
    .map((block, index) => {
      const trimmed = block.trim()
      const lines = trimmed.split('\n')
      const key = `block-${index}`

      if (/^#{1,4}\s/.test(lines[0])) {
        const level = lines[0].match(/^#+/)?.[0].length ?? 1
        const content = lines[0].replace(/^#{1,4}\s+/, '')
        const headingClass =
          level === 1
            ? 'text-lg font-semibold text-zinc-100'
            : level === 2
              ? 'text-base font-semibold text-zinc-100'
              : 'text-sm font-semibold uppercase tracking-[0.12em] text-zinc-300'
        return (
          <h4 key={key} className={headingClass}>
            {renderInlineMarkdown(content)}
          </h4>
        )
      }

      if (lines.every((line) => /^[-*]\s+/.test(line))) {
        return (
          <ul key={key} className="ml-5 list-disc space-y-1 text-zinc-200 marker:text-zinc-500">
            {lines.map((line, itemIndex) => (
              <li key={`${key}-${itemIndex}`}>{renderInlineMarkdown(line.replace(/^[-*]\s+/, ''))}</li>
            ))}
          </ul>
        )
      }

      if (lines.every((line) => /^\d+\.\s+/.test(line))) {
        return (
          <ol key={key} className="ml-5 list-decimal space-y-1 text-zinc-200 marker:text-zinc-500">
            {lines.map((line, itemIndex) => (
              <li key={`${key}-${itemIndex}`}>{renderInlineMarkdown(line.replace(/^\d+\.\s+/, ''))}</li>
            ))}
          </ol>
        )
      }

      if (lines.every((line) => /^>\s?/.test(line))) {
        return (
          <blockquote
            key={key}
            className="border-l-2 border-zinc-600 pl-3 text-sm text-zinc-300"
          >
            {renderTextParagraph(lines.map((line) => line.replace(/^>\s?/, '')).join('\n'), key, '')}
          </blockquote>
        )
      }

      return renderTextParagraph(trimmed, key, paragraphClass)
    })
}

function CodeFence({ code, language }: { code: string; language: string | null }) {
  const [copied, setCopied] = useState(false)

  async function handleCopy() {
    if (typeof navigator === 'undefined' || !navigator.clipboard?.writeText) {
      return
    }
    await navigator.clipboard.writeText(code)
    setCopied(true)
    window.setTimeout(() => setCopied(false), 1200)
  }

  return (
    <div className="overflow-hidden rounded-lg border border-zinc-700 bg-zinc-950">
      <div className="flex items-center justify-between gap-3 border-b border-zinc-800 px-3 py-2 text-[11px] uppercase tracking-[0.16em] text-zinc-400">
        <span>{language ?? 'code'}</span>
        <button
          type="button"
          onClick={() => void handleCopy()}
          className="rounded bg-zinc-800 px-2 py-1 text-[10px] font-semibold text-zinc-200 hover:bg-zinc-700"
        >
          {copied ? 'Copied' : 'Copy code'}
        </button>
      </div>
      <pre className="overflow-x-auto px-3 py-3 text-sm text-zinc-200">
        <code className="font-mono whitespace-pre">{code}</code>
      </pre>
    </div>
  )
}

export function MarkdownMessage({ text, muted = false }: MarkdownMessageProps) {
  const blocks = splitMarkdownBlocks(text)

  return (
    <div className="space-y-3">
      {blocks.map((block, index) => (
        <Fragment key={`${block.type}-${index}`}>
          {block.type === 'code'
            ? <CodeFence code={block.code} language={block.language} />
            : renderTextBlocks(block.text, muted)}
        </Fragment>
      ))}
    </div>
  )
}
