import { useMemo, useState } from 'react';
import { Button } from '../../core/Button';
import { PanelEmptyRow } from '../../core/PanelChrome';
import {
  buildEmbeddedPairingTokenPreview,
  buildSettingsOnboardingGuide,
} from '../../data/operator';
import type {
  ClusterBootstrapData,
  ClusterWorkersData,
  IntegrationsData,
  LinkScopeData,
  LinkedNodeData,
  PairingTokenData,
  SettingsData,
  WorkerPresenceData,
} from '../../types';
import {
  SystemDocumentField,
  SystemDocumentItem,
  SystemDocumentList,
  SystemDocumentMetaRow,
  SystemDocumentSectionLabel,
  SystemDocumentStatsGrid,
  SystemDocumentStatusChip,
} from '../../core/SystemDocument';
import { systemChildAnchor } from './systemNavigation';

type ScopeKey = keyof LinkScopeData;

const DEFAULT_SCOPE_DRAFT: LinkScopeData = {
  read_context: true,
  write_safe_actions: false,
  execute_repo_tasks: false,
};

function formatTimestamp(value: string | null | undefined): string {
  if (!value) {
    return 'Unavailable';
  }
  const parsed = Date.parse(value);
  if (Number.isNaN(parsed)) {
    return value;
  }
  try {
    return new Intl.DateTimeFormat('en-US', {
      month: 'short',
      day: 'numeric',
      hour: 'numeric',
      minute: '2-digit',
    }).format(new Date(parsed));
  } catch {
    return value;
  }
}

function linkedNodeTone(status: LinkedNodeData['status']) {
  switch (status) {
    case 'linked':
      return 'done' as const;
    case 'pending':
      return 'warning' as const;
    case 'revoked':
    case 'expired':
      return 'offline' as const;
    default:
      return 'neutral' as const;
  }
}

function scopeSummary(scopes: LinkScopeData): string {
  const labels = [];
  if (scopes.read_context) {
    labels.push('read context');
  }
  if (scopes.write_safe_actions) {
    labels.push('write safe actions');
  }
  if (scopes.execute_repo_tasks) {
    labels.push('execute repo tasks');
  }
  return labels.length > 0 ? labels.join(', ') : 'No scopes selected';
}

function routeSummary(node: {
  sync_base_url?: string | null;
  tailscale_base_url?: string | null;
  lan_base_url?: string | null;
  localhost_base_url?: string | null;
  public_base_url?: string | null;
}) {
  const routes = [
    node.sync_base_url ? `sync: ${node.sync_base_url}` : null,
    node.tailscale_base_url ? `tailscale: ${node.tailscale_base_url}` : null,
    node.lan_base_url ? `lan: ${node.lan_base_url}` : null,
    node.localhost_base_url ? `localhost: ${node.localhost_base_url}` : null,
    node.public_base_url ? `public: ${node.public_base_url}` : null,
  ].filter((value): value is string => Boolean(value));

  return routes.length > 0 ? routes.join(' | ') : 'No route advertised';
}

function bootstrapArtifactRouteSummary(artifact: {
  endpoints: Array<{ kind: string; base_url: string }>;
} | null | undefined): string | null {
  const routes = artifact?.endpoints
    ?.map((endpoint) => `${endpoint.kind}: ${endpoint.base_url}`)
    .filter((value): value is string => Boolean(value));
  return routes && routes.length > 0 ? routes.join(' | ') : null;
}

function preferredTransport(
  worker: WorkerPresenceData | null | undefined,
  bootstrap: ClusterBootstrapData | null | undefined,
): string | null {
  if (worker?.tailscale_preferred && worker.tailscale_base_url) {
    return 'tailscale';
  }
  return bootstrap?.sync_transport ?? worker?.sync_transport ?? null;
}

function localNodeDisplayName(
  settings: SettingsData | null,
  bootstrap: ClusterBootstrapData | null,
): string {
  return settings?.node_display_name?.trim() || bootstrap?.node_display_name || 'Unnamed node';
}

function ScopeToggle({
  label,
  detail,
  checked,
  onChange,
}: {
  label: string;
  detail: string;
  checked: boolean;
  onChange: () => void;
}) {
  return (
    <label className="flex items-start gap-3 rounded-[16px] border border-[var(--vel-color-border)] bg-[rgba(255,255,255,0.02)] px-3 py-3">
      <input
        type="checkbox"
        checked={checked}
        onChange={onChange}
        className="mt-0.5 h-4 w-4 accent-[var(--vel-color-accent-strong)]"
      />
      <span className="min-w-0">
        <span className="block text-[13px] font-medium leading-5 text-[var(--vel-color-text)]">
          {label}
        </span>
        <span className="block text-[12px] leading-5 text-[var(--vel-color-muted)]">
          {detail}
        </span>
      </span>
    </label>
  );
}

