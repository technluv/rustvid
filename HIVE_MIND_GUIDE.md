# 🐝 Claude Flow Hive Mind - Complete Usage Guide

## 🚀 Quick Start

### 1. **Initialize a Hive Mind**
```bash
# Basic initialization
npx claude-flow@alpha hive-mind init

# With specific configuration
npx claude-flow@alpha hive-mind init --queen strategic --workers 5 --objective "Build video editing features"
```

### 2. **Spawn Worker Agents**
```bash
# Spawn individual workers
npx claude-flow@alpha hive-mind spawn --type researcher --name "API Explorer"
npx claude-flow@alpha hive-mind spawn --type coder --name "Feature Builder"
npx claude-flow@alpha hive-mind spawn --type tester --name "Quality Checker"
```

### 3. **Assign Tasks**
```bash
# Assign task to swarm
npx claude-flow@alpha hive-mind task "Implement video timeline component"

# Assign to specific worker
npx claude-flow@alpha hive-mind task "Write unit tests" --worker "Quality Checker"
```

### 4. **Monitor Progress**
```bash
# Check swarm status
npx claude-flow@alpha hive-mind status

# Watch real-time activity
npx claude-flow@alpha hive-mind monitor

# View specific swarm
npx claude-flow@alpha hive-mind status --swarm swarm-id
```

## 🎯 Real-World Example: Building a Video Editor Feature

Let me demonstrate with a practical example:

```bash
# Step 1: Create a hive for video timeline feature
npx claude-flow@alpha hive-mind init \
  --queen technical \
  --workers 6 \
  --objective "Build complete video timeline with drag-drop support"

# Step 2: The hive automatically spawns specialized workers:
# - 👑 Queen: Coordinates overall strategy
# - 🔍 Researcher: Analyzes existing timeline libraries
# - 💻 Coder (x2): Implements frontend and backend
# - 🧪 Tester: Creates test cases
# - 📊 Analyst: Monitors performance
# - 📝 Documenter: Updates documentation

# Step 3: Assign the main task
npx claude-flow@alpha hive-mind task "Build video timeline with:
- Drag and drop clips
- Multiple tracks
- Zoom controls
- Snap to grid
- Real-time preview"

# Step 4: Workers automatically collaborate
# The Queen breaks down the task and assigns to workers
# Workers share findings through collective memory
# Progress is tracked automatically
```

## 🔧 Advanced Commands

### **Collective Memory**
```bash
# Store knowledge
npx claude-flow@alpha hive-mind memory store "timeline-api" "Use React DnD for drag-drop"

# Retrieve knowledge
npx claude-flow@alpha hive-mind memory get "timeline-api"

# Search memory
npx claude-flow@alpha hive-mind memory search "drag"
```

### **Consensus Decision Making**
```bash
# Create a decision point
npx claude-flow@alpha hive-mind consensus "Should we use Canvas or SVG for timeline?"

# Workers will analyze and vote
# Queen makes final decision based on consensus
```

### **Swarm Management**
```bash
# List all active swarms
npx claude-flow@alpha hive-mind list

# Pause a swarm
npx claude-flow@alpha hive-mind pause --swarm swarm-id

# Resume a swarm
npx claude-flow@alpha hive-mind resume --swarm swarm-id

# Terminate a swarm
npx claude-flow@alpha hive-mind terminate --swarm swarm-id
```

## 🧠 Hive Mind Architecture

```
                    👑 QUEEN
                (Strategic Coordinator)
                       |
        ┌──────────────┼──────────────┐
        |              |              |
    🔍 Researcher  💻 Coders    🧪 Tester
        |              |              |
        └──────────────┼──────────────┘
                       |
                📊 Collective Memory
                  (Shared Knowledge)
```

## 💡 Best Practices

### 1. **Choose the Right Queen Type**
- `strategic`: For high-level planning
- `technical`: For implementation tasks
- `creative`: For design and UX work
- `analytical`: For optimization and debugging

### 2. **Optimal Worker Count**
- Simple tasks: 3-4 workers
- Medium complexity: 5-7 workers
- Large projects: 8-12 workers

### 3. **Task Breakdown**
```bash
# Good: Specific and measurable
npx claude-flow@alpha hive-mind task "Implement video trimming with frame-accurate cuts"

# Bad: Too vague
npx claude-flow@alpha hive-mind task "Make editor better"
```

