-- Thread-level browser call mode for shared STT/TTS assistant flow.

ALTER TABLE conversations
ADD COLUMN call_mode_active INTEGER NOT NULL DEFAULT 0;
