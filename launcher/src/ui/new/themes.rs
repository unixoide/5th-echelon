//! Provides themes for the `imgui` UI by defining color schemes.
//!
//! This module contains functions that apply color styles to the `imgui` context,
//! allowing for a consistent look and feel across the application.

const TRANSPARENT: [f32; 4] = [0.00, 0.00, 0.00, 0.00];

// Constants for imgui style colors, to make the code more readable.
#[allow(non_upper_case_globals)]
const ImGuiCol_Text: usize = imgui::StyleColor::Text as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TextDisabled: usize = imgui::StyleColor::TextDisabled as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_WindowBg: usize = imgui::StyleColor::WindowBg as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_ChildBg: usize = imgui::StyleColor::ChildBg as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_PopupBg: usize = imgui::StyleColor::PopupBg as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_Border: usize = imgui::StyleColor::Border as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_BorderShadow: usize = imgui::StyleColor::BorderShadow as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_FrameBg: usize = imgui::StyleColor::FrameBg as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_FrameBgHovered: usize = imgui::StyleColor::FrameBgHovered as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_FrameBgActive: usize = imgui::StyleColor::FrameBgActive as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TitleBg: usize = imgui::StyleColor::TitleBg as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TitleBgActive: usize = imgui::StyleColor::TitleBgActive as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TitleBgCollapsed: usize = imgui::StyleColor::TitleBgCollapsed as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_MenuBarBg: usize = imgui::StyleColor::MenuBarBg as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_ScrollbarBg: usize = imgui::StyleColor::ScrollbarBg as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_ScrollbarGrab: usize = imgui::StyleColor::ScrollbarGrab as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_ScrollbarGrabHovered: usize = imgui::StyleColor::ScrollbarGrabHovered as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_ScrollbarGrabActive: usize = imgui::StyleColor::ScrollbarGrabActive as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_CheckMark: usize = imgui::StyleColor::CheckMark as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_SliderGrab: usize = imgui::StyleColor::SliderGrab as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_SliderGrabActive: usize = imgui::StyleColor::SliderGrabActive as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_Button: usize = imgui::StyleColor::Button as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_ButtonHovered: usize = imgui::StyleColor::ButtonHovered as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_ButtonActive: usize = imgui::StyleColor::ButtonActive as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_Header: usize = imgui::StyleColor::Header as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_HeaderHovered: usize = imgui::StyleColor::HeaderHovered as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_HeaderActive: usize = imgui::StyleColor::HeaderActive as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_Separator: usize = imgui::StyleColor::Separator as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_SeparatorHovered: usize = imgui::StyleColor::SeparatorHovered as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_SeparatorActive: usize = imgui::StyleColor::SeparatorActive as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_ResizeGrip: usize = imgui::StyleColor::ResizeGrip as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_ResizeGripHovered: usize = imgui::StyleColor::ResizeGripHovered as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_ResizeGripActive: usize = imgui::StyleColor::ResizeGripActive as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_Tab: usize = imgui::StyleColor::Tab as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TabHovered: usize = imgui::StyleColor::TabHovered as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TabActive: usize = imgui::StyleColor::TabActive as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TabUnfocused: usize = imgui::StyleColor::TabUnfocused as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TabUnfocusedActive: usize = imgui::StyleColor::TabUnfocusedActive as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_DockingPreview: usize = imgui::StyleColor::DockingPreview as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_DockingEmptyBg: usize = imgui::StyleColor::DockingEmptyBg as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_PlotLines: usize = imgui::StyleColor::PlotLines as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_PlotLinesHovered: usize = imgui::StyleColor::PlotLinesHovered as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_PlotHistogram: usize = imgui::StyleColor::PlotHistogram as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_PlotHistogramHovered: usize = imgui::StyleColor::PlotHistogramHovered as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TableHeaderBg: usize = imgui::StyleColor::TableHeaderBg as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TableBorderStrong: usize = imgui::StyleColor::TableBorderStrong as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TableBorderLight: usize = imgui::StyleColor::TableBorderLight as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TableRowBg: usize = imgui::StyleColor::TableRowBg as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TableRowBgAlt: usize = imgui::StyleColor::TableRowBgAlt as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_TextSelectedBg: usize = imgui::StyleColor::TextSelectedBg as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_DragDropTarget: usize = imgui::StyleColor::DragDropTarget as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_NavHighlight: usize = imgui::StyleColor::NavHighlight as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_NavWindowingHighlight: usize = imgui::StyleColor::NavWindowingHighlight as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_NavWindowingDimBg: usize = imgui::StyleColor::NavWindowingDimBg as usize;
#[allow(non_upper_case_globals)]
const ImGuiCol_ModalWindowDimBg: usize = imgui::StyleColor::ModalWindowDimBg as usize;

