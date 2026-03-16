#!/usr/bin/env bash
# Seed dev data for chat UI. Run with veld already running. Ticket 033.
set -e
BASE="${VEL_API_URL:-http://localhost:4130}"

echo "Seeding at $BASE"

# Create conversation
CONV=$(curl -s -X POST "$BASE/api/conversations" \
  -H "Content-Type: application/json" \
  -d '{"title":"Seed conversation","kind":"general"}')
CONV_ID=$(echo "$CONV" | jq -r '.data.id')
if [ "$CONV_ID" = "null" ] || [ -z "$CONV_ID" ]; then
  echo "Failed to create conversation: $CONV"
  exit 1
fi
echo "Created conversation: $CONV_ID"

# Text message
curl -s -X POST "$BASE/api/conversations/$CONV_ID/messages" \
  -H "Content-Type: application/json" \
  -d '{"role":"user","kind":"text","content":{"text":"Hello, seed message."}}' > /dev/null

# Reminder card
curl -s -X POST "$BASE/api/conversations/$CONV_ID/messages" \
  -H "Content-Type: application/json" \
  -d '{"role":"assistant","kind":"reminder_card","content":{"title":"Take meds","due_time":'$(($(date +%s) + 3600))',"reason":"Daily routine","confidence":0.9}}' > /dev/null

# Risk card
curl -s -X POST "$BASE/api/conversations/$CONV_ID/messages" \
  -H "Content-Type: application/json" \
  -d '{"role":"assistant","kind":"risk_card","content":{"commitment_title":"Ship feature X","risk_level":"high","top_drivers":["blocked on API","scope creep"],"proposed_next_step":"Unblock API first"}}' > /dev/null

# Suggestion card
curl -s -X POST "$BASE/api/conversations/$CONV_ID/messages" \
  -H "Content-Type: application/json" \
  -d '{"role":"assistant","kind":"suggestion_card","content":{"suggestion_text":"Add 15 min buffer before meetings","linked_goal":"Reduce context switches","expected_benefit":"Fewer overruns"}}' > /dev/null

# Summary card
curl -s -X POST "$BASE/api/conversations/$CONV_ID/messages" \
  -H "Content-Type: application/json" \
  -d '{"role":"assistant","kind":"summary_card","content":{"title":"Week in review","timeframe":"Mar 10–16","top_items":["Shipped auth","3 commitments done"],"recommended_actions":["Plan next sprint","Update docs"]}}' > /dev/null

# System notice
curl -s -X POST "$BASE/api/conversations/$CONV_ID/messages" \
  -H "Content-Type: application/json" \
  -d '{"role":"system","kind":"system_notice","content":{"text":"Seed data loaded. You can now try the UI."}}' > /dev/null

echo "Seed done. Open the app and select conversation: $CONV_ID"
