#!/usr/bin/env bash
# ==============================================================================
# 📐 SOLID Scale & Code Bloat Audit Tool
# Scans Rust source files in src/ for LOC bloat, SRP violations, and scale risks
# ==============================================================================

set -euo pipefail

TARGET_DIR="${1:-src}"
THRESHOLD_RED=450
THRESHOLD_YELLOW=250

echo "======================================================================"
echo "📐 SOLID & SCALABILITY ARCHITECTURE AUDITOR"
echo "Target Directory : ${TARGET_DIR}"
echo "Thresholds       : 🔴 Red > ${THRESHOLD_RED} LOC | 🟡 Yellow > ${THRESHOLD_YELLOW} LOC | 🟢 Green <= ${THRESHOLD_YELLOW} LOC"
echo "======================================================================"

RED_COUNT=0
YELLOW_COUNT=0
GREEN_COUNT=0
TOTAL_FILES=0

printf "\n%-8s %-6s %-8s %-45s %s\n" "STATUS" "LOC" "STRUCTS" "FILE PATH" "SCALE FORECAST & RECOMMENDATION"
echo "----------------------------------------------------------------------------------------------------------------"

# Find all .rs files and sort by line count descending
while IFS= read -r line; do
    LOC=$(echo "$line" | awk '{print $1}')
    FILE=$(echo "$line" | awk '{print $2}')
    
    # Skip total line
    if [[ "$FILE" == "total" || -z "$FILE" ]]; then
        continue
    fi

    TOTAL_FILES=$((TOTAL_FILES + 1))

    # Count responsibilities: structs, enums, traits
    RESPONSIBILITIES=$(grep -cE '^\s*(pub\s+)?(struct|enum|trait)\s+' "$FILE" || true)

    if (( LOC > THRESHOLD_RED )); then
        RED_COUNT=$((RED_COUNT + 1))
        STATUS="🔴 RED"
        FORECAST="High SRP risk. Split into domain sub-handlers."
    elif (( LOC > THRESHOLD_YELLOW )); then
        YELLOW_COUNT=$((YELLOW_COUNT + 1))
        STATUS="🟡 WARN"
        FORECAST="Growth strain. Monitor for multi-responsibility."
    else
        GREEN_COUNT=$((GREEN_COUNT + 1))
        STATUS="🟢 OK"
        FORECAST="Cohesive single responsibility."
    fi

    printf "%-8s %-6d %-8d %-45s %s\n" "$STATUS" "$LOC" "$RESPONSIBILITIES" "$FILE" "$FORECAST"

done < <(find "$TARGET_DIR" -name "*.rs" -exec wc -l {} + | sort -rn)

echo "----------------------------------------------------------------------------------------------------------------"
echo "AUDIT SUMMARY:"
echo "  Total Files Audited : ${TOTAL_FILES}"
echo "  🟢 Green (Healthy)  : ${GREEN_COUNT}"
echo "  🟡 Yellow (Warning) : ${YELLOW_COUNT}"
echo "  🔴 Red (Over-bloat) : ${RED_COUNT}"
echo "======================================================================"

if (( RED_COUNT > 0 )); then
    echo "⚠️  ACTION REQUIRED: ${RED_COUNT} file(s) exceed ${THRESHOLD_RED} LOC, violating Single Responsibility."
    echo "   See .agents/skills/solid-scale-auditor/SKILL.md for decomposition patterns."
    exit 1
else
    echo "✅ ARCHITECTURE CHECK PASSED: All files conform to modular SOLID size standards."
    exit 0
fi
