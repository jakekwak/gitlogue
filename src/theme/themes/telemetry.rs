use super::super::Theme;
use ratatui::style::Color;

/// Telemetry - "Encrypt the signal; the void is listening."
/// Color palette by bjarneo (https://github.com/bjarneo) from Aether (https://aethr.no)
pub fn telemetry() -> Theme {
    Theme {
        background_left: Color::Rgb(15, 27, 29),
        background_right: Color::Rgb(22, 34, 36),

        editor_line_number: Color::Rgb(107, 141, 148),
        editor_line_number_cursor: Color::Rgb(244, 174, 89),
        editor_separator: Color::Rgb(107, 141, 148),
        editor_cursor_char_bg: Color::Rgb(244, 174, 89),
        editor_cursor_char_fg: Color::Rgb(15, 27, 29),
        editor_cursor_line_bg: Color::Rgb(29, 46, 49),

        file_tree_added: Color::Rgb(141, 172, 139),
        file_tree_deleted: Color::Rgb(194, 113, 102),
        file_tree_modified: Color::Rgb(244, 174, 89),
        file_tree_renamed: Color::Rgb(147, 191, 194),
        file_tree_directory: Color::Rgb(194, 113, 102),
        file_tree_current_file_bg: Color::Rgb(29, 46, 49),
        file_tree_current_file_fg: Color::Rgb(154, 191, 190),
        file_tree_default: Color::Rgb(154, 191, 190),
        file_tree_stats_added: Color::Rgb(141, 172, 139),
        file_tree_stats_deleted: Color::Rgb(194, 113, 102),

        terminal_command: Color::Rgb(154, 191, 190),
        terminal_output: Color::Rgb(107, 141, 148),
        terminal_cursor_bg: Color::Rgb(244, 174, 89),
        terminal_cursor_fg: Color::Rgb(15, 27, 29),

        status_hash: Color::Rgb(244, 174, 89),
        status_author: Color::Rgb(141, 172, 139),
        status_date: Color::Rgb(147, 191, 194),
        status_message: Color::Rgb(154, 191, 190),
        status_no_commit: Color::Rgb(107, 141, 148),

        separator: Color::Rgb(107, 141, 148),

        syntax_keyword: Color::Rgb(194, 113, 102),
        syntax_type: Color::Rgb(212, 154, 79),
        syntax_function: Color::Rgb(212, 154, 79),
        syntax_variable: Color::Rgb(154, 191, 190),
        syntax_string: Color::Rgb(141, 172, 139),
        syntax_number: Color::Rgb(196, 132, 122),
        syntax_comment: Color::Rgb(107, 141, 148),
        syntax_operator: Color::Rgb(143, 181, 179),
        syntax_punctuation: Color::Rgb(122, 154, 153),
        syntax_constant: Color::Rgb(196, 132, 122),
        syntax_parameter: Color::Rgb(122, 154, 153),
        syntax_property: Color::Rgb(154, 191, 190),
        syntax_label: Color::Rgb(194, 113, 102),
    }
}