## 🎬 Rust Video Editor Specific Examples

### **Example 1: Add Export Feature**
```bash
# Initialize specialized hive
npx claude-flow@alpha hive-mind init \
  --queen technical \
  --workers 5 \
  --objective "Add multi-format video export"

# Assign comprehensive task
npx claude-flow@alpha hive-mind task "
1. Research FFmpeg export options
2. Design export UI dialog
3. Implement progress tracking
4. Add format presets (YouTube, Vimeo, etc)
5. Test with various codecs
6. Document the API
"

# Monitor progress
npx claude-flow@alpha hive-mind monitor
```

### **Example 2: Performance Optimization**
```bash
# Create analytical hive
npx claude-flow@alpha hive-mind init \
  --queen analytical \
  --workers 4 \
  --objective "Optimize video playback performance"

# Task for optimization
npx claude-flow@alpha hive-mind task "
- Profile current performance
- Identify bottlenecks
- Implement frame caching
- Add GPU acceleration
- Benchmark improvements
"
```

### **Example 3: Bug Fixing Sprint**
```bash
# Quick bug-fix hive
npx claude-flow@alpha hive-mind init \
  --queen strategic \
  --workers 6 \
  --objective "Fix critical bugs for v1.0 release"

# Batch assign bugs
npx claude-flow@alpha hive-mind task "Fix issues: #142, #156, #167, #189"
```

## 🔄 Workflow Integration

### **With Git**
```bash
# Hive can create branches
npx claude-flow@alpha hive-mind task "Implement feature X" --git-branch feature/timeline

# Auto-commit progress
npx claude-flow@alpha hive-mind config --auto-commit true
```

### **With Issue Trackers**
```bash
# Link to GitHub issues
npx claude-flow@alpha hive-mind task --github-issue 123

# Update issue status automatically
npx claude-flow@alpha hive-mind config --github-token YOUR_TOKEN
```

## 📊 Monitoring & Analytics

### **Real-time Dashboard**
```bash
# Open web dashboard
npx claude-flow@alpha hive-mind dashboard

# Terminal UI
npx claude-flow@alpha hive-mind monitor --ui
```

### **Progress Reports**
```bash
# Generate report
npx claude-flow@alpha hive-mind report

# Export as JSON
npx claude-flow@alpha hive-mind report --format json > report.json

# Detailed analytics
npx claude-flow@alpha hive-mind analytics --swarm swarm-id
```

## 🛠️ Troubleshooting

### **Common Issues**

1. **Workers not responding**
```bash
# Check worker health
npx claude-flow@alpha hive-mind health

# Restart stuck workers
npx claude-flow@alpha hive-mind restart-workers
```

2. **Memory overflow**
```bash
# Clear old memories
npx claude-flow@alpha hive-mind memory prune --days 7

# Compact database
npx claude-flow@alpha hive-mind maintenance
```

3. **Task stuck**
```bash
# Force task reassignment
npx claude-flow@alpha hive-mind reassign --task task-id
```

## 🎯 Quick Commands Cheatsheet

```bash
# Initialize
npx claude-flow@alpha hive-mind init

# Quick task
npx claude-flow@alpha hive-mind task "Your task here"

# Status check
npx claude-flow@alpha hive-mind status

# Monitor
npx claude-flow@alpha hive-mind monitor

# List swarms
npx claude-flow@alpha hive-mind list

# Terminate all
npx claude-flow@alpha hive-mind terminate --all
```

## 🚀 Start Your First Hive Mind

Try this right now:
```bash
# Create a hive to improve the Rust Video Editor
npx claude-flow@alpha hive-mind init \
  --queen strategic \
  --workers 5 \
  --objective "Add professional color grading tools"

# Watch the magic happen!
npx claude-flow@alpha hive-mind monitor
```

The Hive Mind will automatically:
- 🔍 Research color grading algorithms
- 💻 Implement the features
- 🧪 Create comprehensive tests
- 📊 Optimize performance
- 📝 Update documentation

All working in parallel with shared intelligence!

---

**Pro Tip**: The Hive Mind learns from each task. The more you use it, the smarter it becomes at solving similar problems!