export function NodePairingDetail({
  loading,
  error,
  clusterBootstrap,
  clusterWorkers,
  linkedNodes,
  settings,
  integrations,
  onIssuePairingToken,
  onRedeemPairingToken,
  onRevokeLinkedNode,
}: {
  loading: boolean;
  error: string | null;
  clusterBootstrap: ClusterBootstrapData | null;
  clusterWorkers: ClusterWorkersData | null;
  linkedNodes: LinkedNodeData[];
  settings: SettingsData | null;
  integrations: IntegrationsData;
  onIssuePairingToken: (payload: {
    issued_by_node_id: string;
    ttl_seconds?: number;
    scopes: LinkScopeData;
    target_node_id?: string;
    target_node_display_name?: string | null;
    target_base_url?: string | null;
  }) => Promise<PairingTokenData>;
  onRedeemPairingToken: (payload: {
    token_code: string;
    node_id: string;
    node_display_name: string;
    transport_hint?: string | null;
    requested_scopes?: LinkScopeData | null;
    sync_base_url?: string | null;
    tailscale_base_url?: string | null;
    lan_base_url?: string | null;
    localhost_base_url?: string | null;
    public_base_url?: string | null;
  }) => Promise<LinkedNodeData>;
  onRevokeLinkedNode: (nodeId: string) => Promise<LinkedNodeData>;
}) {
  const [scopeDraft, setScopeDraft] = useState<LinkScopeData>(DEFAULT_SCOPE_DRAFT);
  const [ttlMinutes, setTtlMinutes] = useState('15');
  const [tokenDraft, setTokenDraft] = useState('');
  const [latestToken, setLatestToken] = useState<PairingTokenData | null>(null);
  const [feedback, setFeedback] = useState<string | null>(null);
  const [pendingAction, setPendingAction] = useState<string | null>(null);

  const localWorker = useMemo(
    () =>
      clusterBootstrap
        ? clusterWorkers?.workers.find((worker) => worker.node_id === clusterBootstrap.node_id) ?? null
        : null,
    [clusterBootstrap, clusterWorkers],
  );
  const activeLinkedNodes = useMemo(
    () => linkedNodes.filter((node) => node.status === 'linked'),
    [linkedNodes],
  );
  const discoveredCompanions = useMemo(
    () =>
      clusterBootstrap
        ? (clusterWorkers?.workers ?? []).filter(
          (worker) =>
            worker.node_id !== clusterBootstrap.node_id
            && !activeLinkedNodes.some((node) => node.node_id === worker.node_id),
        )
        : [],
    [activeLinkedNodes, clusterBootstrap, clusterWorkers],
  );
  const onboardingGuide = useMemo(
    () =>
      buildSettingsOnboardingGuide({
        clusterBootstrap,
        clusterWorkers,
        linkedNodes,
        integrations,
      }),
    [clusterBootstrap, clusterWorkers, integrations, linkedNodes],
  );
  const scopesValid = Object.values(scopeDraft).some(Boolean);
  const parsedTtlMinutes = Number.parseInt(ttlMinutes, 10);
  const resolvedTtlMinutes = Number.isFinite(parsedTtlMinutes) && parsedTtlMinutes > 0
    ? parsedTtlMinutes
    : 15;
  const incomingPrompt = localWorker?.incoming_linking_prompt ?? null;

  function toggleScope(scopeKey: ScopeKey) {
    setScopeDraft((current) => ({
      ...current,
      [scopeKey]: !current[scopeKey],
    }));
  }

  async function issueTokenForWorker(worker: WorkerPresenceData | null) {
    if (!clusterBootstrap) {
      setFeedback('Cluster bootstrap is unavailable, so this node cannot issue pairing tokens yet.');
      return;
    }
    if (!scopesValid) {
      setFeedback('Select at least one trust scope before issuing a token.');
      return;
    }

    setPendingAction(worker ? `issue:${worker.node_id}` : 'issue:generic');
    setFeedback(null);
    try {
      const token = await onIssuePairingToken({
        issued_by_node_id: clusterBootstrap.node_id,
        ttl_seconds: resolvedTtlMinutes * 60,
        scopes: scopeDraft,
        target_node_id: worker?.node_id,
        target_node_display_name: worker?.node_display_name ?? null,
        target_base_url: worker?.sync_base_url ?? null,
      });
      setLatestToken(token);
      setFeedback(
        worker
          ? `Issued ${token.token_code} for ${worker.node_display_name}. Share the code on that device before ${formatTimestamp(token.expires_at)}.`
          : `Issued ${token.token_code}. Use one of the suggested routes below to redeem it before ${formatTimestamp(token.expires_at)}.`,
      );
    } catch (issueError) {
      setFeedback(issueError instanceof Error ? issueError.message : String(issueError));
    } finally {
      setPendingAction(null);
    }
  }

  async function redeemToken() {
    if (!clusterBootstrap) {
      setFeedback('Cluster bootstrap is unavailable, so this node cannot redeem pairing tokens yet.');
      return;
    }
    const normalizedToken = buildEmbeddedPairingTokenPreview(tokenDraft);
    if (!normalizedToken) {
      setFeedback('Enter the pairing token shown on the issuing node.');
      return;
    }

    setPendingAction('redeem');
    setFeedback(null);
    try {
      const linkedNode = await onRedeemPairingToken({
        token_code: normalizedToken,
        node_id: clusterBootstrap.node_id,
        node_display_name: localNodeDisplayName(settings, clusterBootstrap),
        transport_hint: preferredTransport(localWorker, clusterBootstrap),
        sync_base_url: clusterBootstrap.sync_base_url,
        tailscale_base_url: clusterBootstrap.tailscale_base_url,
        lan_base_url: clusterBootstrap.lan_base_url,
        localhost_base_url: clusterBootstrap.localhost_base_url,
      });
      setTokenDraft('');
      setFeedback(`Linked ${linkedNode.node_display_name} with ${scopeSummary(linkedNode.scopes)}.`);
    } catch (redeemError) {
      setFeedback(redeemError instanceof Error ? redeemError.message : String(redeemError));
    } finally {
      setPendingAction(null);
    }
  }

  async function revokeNode(nodeId: string, nodeDisplayName: string) {
    setPendingAction(`revoke:${nodeId}`);
    setFeedback(null);
    try {
      await onRevokeLinkedNode(nodeId);
      setFeedback(`Revoked trust for ${nodeDisplayName}.`);
    } catch (revokeError) {
      setFeedback(revokeError instanceof Error ? revokeError.message : String(revokeError));
    } finally {
      setPendingAction(null);
    }
  }

  if (loading) {
    return <PanelEmptyRow>Loading node pairing state…</PanelEmptyRow>;
  }

  if (error) {
    return <PanelEmptyRow>{error}</PanelEmptyRow>;
  }

  return (
    <div className="space-y-6">
      <div className="space-y-3">
        <SystemDocumentSectionLabel>Pairing guide</SystemDocumentSectionLabel>
        <p
          id={systemChildAnchor('pairing', 'guide')}
          className="scroll-mt-24 text-[13px] leading-6 text-[var(--vel-color-text)]"
        >
          {onboardingGuide.nextAction}
        </p>
        <SystemDocumentList>
          {onboardingGuide.steps.map((step) => (
            <SystemDocumentItem
              key={step.id}
              title={step.title}
              subtitle={step.detail}
              trailing={(
                <SystemDocumentStatusChip tone={step.status === 'done' ? 'done' : step.status === 'ready' ? 'active' : 'warning'}>
                  {step.status}
                </SystemDocumentStatusChip>
              )}
            />
          ))}
        </SystemDocumentList>
      </div>

      <div className="space-y-3">
        <SystemDocumentSectionLabel>Trust scope</SystemDocumentSectionLabel>
        <div
          id={systemChildAnchor('pairing', 'scope')}
          className="grid gap-3 scroll-mt-24 md:grid-cols-3"
        >
          <ScopeToggle
            label="Read context"
            detail="Lets the linked node read canonical context, schedule state, and support records."
            checked={scopeDraft.read_context}
            onChange={() => toggleScope('read_context')}
          />
          <ScopeToggle
            label="Write safe actions"
            detail="Allows safe writeback lanes such as acknowledged operator actions, not broad mutation."
            checked={scopeDraft.write_safe_actions}
            onChange={() => toggleScope('write_safe_actions')}
          />
          <ScopeToggle
            label="Execute repo tasks"
            detail="Allows supervised repo-task execution from the linked node when the boundary permits it."
            checked={scopeDraft.execute_repo_tasks}
            onChange={() => toggleScope('execute_repo_tasks')}
          />
        </div>
        <SystemDocumentStatsGrid className="gap-x-6">
          <SystemDocumentMetaRow label="Default TTL" value={`${resolvedTtlMinutes} minutes`} />
          <SystemDocumentMetaRow label="Selected scope" value={scopeSummary(scopeDraft)} />
        </SystemDocumentStatsGrid>
        <label className="block max-w-[12rem]">
          <span className="text-[11px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)]">
            Token lifetime
          </span>
          <select
            aria-label="Token lifetime"
            value={ttlMinutes}
            onChange={(event) => setTtlMinutes(event.target.value)}
            className="mt-1 w-full rounded-[12px] border border-[var(--vel-color-border)] bg-[rgba(255,255,255,0.02)] px-3 py-2 text-sm text-[var(--vel-color-text)] outline-none"
          >
            <option value="15">15 minutes</option>
            <option value="30">30 minutes</option>
            <option value="60">60 minutes</option>
          </select>
        </label>
      </div>

      <div className="space-y-3">
        <SystemDocumentSectionLabel>Issue token</SystemDocumentSectionLabel>
        <div id={systemChildAnchor('pairing', 'issue')} className="scroll-mt-24 space-y-3">
          {clusterBootstrap ? (
            <SystemDocumentItem
              title="This node"
              subtitle={localNodeDisplayName(settings, clusterBootstrap)}
              trailing={<SystemDocumentStatusChip tone="neutral">{clusterBootstrap.node_id}</SystemDocumentStatusChip>}
            >
              <SystemDocumentStatsGrid className="gap-x-6">
                <SystemDocumentMetaRow label="Recommended route" value={`${clusterBootstrap.sync_transport}: ${clusterBootstrap.sync_base_url}`} />
                <SystemDocumentMetaRow label="All routes" value={routeSummary(clusterBootstrap)} />
              </SystemDocumentStatsGrid>
            </SystemDocumentItem>
          ) : null}

          <div className="flex flex-wrap gap-2">
            <Button
              variant="secondary"
              loading={pendingAction === 'issue:generic'}
              onClick={() => void issueTokenForWorker(null)}
            >
              Issue generic token
            </Button>
          </div>

          {discoveredCompanions.length === 0 ? (
            <PanelEmptyRow>
              No unlinked companion node is visible yet. Open Vel on the other device or use the generic token path.
            </PanelEmptyRow>
          ) : (
            <SystemDocumentList>
              {discoveredCompanions.map((worker) => (
                <SystemDocumentItem
                  key={worker.worker_id}
                  title={worker.node_display_name}
                  subtitle={worker.client_kind ?? worker.node_id}
                  trailing={(
                    <Button
                      variant="secondary"
                      loading={pendingAction === `issue:${worker.node_id}`}
                      onClick={() => void issueTokenForWorker(worker)}
                    >
                      Issue token for {worker.node_display_name}
                    </Button>
                  )}
                >
                  <SystemDocumentStatsGrid className="gap-x-6">
                    <SystemDocumentMetaRow label="Route" value={routeSummary(worker)} />
                    <SystemDocumentMetaRow label="Reachability" value={worker.reachability} />
                  </SystemDocumentStatsGrid>
                </SystemDocumentItem>
              ))}
            </SystemDocumentList>
          )}

          {latestToken ? (
            <SystemDocumentItem
              title="Latest token"
              subtitle="Pairing tokens are short-lived and should be redeemed before the expiry window closes."
              trailing={<SystemDocumentStatusChip tone="active">{latestToken.token_code}</SystemDocumentStatusChip>}
            >
              <SystemDocumentStatsGrid className="gap-x-6">
                <SystemDocumentMetaRow label="Issued" value={formatTimestamp(latestToken.issued_at)} />
                <SystemDocumentMetaRow label="Expires" value={formatTimestamp(latestToken.expires_at)} />
                <SystemDocumentMetaRow label="Scope" value={scopeSummary(latestToken.scopes)} />
                <SystemDocumentMetaRow label="Issuer" value={latestToken.issued_by_node_id} />
              </SystemDocumentStatsGrid>
              {latestToken.suggested_targets.length > 0 ? (
                <div className="space-y-2 pt-2">
                  <SystemDocumentSectionLabel>Suggested redeem routes</SystemDocumentSectionLabel>
                  <SystemDocumentList>
                    {latestToken.suggested_targets.map((target) => (
                      <SystemDocumentItem
                        key={`${target.transport_hint}:${target.base_url}`}
                        title={target.label}
                        subtitle={target.redeem_command_hint}
                        trailing={(
                          <SystemDocumentStatusChip tone={target.recommended ? 'done' : 'neutral'}>
                            {target.transport_hint}
                          </SystemDocumentStatusChip>
                        )}
                      >
                        <SystemDocumentStatsGrid className="gap-x-6">
                          <SystemDocumentMetaRow label="Base URL" value={target.base_url} />
                          <SystemDocumentMetaRow label="CLI fallback" value={target.redeem_command_hint} />
                        </SystemDocumentStatsGrid>
                      </SystemDocumentItem>
                    ))}
                  </SystemDocumentList>
                </div>
              ) : null}
            </SystemDocumentItem>
          ) : null}
        </div>
      </div>

      <div className="space-y-3">
        <SystemDocumentSectionLabel>Redeem token</SystemDocumentSectionLabel>
        <div id={systemChildAnchor('pairing', 'redeem')} className="scroll-mt-24 space-y-3">
          {incomingPrompt ? (
            <SystemDocumentItem
              title={`Incoming prompt from ${incomingPrompt.issued_by_node_display_name ?? incomingPrompt.issued_by_node_id}`}
              subtitle="This node already has a prompt waiting. Enter the token from the issuing node to finish linking."
              trailing={<SystemDocumentStatusChip tone="warning">{incomingPrompt.issuer_sync_transport}</SystemDocumentStatusChip>}
            >
              <SystemDocumentStatsGrid className="gap-x-6">
                <SystemDocumentMetaRow label="Prompt scope" value={scopeSummary(incomingPrompt.scopes)} />
                <SystemDocumentMetaRow
                  label="Issuer route"
                  value={
                    bootstrapArtifactRouteSummary(incomingPrompt.bootstrap_artifact)
                    ?? routeSummary({
                      sync_base_url: incomingPrompt.issuer_sync_base_url,
                      tailscale_base_url: incomingPrompt.issuer_tailscale_base_url,
                      lan_base_url: incomingPrompt.issuer_lan_base_url,
                      localhost_base_url: incomingPrompt.issuer_localhost_base_url,
                      public_base_url: incomingPrompt.issuer_public_base_url,
                    })
                  }
                />
                <SystemDocumentMetaRow label="Prompt expires" value={formatTimestamp(incomingPrompt.expires_at)} />
              </SystemDocumentStatsGrid>
            </SystemDocumentItem>
          ) : null}

          <SystemDocumentField
            label="Pairing token"
            value={tokenDraft}
            placeholder="VEL-PAIR-123"
            onChange={setTokenDraft}
          />
          <div className="flex flex-wrap gap-2">
            <Button
              loading={pendingAction === 'redeem'}
              onClick={() => void redeemToken()}
            >
              Redeem token
            </Button>
          </div>
        </div>
      </div>

      <div className="space-y-3">
        <SystemDocumentSectionLabel>Linked nodes</SystemDocumentSectionLabel>
        <div id={systemChildAnchor('pairing', 'linked')} className="scroll-mt-24">
          {linkedNodes.length === 0 ? (
            <PanelEmptyRow>No linked node is recorded yet.</PanelEmptyRow>
          ) : (
            <SystemDocumentList>
              {linkedNodes.map((node) => (
                <SystemDocumentItem
                  key={node.node_id}
                  title={node.node_display_name}
                  subtitle={routeSummary(node)}
                  trailing={(
                    <div className="flex items-center gap-2">
                      <SystemDocumentStatusChip tone={linkedNodeTone(node.status)}>
                        {node.status}
                      </SystemDocumentStatusChip>
                      {node.status === 'linked' ? (
                        <Button
                          variant="ghost"
                          loading={pendingAction === `revoke:${node.node_id}`}
                          onClick={() => void revokeNode(node.node_id, node.node_display_name)}
                        >
                          Revoke {node.node_display_name}
                        </Button>
                      ) : null}
                    </div>
                  )}
                >
                  <SystemDocumentStatsGrid className="gap-x-6">
                    <SystemDocumentMetaRow label="Scope" value={scopeSummary(node.scopes)} />
                    <SystemDocumentMetaRow label="Linked at" value={formatTimestamp(node.linked_at)} />
                    <SystemDocumentMetaRow label="Last seen" value={formatTimestamp(node.last_seen_at)} />
                    <SystemDocumentMetaRow label="Transport" value={node.transport_hint ?? 'Unavailable'} />
                  </SystemDocumentStatsGrid>
                </SystemDocumentItem>
              ))}
            </SystemDocumentList>
          )}
        </div>
      </div>

      {feedback ? (
        <p className="rounded-[16px] border border-[var(--vel-color-border)] bg-[rgba(255,255,255,0.02)] px-4 py-3 text-[13px] leading-5 text-[var(--vel-color-text)]">
          {feedback}
        </p>
      ) : null}
    </div>
  );
}
