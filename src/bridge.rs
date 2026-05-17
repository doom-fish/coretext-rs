#![allow(non_snake_case)]

use core::ffi::{c_char, c_void};

use crate::types::{CFRange, CGAffineTransform, CGPoint, CGRect, CGSize};

pub type Handle = *mut c_void;

unsafe extern "C" {
    pub fn ct_retain(handle: Handle) -> Handle;
    pub fn ct_release(handle: Handle);
    pub fn ct_string_release(value: *mut c_char);

    pub fn ct_attributed_string_create(
        text: *const c_char,
        font: Handle,
        paragraph_style: Handle,
    ) -> Handle;

    pub fn ct_font_create_with_name(name: *const c_char, size: f64) -> Handle;
    pub fn ct_font_create_with_descriptor(descriptor: Handle, size: f64) -> Handle;
    pub fn ct_font_create_ui_font(ui_type: u32, size: f64, language: *const c_char) -> Handle;
    pub fn ct_font_copy_with_attributes(font: Handle, size: f64, descriptor: Handle) -> Handle;
    pub fn ct_font_copy_with_family(font: Handle, size: f64, family: *const c_char) -> Handle;
    pub fn ct_font_copy_with_symbolic_traits(
        font: Handle,
        size: f64,
        trait_value: u32,
        trait_mask: u32,
    ) -> Handle;
    pub fn ct_font_create_for_string(
        font: Handle,
        string: *const c_char,
        range: CFRange,
        language: *const c_char,
    ) -> Handle;
    pub fn ct_font_copy_descriptor(font: Handle) -> Handle;
    pub fn ct_font_get_size(font: Handle) -> f64;
    pub fn ct_font_get_matrix(font: Handle) -> CGAffineTransform;
    pub fn ct_font_get_symbolic_traits(font: Handle) -> u32;
    pub fn ct_font_copy_traits_json(font: Handle) -> *mut c_char;
    pub fn ct_font_copy_postscript_name(font: Handle) -> *mut c_char;
    pub fn ct_font_copy_family_name(font: Handle) -> *mut c_char;
    pub fn ct_font_copy_full_name(font: Handle) -> *mut c_char;
    pub fn ct_font_copy_display_name(font: Handle) -> *mut c_char;
    pub fn ct_font_copy_name(font: Handle, key: u32) -> *mut c_char;
    pub fn ct_font_copy_localized_name(font: Handle, key: u32) -> *mut c_char;
    pub fn ct_font_get_ascent(font: Handle) -> f64;
    pub fn ct_font_get_descent(font: Handle) -> f64;
    pub fn ct_font_get_leading(font: Handle) -> f64;
    pub fn ct_font_get_units_per_em(font: Handle) -> u32;
    pub fn ct_font_get_glyph_count(font: Handle) -> isize;
    pub fn ct_font_get_bounding_box(font: Handle) -> CGRect;
    pub fn ct_font_get_underline_position(font: Handle) -> f64;
    pub fn ct_font_get_underline_thickness(font: Handle) -> f64;
    pub fn ct_font_get_slant_angle(font: Handle) -> f64;
    pub fn ct_font_get_cap_height(font: Handle) -> f64;
    pub fn ct_font_get_x_height(font: Handle) -> f64;
    pub fn ct_font_copy_supported_languages_json(font: Handle) -> *mut c_char;
    pub fn ct_font_get_glyphs_for_characters(
        font: Handle,
        characters: *const u16,
        glyphs: *mut u16,
        count: isize,
    ) -> bool;
    pub fn ct_font_get_glyph_with_name(font: Handle, glyph_name: *const c_char) -> u16;
    pub fn ct_font_copy_name_for_glyph(font: Handle, glyph: u16) -> *mut c_char;
    pub fn ct_font_get_bounding_rects_for_glyphs(
        font: Handle,
        orientation: u32,
        glyphs: *const u16,
        bounding_rects: *mut CGRect,
        count: isize,
    ) -> CGRect;
    pub fn ct_font_get_optical_bounds_for_glyphs(
        font: Handle,
        glyphs: *const u16,
        bounding_rects: *mut CGRect,
        count: isize,
    ) -> CGRect;
    pub fn ct_font_get_advances_for_glyphs(
        font: Handle,
        orientation: u32,
        glyphs: *const u16,
        advances: *mut CGSize,
        count: isize,
    ) -> f64;
    pub fn ct_font_get_vertical_translations_for_glyphs(
        font: Handle,
        glyphs: *const u16,
        translations: *mut CGSize,
        count: isize,
    );
    pub fn ct_font_copy_variation_axes_json(font: Handle) -> *mut c_char;
    pub fn ct_font_copy_variation_json(font: Handle) -> *mut c_char;
    pub fn ct_font_copy_features_json(font: Handle) -> *mut c_char;
    pub fn ct_font_copy_feature_settings_json(font: Handle) -> *mut c_char;
    pub fn ct_font_copy_available_tables_json(font: Handle) -> *mut c_char;
    pub fn ct_font_has_table(font: Handle, tag: u32) -> bool;

    pub fn ct_font_descriptor_create(name: *const c_char, size: f64) -> Handle;
    pub fn ct_font_descriptor_copy_with_family(descriptor: Handle, family: *const c_char)
        -> Handle;
    pub fn ct_font_descriptor_copy_with_symbolic_traits(
        descriptor: Handle,
        trait_value: u32,
        trait_mask: u32,
    ) -> Handle;
    pub fn ct_font_descriptor_copy_with_variation(
        descriptor: Handle,
        variation_identifier: u32,
        variation_value: f64,
    ) -> Handle;
    pub fn ct_font_descriptor_copy_with_feature(
        descriptor: Handle,
        feature_type_identifier: i64,
        feature_selector_identifier: i64,
    ) -> Handle;
    pub fn ct_font_descriptor_create_matching_descriptor(descriptor: Handle) -> Handle;
    pub fn ct_font_descriptor_get_matching_descriptor_count(descriptor: Handle) -> isize;
    pub fn ct_font_descriptor_copy_matching_descriptors(
        descriptor: Handle,
        buffer: *mut Handle,
        capacity: isize,
    ) -> isize;
    pub fn ct_font_descriptor_copy_postscript_name(descriptor: Handle) -> *mut c_char;
    pub fn ct_font_descriptor_copy_display_name(descriptor: Handle) -> *mut c_char;
    pub fn ct_font_descriptor_copy_family_name(descriptor: Handle) -> *mut c_char;
    pub fn ct_font_descriptor_copy_style_name(descriptor: Handle) -> *mut c_char;
    pub fn ct_font_descriptor_copy_url_path(descriptor: Handle) -> *mut c_char;
    pub fn ct_font_descriptor_get_size(descriptor: Handle) -> f64;
    pub fn ct_font_descriptor_get_orientation(descriptor: Handle) -> u32;
    pub fn ct_font_descriptor_get_format(descriptor: Handle) -> u32;
    pub fn ct_font_descriptor_is_enabled(descriptor: Handle) -> bool;
    pub fn ct_font_descriptor_is_downloadable(descriptor: Handle) -> bool;
    pub fn ct_font_descriptor_copy_traits_json(descriptor: Handle) -> *mut c_char;
    pub fn ct_font_descriptor_copy_variation_axes_json(descriptor: Handle) -> *mut c_char;
    pub fn ct_font_descriptor_copy_variation_json(descriptor: Handle) -> *mut c_char;
    pub fn ct_font_descriptor_copy_features_json(descriptor: Handle) -> *mut c_char;
    pub fn ct_font_descriptor_copy_feature_settings_json(descriptor: Handle) -> *mut c_char;
    pub fn ct_font_descriptor_copy_attributes_json(descriptor: Handle) -> *mut c_char;

    pub fn ct_font_collection_create_available(options_json: *const c_char) -> Handle;
    pub fn ct_font_collection_create_with_descriptors(
        descriptors: *const Handle,
        descriptor_count: isize,
        options_json: *const c_char,
    ) -> Handle;
    pub fn ct_font_collection_copy_with_descriptors(
        collection: Handle,
        descriptors: *const Handle,
        descriptor_count: isize,
        options_json: *const c_char,
    ) -> Handle;
    pub fn ct_font_collection_get_query_descriptor_count(collection: Handle) -> isize;
    pub fn ct_font_collection_copy_query_descriptors(
        collection: Handle,
        buffer: *mut Handle,
        capacity: isize,
    ) -> isize;
    pub fn ct_font_collection_get_matching_descriptor_count(collection: Handle) -> isize;
    pub fn ct_font_collection_copy_matching_descriptors(
        collection: Handle,
        buffer: *mut Handle,
        capacity: isize,
    ) -> isize;
    pub fn ct_font_collection_get_matching_descriptors_for_family_count(
        collection: Handle,
        family_name: *const c_char,
    ) -> isize;
    pub fn ct_font_collection_copy_matching_descriptors_for_family(
        collection: Handle,
        family_name: *const c_char,
        buffer: *mut Handle,
        capacity: isize,
    ) -> isize;

    pub fn ct_font_manager_copy_available_postscript_names_json() -> *mut c_char;
    pub fn ct_font_manager_copy_available_font_family_names_json() -> *mut c_char;
    pub fn ct_font_manager_copy_available_font_urls_json() -> *mut c_char;
    pub fn ct_font_manager_get_descriptor_count_for_url(url_path: *const c_char) -> isize;
    pub fn ct_font_manager_copy_descriptors_from_url(
        url_path: *const c_char,
        buffer: *mut Handle,
        capacity: isize,
    ) -> isize;
    pub fn ct_font_manager_is_supported_font(url_path: *const c_char) -> bool;
    pub fn ct_font_manager_register_fonts_for_url(
        url_path: *const c_char,
        scope: u32,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn ct_font_manager_unregister_fonts_for_url(
        url_path: *const c_char,
        scope: u32,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn ct_font_manager_get_scope_for_url(url_path: *const c_char) -> u32;
    pub fn ct_font_manager_get_auto_activation_setting(bundle_identifier: *const c_char) -> u32;
    pub fn ct_font_manager_set_auto_activation_setting(
        bundle_identifier: *const c_char,
        setting: u32,
    );

    pub fn ct_frame_get_string_range(frame: Handle) -> CFRange;
    pub fn ct_frame_get_visible_string_range(frame: Handle) -> CFRange;
    pub fn ct_frame_copy_path_bounding_box(frame: Handle) -> CGRect;
    pub fn ct_frame_has_frame_attributes(frame: Handle) -> bool;
    pub fn ct_frame_get_line_count(frame: Handle) -> isize;
    pub fn ct_frame_copy_lines(frame: Handle, buffer: *mut Handle, capacity: isize) -> isize;
    pub fn ct_frame_copy_line_origins(
        frame: Handle,
        buffer: *mut CGPoint,
        capacity: isize,
    ) -> isize;

    pub fn ct_framesetter_create_with_attributed_string(attributed_string: Handle) -> Handle;
    pub fn ct_framesetter_create_with_typesetter(typesetter: Handle) -> Handle;
    pub fn ct_framesetter_copy_typesetter(framesetter: Handle) -> Handle;
    pub fn ct_framesetter_suggest_frame_size(
        framesetter: Handle,
        string_range: CFRange,
        constraints: CGSize,
        fit_range: *mut CFRange,
    ) -> CGSize;
    pub fn ct_framesetter_create_frame_in_rect(
        framesetter: Handle,
        string_range: CFRange,
        rect: CGRect,
    ) -> Handle;

    pub fn ct_glyph_info_create_with_glyph_name(
        glyph_name: *const c_char,
        font: Handle,
        base_string: *const c_char,
    ) -> Handle;
    pub fn ct_glyph_info_create_with_glyph(
        glyph: u16,
        font: Handle,
        base_string: *const c_char,
    ) -> Handle;
    pub fn ct_glyph_info_create_with_character_identifier(
        character_identifier: u16,
        collection: u16,
        base_string: *const c_char,
    ) -> Handle;
    pub fn ct_glyph_info_copy_glyph_name(glyph_info: Handle) -> *mut c_char;
    pub fn ct_glyph_info_get_glyph(glyph_info: Handle) -> u16;
    pub fn ct_glyph_info_get_character_identifier(glyph_info: Handle) -> u16;
    pub fn ct_glyph_info_get_character_collection(glyph_info: Handle) -> u16;

    pub fn ct_line_create_with_attributed_string(attributed_string: Handle) -> Handle;
    pub fn ct_line_create_truncated_line(
        line: Handle,
        width: f64,
        truncation_type: u32,
        truncation_token: Handle,
    ) -> Handle;
    pub fn ct_line_create_justified_line(
        line: Handle,
        justification_factor: f64,
        justification_width: f64,
    ) -> Handle;
    pub fn ct_line_get_glyph_count(line: Handle) -> isize;
    pub fn ct_line_get_string_range(line: Handle) -> CFRange;
    pub fn ct_line_get_pen_offset_for_flush(
        line: Handle,
        flush_factor: f64,
        flush_width: f64,
    ) -> f64;
    pub fn ct_line_get_typographic_bounds(
        line: Handle,
        ascent: *mut f64,
        descent: *mut f64,
        leading: *mut f64,
    ) -> f64;
    pub fn ct_line_get_bounds_with_options(line: Handle, options: u64) -> CGRect;
    pub fn ct_line_get_trailing_whitespace_width(line: Handle) -> f64;
    pub fn ct_line_get_image_bounds(line: Handle) -> CGRect;
    pub fn ct_line_get_string_index_for_position(line: Handle, position: CGPoint) -> isize;
    pub fn ct_line_get_offset_for_string_index(
        line: Handle,
        char_index: isize,
        secondary_offset: *mut f64,
    ) -> f64;
    pub fn ct_line_get_run_count(line: Handle) -> isize;
    pub fn ct_line_copy_runs(line: Handle, buffer: *mut Handle, capacity: isize) -> isize;

    pub fn ct_paragraph_style_create(
        options_json: *const c_char,
        text_tabs: *const Handle,
        tab_count: isize,
    ) -> Handle;
    pub fn ct_paragraph_style_copy(paragraph_style: Handle) -> Handle;
    pub fn ct_paragraph_style_get_alignment(paragraph_style: Handle) -> u8;
    pub fn ct_paragraph_style_get_first_line_head_indent(paragraph_style: Handle) -> f64;
    pub fn ct_paragraph_style_get_head_indent(paragraph_style: Handle) -> f64;
    pub fn ct_paragraph_style_get_tail_indent(paragraph_style: Handle) -> f64;
    pub fn ct_paragraph_style_get_default_tab_interval(paragraph_style: Handle) -> f64;
    pub fn ct_paragraph_style_get_line_break_mode(paragraph_style: Handle) -> u8;
    pub fn ct_paragraph_style_get_line_height_multiple(paragraph_style: Handle) -> f64;
    pub fn ct_paragraph_style_get_maximum_line_height(paragraph_style: Handle) -> f64;
    pub fn ct_paragraph_style_get_minimum_line_height(paragraph_style: Handle) -> f64;
    pub fn ct_paragraph_style_get_paragraph_spacing(paragraph_style: Handle) -> f64;
    pub fn ct_paragraph_style_get_paragraph_spacing_before(paragraph_style: Handle) -> f64;
    pub fn ct_paragraph_style_get_base_writing_direction(paragraph_style: Handle) -> i8;
    pub fn ct_paragraph_style_get_maximum_line_spacing(paragraph_style: Handle) -> f64;
    pub fn ct_paragraph_style_get_minimum_line_spacing(paragraph_style: Handle) -> f64;
    pub fn ct_paragraph_style_get_line_spacing_adjustment(paragraph_style: Handle) -> f64;
    pub fn ct_paragraph_style_get_line_bounds_options(paragraph_style: Handle) -> u64;
    pub fn ct_paragraph_style_get_text_tab_count(paragraph_style: Handle) -> isize;
    pub fn ct_paragraph_style_copy_text_tabs(
        paragraph_style: Handle,
        buffer: *mut Handle,
        capacity: isize,
    ) -> isize;

    pub fn ct_run_get_glyph_count(run: Handle) -> isize;
    pub fn ct_run_get_status(run: Handle) -> u32;
    pub fn ct_run_copy_attributes_json(run: Handle) -> *mut c_char;
    pub fn ct_run_copy_glyphs(run: Handle, buffer: *mut u16, capacity: isize) -> isize;
    pub fn ct_run_copy_positions(run: Handle, buffer: *mut CGPoint, capacity: isize) -> isize;
    pub fn ct_run_copy_advances(run: Handle, buffer: *mut CGSize, capacity: isize) -> isize;
    pub fn ct_run_copy_string_indices(run: Handle, buffer: *mut isize, capacity: isize) -> isize;
    pub fn ct_run_get_string_range(run: Handle) -> CFRange;
    pub fn ct_run_get_typographic_bounds(
        run: Handle,
        ascent: *mut f64,
        descent: *mut f64,
        leading: *mut f64,
    ) -> f64;
    pub fn ct_run_get_image_bounds(run: Handle) -> CGRect;
    pub fn ct_run_get_text_matrix(run: Handle) -> CGAffineTransform;
    pub fn ct_run_copy_base_advances_and_origins(
        run: Handle,
        advances: *mut CGSize,
        origins: *mut CGPoint,
        capacity: isize,
    ) -> isize;

    pub fn ct_text_tab_create(alignment: u8, location: f64) -> Handle;
    pub fn ct_text_tab_get_alignment(text_tab: Handle) -> u8;
    pub fn ct_text_tab_get_location(text_tab: Handle) -> f64;

    pub fn ct_typesetter_create_with_attributed_string(
        attributed_string: Handle,
        options_json: *const c_char,
    ) -> Handle;
    pub fn ct_typesetter_create_line(typesetter: Handle, string_range: CFRange) -> Handle;
    pub fn ct_typesetter_create_line_with_offset(
        typesetter: Handle,
        string_range: CFRange,
        offset: f64,
    ) -> Handle;
    pub fn ct_typesetter_suggest_line_break(
        typesetter: Handle,
        start_index: isize,
        width: f64,
    ) -> isize;
    pub fn ct_typesetter_suggest_line_break_with_offset(
        typesetter: Handle,
        start_index: isize,
        width: f64,
        offset: f64,
    ) -> isize;
    pub fn ct_typesetter_suggest_cluster_break(
        typesetter: Handle,
        start_index: isize,
        width: f64,
    ) -> isize;
    pub fn ct_typesetter_suggest_cluster_break_with_offset(
        typesetter: Handle,
        start_index: isize,
        width: f64,
        offset: f64,
    ) -> isize;

    pub fn ct_ruby_annotation_create(
        alignment: u8,
        overhang: u8,
        size_factor: f64,
        before_text: *const c_char,
        after_text: *const c_char,
        inter_character_text: *const c_char,
        inline_text: *const c_char,
    ) -> Handle;
    pub fn ct_ruby_annotation_copy(ruby_annotation: Handle) -> Handle;
    pub fn ct_ruby_annotation_get_alignment(ruby_annotation: Handle) -> u8;
    pub fn ct_ruby_annotation_get_overhang(ruby_annotation: Handle) -> u8;
    pub fn ct_ruby_annotation_get_size_factor(ruby_annotation: Handle) -> f64;
    pub fn ct_ruby_annotation_copy_text_for_position(
        ruby_annotation: Handle,
        position: u8,
    ) -> *mut c_char;
}

unsafe extern "C" {
    pub fn ct_font_collection_get_exclusion_descriptor_count(collection: Handle) -> isize;
    pub fn ct_font_collection_copy_exclusion_descriptors(
        collection: Handle,
        buffer: *mut Handle,
        capacity: isize,
    ) -> isize;
    pub fn ct_font_collection_copy_font_attribute_json(
        collection: Handle,
        attribute_name: *const c_char,
    ) -> *mut c_char;
    pub fn ct_font_collection_copy_font_attributes_json(
        collection: Handle,
        attribute_names_json: *const c_char,
    ) -> *mut c_char;
    pub fn ct_font_collection_matching_with_options_count(
        collection: Handle,
        options_json: *const c_char,
    ) -> isize;
    pub fn ct_font_collection_copy_matching_with_options(
        collection: Handle,
        options_json: *const c_char,
        buffer: *mut Handle,
        capacity: isize,
    ) -> isize;
    pub fn ct_font_collection_create_mutable_copy(collection: Handle) -> Handle;
    pub fn ct_font_collection_get_type_id() -> u64;
    pub fn ct_font_collection_set_exclusion_descriptors(
        collection: Handle,
        descriptors: *const Handle,
        count: isize,
    );
    pub fn ct_font_collection_set_query_descriptors(
        collection: Handle,
        descriptors: *const Handle,
        count: isize,
    );

    pub fn ct_font_copy_attribute_json(font: Handle, attribute_name: *const c_char) -> *mut c_char;
    pub fn ct_font_copy_default_cascade_list_count(
        font: Handle,
        languages_json: *const c_char,
    ) -> isize;
    pub fn ct_font_copy_default_cascade_list(
        font: Handle,
        languages_json: *const c_char,
        buffer: *mut Handle,
        capacity: isize,
    ) -> isize;
    pub fn ct_font_copy_table_bytes(font: Handle, tag: u32, out_len: *mut isize) -> *mut u8;
    pub fn ct_font_create_with_descriptor_and_options(
        descriptor: Handle,
        size: f64,
        options: u32,
    ) -> Handle;
    pub fn ct_font_create_with_name_and_options(
        name: *const c_char,
        size: f64,
        options: u32,
    ) -> Handle;
    pub fn ct_font_get_ligature_caret_positions(
        font: Handle,
        glyph: u16,
        buffer: *mut f64,
        max_positions: isize,
    ) -> isize;
    pub fn ct_font_get_string_encoding(font: Handle) -> u32;
    pub fn ct_font_get_type_id() -> u64;

    pub fn ct_font_descriptor_copy_attribute_json(
        descriptor: Handle,
        attribute_name: *const c_char,
    ) -> *mut c_char;
    pub fn ct_font_descriptor_copy_localized_attribute_json(
        descriptor: Handle,
        attribute_name: *const c_char,
    ) -> *mut c_char;
    pub fn ct_font_descriptor_create_copy_with_attributes_json(
        descriptor: Handle,
        attrs_json: *const c_char,
    ) -> Handle;
    pub fn ct_font_descriptor_create_with_attributes_json(attrs_json: *const c_char) -> Handle;
    pub fn ct_font_descriptor_get_type_id() -> u64;

    pub fn ct_font_manager_copy_registered_descriptor_count(scope: u32, enabled: bool) -> isize;
    pub fn ct_font_manager_copy_registered_descriptors(
        scope: u32,
        enabled: bool,
        buffer: *mut Handle,
        capacity: isize,
    ) -> isize;
    pub fn ct_font_manager_create_descriptor_from_data(
        bytes: *const u8,
        length: isize,
    ) -> Handle;
    pub fn ct_font_manager_create_descriptors_from_data_count(
        bytes: *const u8,
        length: isize,
    ) -> isize;
    pub fn ct_font_manager_create_descriptors_from_data(
        bytes: *const u8,
        length: isize,
        buffer: *mut Handle,
        capacity: isize,
    ) -> isize;
    pub fn ct_font_manager_enable_font_descriptors(
        descriptors: *const Handle,
        count: isize,
        enable: bool,
    );
    pub fn ct_font_manager_register_font_descriptors(
        descriptors: *const Handle,
        count: isize,
        scope: u32,
        enabled: bool,
    ) -> *mut c_char;
    pub fn ct_font_manager_register_font_urls(
        url_paths_json: *const c_char,
        scope: u32,
        enabled: bool,
    ) -> *mut c_char;
    pub fn ct_font_manager_register_fonts_for_urls(
        url_paths_json: *const c_char,
        scope: u32,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn ct_font_manager_register_fonts_with_asset_names(
        asset_names_json: *const c_char,
        scope: u32,
        enabled: bool,
    ) -> bool;
    pub fn ct_font_manager_unregister_font_descriptors(
        descriptors: *const Handle,
        count: isize,
        scope: u32,
    ) -> *mut c_char;
    pub fn ct_font_manager_unregister_font_urls(
        url_paths_json: *const c_char,
        scope: u32,
    ) -> *mut c_char;
    pub fn ct_font_manager_unregister_fonts_for_urls(
        url_paths_json: *const c_char,
        scope: u32,
        error_out: *mut *mut c_char,
    ) -> bool;

    pub fn ct_frame_copy_frame_attributes_json(frame: Handle) -> *mut c_char;
    pub fn ct_frame_get_type_id() -> u64;

    pub fn ct_framesetter_get_type_id() -> u64;
    pub fn ct_glyph_info_get_type_id() -> u64;
    pub fn ct_line_get_type_id() -> u64;
    pub fn ct_paragraph_style_get_type_id() -> u64;
    pub fn ct_paragraph_style_get_value_for_specifier_json(
        paragraph_style: Handle,
        specifier: u32,
    ) -> *mut c_char;
    pub fn ct_ruby_annotation_create_with_attributes_json(attrs_json: *const c_char) -> Handle;
    pub fn ct_ruby_annotation_get_type_id() -> u64;
    pub fn ct_run_get_type_id() -> u64;
    pub fn ct_text_tab_get_options_json(text_tab: Handle) -> *mut c_char;
    pub fn ct_text_tab_get_type_id() -> u64;
    pub fn ct_typesetter_get_type_id() -> u64;
}
