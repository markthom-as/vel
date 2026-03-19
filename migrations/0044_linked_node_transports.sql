ALTER TABLE linked_nodes
    ADD COLUMN sync_base_url TEXT;

ALTER TABLE linked_nodes
    ADD COLUMN tailscale_base_url TEXT;

ALTER TABLE linked_nodes
    ADD COLUMN lan_base_url TEXT;

ALTER TABLE linked_nodes
    ADD COLUMN localhost_base_url TEXT;

ALTER TABLE linked_nodes
    ADD COLUMN public_base_url TEXT;