/// A macro to convert C++ style color definitions to Rust.
macro_rules! convert_cpp_to_rs {
    ($style:expr => $(colors[$i:expr] = ImVec4($r:expr, $g:expr, $b:expr, $a:expr);)*) => {
        $(
        $style.colors[$i] = [$r, $g, $b, $a];
        )*
    };
}

/// Applies the "old" theme to the UI.
pub fn old(style: &mut imgui::Style) {
    use imgui::StyleColor;

    style.colors[StyleColor::Text as usize] = [1.00, 1.00, 1.00, 1.00];
    style.colors[StyleColor::TextDisabled as usize] = [0.50, 0.50, 0.50, 1.00];
    style.colors[StyleColor::WindowBg as usize] = [0.03, 0.07, 0.04, 0.94];
    style.colors[StyleColor::ChildBg as usize] = TRANSPARENT;
    style.colors[StyleColor::PopupBg as usize] = [0.08, 0.08, 0.08, 0.94];
    style.colors[StyleColor::Border as usize] = [0.38, 1.00, 0.00, 0.50];
    style.colors[StyleColor::BorderShadow as usize] = [0.01, 0.13, 0.00, 0.63];
    style.colors[StyleColor::FrameBg as usize] = [0.17, 0.48, 0.16, 0.54];
    style.colors[StyleColor::FrameBgHovered as usize] = [0.26, 0.98, 0.32, 0.40];
    style.colors[StyleColor::FrameBgActive as usize] = [0.26, 0.98, 0.28, 0.67];
    style.colors[StyleColor::TitleBg as usize] = [0.01, 0.07, 0.01, 1.00];
    style.colors[StyleColor::TitleBgActive as usize] = [0.0, 0.56, 0.29, 1.0];
    style.colors[StyleColor::TitleBgCollapsed as usize] = [0.00, 0.56, 0.09, 0.51];
    style.colors[StyleColor::MenuBarBg as usize] = [0.0, 0.56, 0.29, 1.0];
    style.colors[StyleColor::ScrollbarBg as usize] = [0.00, 0.15, 0.00, 0.53];
    style.colors[StyleColor::ScrollbarGrab as usize] = [0.10, 0.41, 0.06, 1.00];
    style.colors[StyleColor::ScrollbarGrabHovered as usize] = [0.00, 0.66, 0.04, 1.00];
    style.colors[StyleColor::ScrollbarGrabActive as usize] = [0.04, 0.87, 0.00, 1.00];
    style.colors[StyleColor::CheckMark as usize] = [0.26, 0.98, 0.40, 1.00];
    style.colors[StyleColor::SliderGrab as usize] = [0.21, 0.61, 0.00, 1.00];
    style.colors[StyleColor::SliderGrabActive as usize] = [0.36, 0.87, 0.22, 1.00];
    style.colors[StyleColor::Button as usize] = [0.00, 0.60, 0.05, 0.40];
    style.colors[StyleColor::ButtonHovered as usize] = [0.20, 0.78, 0.32, 1.00];
    style.colors[StyleColor::ButtonActive as usize] = [0.00, 0.57, 0.07, 1.00];
    style.colors[StyleColor::Header as usize] = [0.12, 0.82, 0.28, 0.31];
    style.colors[StyleColor::HeaderHovered as usize] = [0.00, 0.74, 0.11, 0.80];
    style.colors[StyleColor::HeaderActive as usize] = [0.09, 0.69, 0.04, 1.00];
    style.colors[StyleColor::Separator as usize] = [0.09, 0.67, 0.01, 0.50];
    style.colors[StyleColor::SeparatorHovered as usize] = [0.32, 0.75, 0.10, 0.78];
    style.colors[StyleColor::SeparatorActive as usize] = [0.10, 0.75, 0.11, 1.00];
    style.colors[StyleColor::ResizeGrip as usize] = [0.32, 0.98, 0.26, 0.20];
    style.colors[StyleColor::ResizeGripHovered as usize] = [0.26, 0.98, 0.28, 0.67];
    style.colors[StyleColor::ResizeGripActive as usize] = [0.22, 0.69, 0.06, 0.95];
    style.colors[StyleColor::Tab as usize] = [0.18, 0.58, 0.18, 0.86];
    style.colors[StyleColor::TabHovered as usize] = [0.26, 0.98, 0.28, 0.80];
    style.colors[StyleColor::TabActive as usize] = [0.20, 0.68, 0.24, 1.00];
    style.colors[StyleColor::TabUnfocused as usize] = [0.07, 0.15, 0.08, 0.97];
    style.colors[StyleColor::TabUnfocusedActive as usize] = [0.14, 0.42, 0.19, 1.00];
    style.colors[StyleColor::PlotLines as usize] = [0.61, 0.61, 0.61, 1.00];
    style.colors[StyleColor::PlotLinesHovered as usize] = [1.00, 0.43, 0.35, 1.00];
    style.colors[StyleColor::PlotHistogram as usize] = [0.90, 0.70, 0.00, 1.00];
    style.colors[StyleColor::PlotHistogramHovered as usize] = [1.00, 0.60, 0.00, 1.00];
    style.colors[StyleColor::TableHeaderBg as usize] = [0.19, 0.19, 0.20, 1.00];
    style.colors[StyleColor::TableBorderStrong as usize] = [0.31, 0.31, 0.35, 1.00];
    style.colors[StyleColor::TableBorderLight as usize] = [0.23, 0.23, 0.25, 1.00];
    style.colors[StyleColor::TableRowBg as usize] = TRANSPARENT;
    style.colors[StyleColor::TableRowBgAlt as usize] = [1.00, 1.00, 1.00, 0.06];
    style.colors[StyleColor::TextSelectedBg as usize] = [0.00, 0.89, 0.20, 0.35];
    style.colors[StyleColor::DragDropTarget as usize] = [1.00, 1.00, 0.00, 0.90];
    style.colors[StyleColor::NavHighlight as usize] = [0.26, 0.98, 0.35, 1.00];
    style.colors[StyleColor::NavWindowingHighlight as usize] = [1.00, 1.00, 1.00, 0.70];
    style.colors[StyleColor::NavWindowingDimBg as usize] = [0.80, 0.80, 0.80, 0.20];
    style.colors[StyleColor::ModalWindowDimBg as usize] = [0.80, 0.80, 0.80, 0.35];
}

