#!/bin/bash

# Track all new tables in Hasura
tables=(
  "calendar_systems"
  "planes"
  "geography_regions"
  "historical_periods"
  "economic_systems"
  "legal_systems"
  "celestial_bodies"
  "races"
  "character_classes"
  "feats"
  "backgrounds"
  "languages"
  "cultures"
  "factions"
  "pantheons"
  "deities"
  "entities"
  "dungeons"
  "buildings"
  "shops"
  "taverns"
  "temples"
  "items"
  "item_effects"
  "sentient_item_properties"
  "entity_relationships"
  "entity_locations"
  "entity_factions"
  "entity_items"
  "location_items"
  "quest_entities"
  "quest_locations"
  "faction_relationships"
  "race_cultures"
)

echo "Tracking tables..."
for table in "${tables[@]}"; do
  echo "Tracking $table..."
  curl -X POST http://localhost:8080/v1/metadata \
    -H "X-Hasura-Admin-Secret: myadminsecretkey" \
    -H "Content-Type: application/json" \
    -d "{\"type\": \"pg_track_table\", \"args\": {\"source\": \"default\", \"table\": {\"name\": \"$table\", \"schema\": \"public\"}}}" \
    2>/dev/null
  echo ""
done

echo "Reloading metadata..."
curl -X POST http://localhost:8080/v1/metadata \
  -H "X-Hasura-Admin-Secret: myadminsecretkey" \
  -H "Content-Type: application/json" \
  -d '{"type": "reload_metadata", "args": {"reload_remote_schemas": true, "reload_sources": true}}' \
  2>/dev/null

echo "Done!"