#ifndef CORE_TEXT_ABI_H
#define CORE_TEXT_ABI_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct {
    const uint8_t *data;
    size_t len;
} CoreTextBytes;

typedef struct {
    const CoreTextBytes *data;
    size_t len;
} CoreTextStrings;

typedef struct {
    CoreTextBytes text;
    void *font;
} CoreTextTextRun;

typedef struct CoreTextValue {
    uint32_t tag;
    int64_t signed_value;
    uint64_t unsigned_value;
    double float_value;
    CoreTextBytes bytes;
    struct CoreTextValue *elements;
    CoreTextBytes *keys;
    size_t count;
} CoreTextValue;

enum {
    CoreTextNull = 0,
    CoreTextBool = 1,
    CoreTextSigned = 2,
    CoreTextUnsigned = 3,
    CoreTextFloat = 4,
    CoreTextString = 5,
    CoreTextData = 6,
    CoreTextArray = 7,
    CoreTextDictionary = 8
};

typedef struct {
    uint32_t symbolic_traits;
    double weight;
    double width;
    double slant;
} CoreTextFontTraits;

typedef struct {
    CoreTextBytes tag;
    uint32_t value;
} CoreTextFeature;

typedef struct {
    const CoreTextFeature *data;
    size_t len;
} CoreTextFeatures;

typedef struct {
    bool remove_duplicates;
    bool include_disabled_fonts;
    bool disallow_auto_activation;
} CoreTextCollectionOptions;

typedef struct {
    bool allow_unbounded_layout;
    bool has_forced_embedding_level;
    int64_t forced_embedding_level;
} CoreTextTypesetterOptions;

typedef struct {
    uint64_t present;
    uint8_t alignment;
    double first_line_head_indent;
    double head_indent;
    double tail_indent;
    double default_tab_interval;
    uint8_t line_break_mode;
    double line_height_multiple;
    double maximum_line_height;
    double minimum_line_height;
    double paragraph_spacing;
    double paragraph_spacing_before;
    int8_t base_writing_direction;
    double maximum_line_spacing;
    double minimum_line_spacing;
    double line_spacing_adjustment;
    uint64_t line_bounds_options;
} CoreTextParagraphOptions;

typedef struct {
    uint8_t alignment;
    uint8_t overhang;
    double size_factor;
    CoreTextStrings texts;
} CoreTextRubyOptions;

void ct_value_release(CoreTextValue *value);

#endif
