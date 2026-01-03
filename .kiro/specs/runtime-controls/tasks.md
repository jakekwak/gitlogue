# Implementation Plan

- [x] 1. Implement speed adjustment in AnimationEngine





  - Add `adjust_speed` method that modifies `speed_ms` and `base_speed_ms` by delta
  - Clamp speed values between 1ms and 500ms
  - Return the new speed value after adjustment
  - Update all file-specific speed rules proportionally when base speed changes
  - _Requirements: 1.1, 1.2, 1.4, 1.5, 4.1, 4.3_

- [x] 1.1 Write property test for speed adjustment increments


  - **Property 1: Speed adjustment increments**
  - **Validates: Requirements 1.1, 1.2, 1.4, 1.5**

- [x] 1.2 Write property test for proportional speed rule adjustment


  - **Property 3: Proportional speed rule adjustment**
  - **Validates: Requirements 4.1, 4.2, 4.3**

- [x] 2. Add navigation state tracking to UI




  - Add `navigation_enabled` field to UI struct
  - Initialize based on repository availability and playback mode
  - Implement `can_navigate` method to check if navigation is allowed
  - _Requirements: 3.1, 3.2, 3.3_

- [x] 3. Implement commit navigation methods in UI





  - [x] 3.1 Implement `load_next_commit` method


    - Handle different playback orders (Random, Asc, Desc)
    - Handle range mode vs full repository mode
    - Implement wrapping logic for loop mode
    - Handle boundary conditions for non-loop mode
    - Reset animation state when loading new commit
    - _Requirements: 2.1, 2.3, 2.4, 2.6_

  - [x] 3.2 Implement `load_previous_commit` method


    - Handle different playback orders (Random, Asc, Desc)
    - Handle range mode vs full repository mode
    - Implement wrapping logic for loop mode
    - Handle boundary conditions for non-loop mode
    - Reset animation state when loading new commit
    - _Requirements: 2.2, 2.3, 2.5, 2.7_

- [x] 3.3 Write property test for navigation wrapping behavior


  - **Property 4: Navigation wrapping behavior**
  - **Validates: Requirements 2.4, 2.5, 2.6, 2.7**

- [x] 3.4 Write property test for navigation sequence correctness

  - **Property 5: Navigation sequence correctness**
  - **Validates: Requirements 2.1, 2.2**

- [x] 3.5 Write property test for navigation interruption


  - **Property 6: Navigation interruption**
  - **Validates: Requirements 2.3**

- [x] 4. Add keyboard input handlers for runtime controls





  - Add '+' and '=' key handlers to increase speed (decrease delay by 5ms)
  - Add '-' key handler to decrease speed (increase delay by 5ms)
  - Add 'N' and 'n' key handlers to navigate to next commit
  - Add 'P' and 'p' key handlers to navigate to previous commit
  - Check `can_navigate()` before processing navigation keys
  - Update animation engine speed when speed keys are pressed
  - _Requirements: 1.1, 1.2, 2.1, 2.2_

- [x] 4.1 Write property test for speed adjustment application


  - **Property 2: Speed adjustment application**
  - **Validates: Requirements 1.3**

- [x] 4.2 Write property test for navigation disabled in single-commit mode


  - **Property 7: Navigation disabled in single-commit mode**
  - **Validates: Requirements 3.1**

- [x] 4.3 Write property test for navigation respects filtering


  - **Property 8: Navigation respects filtering**
  - **Validates: Requirements 3.2, 3.3**

- [x] 5. Checkpoint - Ensure all tests pass





  - Ensure all tests pass, ask the user if questions arise.
