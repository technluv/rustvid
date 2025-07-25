#!/bin/bash

echo "🐝 Hive Mind Quick Start Demo"
echo "============================="
echo ""
echo "This script demonstrates how to use Hive Mind for the Rust Video Editor"
echo ""

# Function to pause and explain
explain() {
    echo ""
    echo "💡 $1"
    echo "Press Enter to continue..."
    read
}

# 1. Check current status
echo "📊 Step 1: Checking current Hive Mind status..."
npx claude-flow@alpha hive-mind status

explain "You have 2 active swarms. Let's work with the timeline UI swarm."

# 2. View collective memory
echo "🧠 Step 2: Viewing collective memory..."
npx claude-flow@alpha hive-mind memory list

explain "The collective memory stores shared knowledge between agents."

# 3. Add knowledge to memory
echo "💾 Step 3: Storing new knowledge..."
npx claude-flow@alpha hive-mind memory store "timeline-zoom" "Use exponential scale for smooth zoom: 2^(level/4)"
npx claude-flow@alpha hive-mind memory store "timeline-colors" "Use HSL for better color interpolation in gradients"

explain "Knowledge stored! Workers can now access this information."

# 4. Create a task with Claude coordination
echo "🎯 Step 4: Creating a coordinated task..."
echo ""
echo "We'll generate Claude Code spawn commands for parallel execution:"
echo ""

npx claude-flow@alpha hive-mind spawn "Add advanced timeline features:
- Magnetic snapping to clips
- Multi-track selection
- Ripple edit mode
- Audio waveform display
- Thumbnail generation" --claude

explain "This generates Claude Code commands that you can run to spawn coordinated agents!"

# 5. Check consensus decisions
echo "🤝 Step 5: Viewing consensus decisions..."
npx claude-flow@alpha hive-mind consensus

explain "Consensus helps agents agree on important architectural decisions."

# 6. View metrics
echo "📈 Step 6: Checking performance metrics..."
npx claude-flow@alpha hive-mind metrics

echo ""
echo "✅ Hive Mind Demo Complete!"
echo ""
echo "🚀 Quick Commands to Remember:"
echo "  - npx claude-flow@alpha hive-mind status      # Check status"
echo "  - npx claude-flow@alpha hive-mind spawn       # Create new swarm"
echo "  - npx claude-flow@alpha hive-mind wizard      # Interactive mode"
echo "  - npx claude-flow@alpha hive-mind memory list # View knowledge"
echo ""
echo "💡 Pro Tip: Use --claude flag to generate coordinated Claude Code commands!"
echo "