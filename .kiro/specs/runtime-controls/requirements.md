# Requirements Document

## Introduction

This document specifies requirements for adding runtime control features to gitlogue, a Git history screensaver application. The features enable users to dynamically adjust animation speed and navigate between commits during playback without restarting the application.

## Glossary

- **gitlogue**: A terminal-based screensaver that replays Git commits with animated typing effects
- **Animation Speed**: The delay in milliseconds between typing each character during commit playback
- **Commit Navigation**: The ability to jump to different commits in the repository history during runtime
- **Runtime Control**: User input handling during active animation playback
- **Animation Engine**: The component responsible for managing animation state and playback timing
- **UI Controller**: The component that handles user input and coordinates between UI components

## Requirements

### Requirement 1

**User Story:** As a user watching gitlogue, I want to adjust the animation speed during playback, so that I can make the typing faster or slower based on my preference without restarting the application.

#### Acceptance Criteria

1. WHEN a user presses the '+' key THEN the system SHALL increase the animation speed by decreasing the delay by 5 milliseconds
2. WHEN a user presses the '-' key THEN the system SHALL decrease the animation speed by increasing the delay by 5 milliseconds
3. WHEN the animation speed is adjusted THEN the system SHALL apply the new speed to subsequent animation steps immediately
4. WHEN the animation speed reaches 1 millisecond THEN the system SHALL prevent further speed increases
5. WHEN the animation speed reaches 500 milliseconds THEN the system SHALL prevent further speed decreases

### Requirement 2

**User Story:** As a user watching gitlogue, I want to navigate between commits during playback, so that I can jump to the next or previous commit without waiting for the current animation to complete.

#### Acceptance Criteria

1. WHEN a user presses the 'N' key THEN the system SHALL load and start animating the next commit in the sequence
2. WHEN a user presses the 'P' key THEN the system SHALL load and start animating the previous commit in the sequence
3. WHEN navigating to a new commit THEN the system SHALL interrupt the current animation and reset the animation state
4. WHEN the user is at the last commit and presses 'N' THEN the system SHALL wrap to the first commit if loop mode is enabled
5. WHEN the user is at the first commit and presses 'P' THEN the system SHALL wrap to the last commit if loop mode is enabled
6. WHEN the user is at the last commit and presses 'N' and loop mode is disabled THEN the system SHALL remain at the current commit
7. WHEN the user is at the first commit and presses 'P' and loop mode is disabled THEN the system SHALL remain at the current commit

### Requirement 3

**User Story:** As a user watching gitlogue in single-commit mode, I want navigation keys to be disabled, so that the application behavior remains consistent with the specified commit.

#### Acceptance Criteria

1. WHEN a single commit is specified via command line and loop mode is disabled THEN the system SHALL ignore 'N' and 'P' key presses
2. WHEN a commit range is specified THEN the system SHALL enable navigation within that range only
3. WHEN author or date filters are active THEN the system SHALL enable navigation within the filtered commit set only

### Requirement 4

**User Story:** As a developer, I want the speed adjustment to respect file-specific speed rules, so that different file types maintain their relative speed differences.

#### Acceptance Criteria

1. WHEN a user adjusts the base animation speed THEN the system SHALL update all file-specific speed rules proportionally
2. WHEN switching between files with different speed rules THEN the system SHALL apply the adjusted speed for each file type
3. WHEN the animation speed is adjusted THEN the system SHALL maintain the ratio between base speed and file-specific speeds
