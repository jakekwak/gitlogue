# Design Document

## Overview

This design document describes the implementation of runtime control features for gitlogue. The features allow users to dynamically adjust animation speed and navigate between commits during playback without restarting the application. These controls enhance the user experience by providing interactive control over the animation playback.

## Architecture

The implementation follows gitlogue's existing architecture with minimal changes:

1. **UI Controller (`ui.rs`)**: Handles keyboard input and coordinates state changes
2. **Animation Engine (`animation.rs`)**: Manages animation timing and speed adjustments
3. **Git Repository (`git.rs`)**: Provides commit navigation methods
4. **Main (`main.rs`)**: Passes initial configuration to UI

The design maintains separation of concerns:
- UI layer handles input events and user interaction
- Animation engine manages timing and playback state
- Git repository provides data access for commit navigation

## Components and Interfaces

### 1. Speed Control Component

**Location**: `animation.rs` - `AnimationEngine`

**New Methods**:
```rust
pub fn adjust_speed(&mut self, delta: i64) -> u64
pub fn get_current_speed(&self) -> u64
```

**Behavior**:
- `adjust_speed`: Modifies `speed_ms` and `base_speed_ms` by the delta value
- Clamps speed between 1ms (minimum) and 500ms (maximum)
- Returns the new speed value after adjustment
- Updates all file-specific speed rules proportionally

### 2. Commit Navigation Component

**Location**: `ui.rs` - `UI` struct

**New Fields**:
```rust
navigation_enabled: bool
```

**New Methods**:
```rust
fn can_navigate(&self) -> bool
fn load_next_commit(&mut self) -> Result<()>
fn load_previous_commit(&mut self) -> Result<()>
```

**Behavior**:
- `can_navigate`: Returns true if repository reference exists and not in single-commit mode
- `load_next_commit`: Loads next commit based on current playback order, handles wrapping
- `load_previous_commit`: Loads previous commit based on current playback order, handles wrapping

### 3. Keyboard Input Handler

**Location**: `ui.rs` - `run_loop` method

**New Key Bindings**:
- `+` or `=`: Increase speed (decrease delay by 5ms)
- `-`: Decrease speed (increase delay by 5ms)
- `N` or `n`: Navigate to next commit
- `P` or `p`: Navigate to previous commit

**Integration Point**: Existing keyboard event polling in `run_loop`

## Data Models

### Speed Adjustment State

```rust
// In AnimationEngine
speed_ms: u64           // Current typing speed
base_speed_ms: u64      // Base speed for proportional adjustments
speed_rules: Vec<SpeedRule>  // File-specific speed rules
```

### Navigation State

```rust
// In UI
navigation_enabled: bool  // Whether navigation is allowed
repo: Option<&'a GitRepository>  // Repository reference for navigation
order: PlaybackOrder     // Current playback order (Random/Asc/Desc)
is_range_mode: bool      // Whether in commit range mode
```

### Commit Index Tracking

The `GitRepository` already maintains:
```rust
commit_index: RefCell<usize>  // Current position in commit sequence
commit_cache: RefCell<Option<Vec<Oid>>>  // Cached commits for asc/desc
commit_range: RefCell<Option<Vec<Oid>>>  // Commits in specified range
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Speed adjustment increments

*For any* current speed value, when a speed increase is requested, the new speed should be exactly 5 milliseconds less than the current speed (unless the current speed is less than or equal to 5ms, in which case it should be clamped to 1ms), and when a speed decrease is requested, the new speed should be exactly 5 milliseconds more than the current speed (unless the current speed is greater than or equal to 495ms, in which case it should be clamped to 500ms).

**Validates: Requirements 1.1, 1.2, 1.4, 1.5**

### Property 2: Speed adjustment application

*For any* speed adjustment operation, all subsequent animation steps should use the new speed value immediately.

**Validates: Requirements 1.3**

### Property 3: Proportional speed rule adjustment

*For any* set of file-specific speed rules and base speed adjustment, the ratio between each file-specific speed and the base speed should remain constant before and after the adjustment (within integer rounding tolerance of ±1ms).

**Validates: Requirements 4.1, 4.2, 4.3**

### Property 4: Navigation wrapping behavior

*For any* commit sequence and current position, when loop mode is enabled, navigating forward from the last commit should load the first commit and navigating backward from the first commit should load the last commit; when loop mode is disabled, navigating forward from the last commit should remain at the last commit and navigating backward from the first commit should remain at the first commit.

**Validates: Requirements 2.4, 2.5, 2.6, 2.7**

### Property 5: Navigation sequence correctness

*For any* commit sequence and current position (not at boundaries), navigating forward should load the commit at position+1 and navigating backward should load the commit at position-1.

**Validates: Requirements 2.1, 2.2**

### Property 6: Navigation interruption

*For any* commit navigation operation during active animation, the animation state should be reset and a new animation should start from the beginning of the target commit.

**Validates: Requirements 2.3**

### Property 7: Navigation disabled in single-commit mode

*For any* single-commit playback configuration without loop mode enabled, navigation operations should not change the currently displayed commit.

**Validates: Requirements 3.1**

### Property 8: Navigation respects filtering

*For any* commit sequence with active filters (range, author, or date), navigation operations should only move between commits that match the filter criteria.

**Validates: Requirements 3.2, 3.3**

## Error Handling

### Speed Adjustment Errors

- **Out of bounds**: Silently clamp to valid range [1, 500]ms
- **No error messages**: Speed adjustments are always valid operations

### Navigation Errors

- **No repository**: Navigation operations are no-ops when `repo` is None
- **End of sequence**: In non-loop mode, stay at current commit
- **Commit load failure**: Log error and remain at current commit
- **Range exhaustion**: In loop mode, wrap to beginning; in non-loop mode, stay at current

### Input Handling Errors

- **Invalid key combinations**: Ignore unrecognized keys
- **Rapid key presses**: Process each event independently

## Testing Strategy

### Unit Testing

Unit tests will verify:
- Speed adjustment clamping at boundaries (1ms and 500ms)
- Speed adjustment by exact increments (±5ms)
- Navigation enabled/disabled logic based on mode
- Commit index wrapping in loop mode
- Commit index bounds in non-loop mode

### Property-Based Testing

Property-based tests will use the `proptest` crate for Rust to verify:
- Speed always remains in valid range after arbitrary adjustment sequences
- File-specific speed rules maintain proportional relationships
- Navigation wrapping behavior in loop mode
- Navigation bounds behavior in non-loop mode
- Animation interruption on navigation

### Integration Testing

Integration tests will verify:
- Keyboard input triggers correct speed adjustments
- Keyboard input triggers correct navigation
- Speed changes apply to subsequent animation steps
- Navigation loads correct commits in different playback orders
- UI state updates correctly after navigation

### Manual Testing

Manual testing will verify:
- Visual feedback of speed changes during animation
- Smooth transitions between commits during navigation
- Keyboard responsiveness during active animation
- Correct behavior at sequence boundaries
