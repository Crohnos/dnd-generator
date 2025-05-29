#!/bin/bash

HASURA_URL="http://localhost:8080/v1/metadata"
ADMIN_SECRET="myadminsecretkey"

echo "🔄 Tracking new tables in Hasura..."

# Array of new tables to track
new_tables=(
    "backstory_element_types"
    "backstory_elements" 
    "calendar_systems"
    "campaign_flags"
    "campaign_sessions"
    "character_backgrounds"
    "character_races"
    "currencies"
    "deities"
    "encounter_types"
    "historical_events"
    "knowledge_sources"
    "location_connections"
    "location_services"
    "location_types"
    "magic_items"
    "magical_phenomena"
    "npc_relationships"
    "organization_memberships"
    "organization_ranks"
    "organization_relationships"
    "organization_types"
    "organizations"
    "quest_progress"
    "quest_types"
    "random_events"
    "religious_organizations"
    "rumors"
    "trade_goods"
    "trade_routes"
)

# Track each table
for table in "${new_tables[@]}"; do
    response=$(curl -s -w "%{http_code}" -o /tmp/hasura_response.json \
        -X POST \
        -H "Content-Type: application/json" \
        -H "X-Hasura-Admin-Secret: $ADMIN_SECRET" \
        -d "{
            \"type\": \"pg_track_table\",
            \"args\": {
                \"source\": \"default\",
                \"table\": {
                    \"schema\": \"public\",
                    \"name\": \"$table\"
                }
            }
        }" \
        "$HASURA_URL")
    
    if [ "$response" = "200" ]; then
        echo "✅ Successfully tracked table: $table"
    else
        echo "❌ Failed to track table $table (HTTP $response)"
        cat /tmp/hasura_response.json
    fi
done

echo ""
echo "🔄 Setting up foreign key relationships..."

# Function to create relationship
create_relationship() {
    local table=$1
    local name=$2
    local foreign_key=$3
    
    response=$(curl -s -w "%{http_code}" -o /tmp/hasura_response.json \
        -X POST \
        -H "Content-Type: application/json" \
        -H "X-Hasura-Admin-Secret: $ADMIN_SECRET" \
        -d "{
            \"type\": \"pg_create_object_relationship\",
            \"args\": {
                \"source\": \"default\",
                \"table\": {
                    \"schema\": \"public\",
                    \"name\": \"$table\"
                },
                \"name\": \"$name\",
                \"using\": {
                    \"foreign_key_constraint_on\": \"$foreign_key\"
                }
            }
        }" \
        "$HASURA_URL")
    
    if [ "$response" = "200" ]; then
        echo "✅ Created relationship: $table.$name"
    else
        echo "❌ Failed to create relationship $table.$name (HTTP $response)"
    fi
}

# Create key relationships
create_relationship "calendar_systems" "campaign" "campaign_id"
create_relationship "historical_events" "campaign" "campaign_id"
create_relationship "historical_events" "calendar_system" "calendar_system_id"

create_relationship "locations" "location_type" "location_type_id"
create_relationship "locations" "parent_location" "parent_location_id"
create_relationship "location_connections" "from_location" "from_location_id"
create_relationship "location_connections" "to_location" "to_location_id"
create_relationship "location_services" "location" "location_id"
create_relationship "location_services" "owner_npc" "owner_npc_id"

create_relationship "npcs" "race" "race_id"
create_relationship "npcs" "background" "background_id"
create_relationship "npcs" "current_location" "current_location_id"
create_relationship "npcs" "home_location" "home_location_id"
create_relationship "npc_relationships" "npc1" "npc1_id"
create_relationship "npc_relationships" "npc2" "npc2_id"

create_relationship "organizations" "campaign" "campaign_id"
create_relationship "organizations" "organization_type" "organization_type_id"
create_relationship "organizations" "headquarters_location" "headquarters_location_id"
create_relationship "organizations" "parent_organization" "parent_organization_id"
create_relationship "organization_ranks" "organization" "organization_id"
create_relationship "organization_memberships" "organization" "organization_id"
create_relationship "organization_memberships" "npc" "npc_id"
create_relationship "organization_memberships" "rank" "rank_id"

create_relationship "backstory_elements" "campaign" "campaign_id"
create_relationship "backstory_elements" "element_type" "element_type_id"
create_relationship "backstory_elements" "connected_npc" "connected_npc_id"
create_relationship "backstory_elements" "connected_location" "connected_location_id"
create_relationship "backstory_elements" "connected_organization" "connected_organization_id"

create_relationship "quest_hooks" "quest_type" "quest_type_id"
create_relationship "quest_hooks" "patron_npc" "patron_npc_id"
create_relationship "quest_hooks" "patron_organization" "patron_organization_id"
create_relationship "quest_hooks" "target_location" "target_location_id"
create_relationship "quest_progress" "quest_hook" "quest_hook_id"

create_relationship "encounters" "encounter_type" "encounter_type_id"

create_relationship "magic_items" "campaign" "campaign_id"
create_relationship "magic_items" "current_location" "current_location_id"
create_relationship "magic_items" "current_owner_npc" "current_owner_npc_id"

create_relationship "knowledge_sources" "campaign" "campaign_id"
create_relationship "knowledge_sources" "author_npc" "author_npc_id"
create_relationship "knowledge_sources" "current_location" "current_location_id"

create_relationship "rumors" "campaign" "campaign_id"
create_relationship "rumors" "origin_location" "origin_location_id"
create_relationship "rumors" "origin_npc" "origin_npc_id"

create_relationship "campaign_sessions" "campaign" "campaign_id"
create_relationship "campaign_flags" "campaign" "campaign_id"

echo ""
echo "✨ Hasura GraphQL schema setup complete!"
echo "🌐 All new tables are now available via GraphQL at http://localhost:8080/v1/graphql"
echo "🔍 You can explore the schema at http://localhost:8080/console"