/// Applies the "new" theme to the UI.
pub fn new(style: &mut imgui::Style) {
    convert_cpp_to_rs! { style =>
        colors[ImGuiCol_Text] = ImVec4(1.00, 1.00, 1.00, 1.00);
        colors[ImGuiCol_TextDisabled] = ImVec4(0.50, 0.50, 0.50, 1.00);
        colors[ImGuiCol_WindowBg] = ImVec4(0.03, 0.07, 0.04, 0.94);
        colors[ImGuiCol_ChildBg] = ImVec4(0.00, 0.00, 0.00, 0.00);
        colors[ImGuiCol_PopupBg] = ImVec4(0.08, 0.08, 0.08, 0.94);
        colors[ImGuiCol_Border] = ImVec4(0.00, 1.00, 0.87, 0.50);
        colors[ImGuiCol_BorderShadow] = ImVec4(0.01, 0.13, 0.00, 0.63);
        colors[ImGuiCol_FrameBg] = ImVec4(0.16, 0.48, 0.42, 0.54);
        colors[ImGuiCol_FrameBgHovered] = ImVec4(0.26, 0.98, 0.96, 0.40);
        colors[ImGuiCol_FrameBgActive] = ImVec4(0.26, 0.98, 0.94, 0.67);
        colors[ImGuiCol_TitleBg] = ImVec4(0.01, 0.07, 0.01, 1.00);
        colors[ImGuiCol_TitleBgActive] = ImVec4(0.00, 0.56, 0.51, 1.00);
        colors[ImGuiCol_TitleBgCollapsed] = ImVec4(0.00, 0.56, 0.53, 0.51);
        colors[ImGuiCol_MenuBarBg] = ImVec4(0.00, 0.47, 0.56, 1.00);
        colors[ImGuiCol_ScrollbarBg] = ImVec4(0.00, 0.15, 0.14, 0.53);
        colors[ImGuiCol_ScrollbarGrab] = ImVec4(0.06, 0.41, 0.38, 1.00);
        colors[ImGuiCol_ScrollbarGrabHovered] = ImVec4(0.00, 0.66, 0.65, 1.00);
        colors[ImGuiCol_ScrollbarGrabActive] = ImVec4(0.00, 0.87, 0.82, 1.00);
        colors[ImGuiCol_CheckMark] = ImVec4(0.26, 0.94, 0.98, 1.00);
        colors[ImGuiCol_SliderGrab] = ImVec4(0.00, 0.61, 0.43, 1.00);
        colors[ImGuiCol_SliderGrabActive] = ImVec4(0.22, 0.87, 0.68, 1.00);
        colors[ImGuiCol_Button] = ImVec4(0.00, 0.60, 0.60, 0.40);
        colors[ImGuiCol_ButtonHovered] = ImVec4(0.20, 0.78, 0.77, 1.00);
        colors[ImGuiCol_ButtonActive] = ImVec4(0.00, 0.57, 0.56, 1.00);
        colors[ImGuiCol_Header] = ImVec4(0.12, 0.82, 0.78, 0.31);
        colors[ImGuiCol_HeaderHovered] = ImVec4(0.00, 0.74, 0.73, 0.80);
        colors[ImGuiCol_HeaderActive] = ImVec4(0.04, 0.69, 0.65, 1.00);
        colors[ImGuiCol_Separator] = ImVec4(0.09, 0.67, 0.01, 0.50);
        colors[ImGuiCol_SeparatorHovered] = ImVec4(0.32, 0.75, 0.10, 0.78);
        colors[ImGuiCol_SeparatorActive] = ImVec4(0.10, 0.75, 0.11, 1.00);
        colors[ImGuiCol_ResizeGrip] = ImVec4(0.26, 0.98, 0.85, 0.20);
        colors[ImGuiCol_ResizeGripHovered] = ImVec4(0.26, 0.98, 0.94, 0.67);
        colors[ImGuiCol_ResizeGripActive] = ImVec4(0.06, 0.69, 0.50, 0.95);
        colors[ImGuiCol_Tab] = ImVec4(0.18, 0.58, 0.56, 0.86);
        colors[ImGuiCol_TabHovered] = ImVec4(0.26, 0.98, 0.94, 0.80);
        colors[ImGuiCol_TabActive] = ImVec4(0.20, 0.68, 0.67, 1.00);
        colors[ImGuiCol_TabUnfocused] = ImVec4(0.07, 0.15, 0.08, 0.97);
        colors[ImGuiCol_TabUnfocusedActive] = ImVec4(0.14, 0.42, 0.19, 1.00);
        colors[ImGuiCol_DockingPreview] = ImVec4(0.26, 0.59, 0.98, 0.70);
        colors[ImGuiCol_DockingEmptyBg] = ImVec4(0.20, 0.20, 0.20, 1.00);
        colors[ImGuiCol_PlotLines] = ImVec4(0.61, 0.61, 0.61, 1.00);
        colors[ImGuiCol_PlotLinesHovered] = ImVec4(1.00, 0.43, 0.35, 1.00);
        colors[ImGuiCol_PlotHistogram] = ImVec4(0.90, 0.70, 0.00, 1.00);
        colors[ImGuiCol_PlotHistogramHovered] = ImVec4(1.00, 0.60, 0.00, 1.00);
        colors[ImGuiCol_TableHeaderBg] = ImVec4(0.19, 0.19, 0.20, 1.00);
        colors[ImGuiCol_TableBorderStrong] = ImVec4(0.31, 0.31, 0.35, 1.00);
        colors[ImGuiCol_TableBorderLight] = ImVec4(0.23, 0.23, 0.25, 1.00);
        colors[ImGuiCol_TableRowBg] = ImVec4(0.00, 0.00, 0.00, 0.00);
        colors[ImGuiCol_TableRowBgAlt] = ImVec4(1.00, 1.00, 1.00, 0.06);
        colors[ImGuiCol_TextSelectedBg] = ImVec4(0.00, 0.89, 0.88, 0.35);
        colors[ImGuiCol_DragDropTarget] = ImVec4(1.00, 1.00, 0.00, 0.90);
        colors[ImGuiCol_NavHighlight] = ImVec4(0.26, 0.98, 0.35, 1.00);
        colors[ImGuiCol_NavWindowingHighlight] = ImVec4(1.00, 1.00, 1.00, 0.70);
        colors[ImGuiCol_NavWindowingDimBg] = ImVec4(0.80, 0.80, 0.80, 0.20);
        colors[ImGuiCol_ModalWindowDimBg] = ImVec4(0.80, 0.80, 0.80, 0.35);
    }
}
