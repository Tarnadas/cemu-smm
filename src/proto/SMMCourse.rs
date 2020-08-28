// This file is generated by rust-protobuf 2.17.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `SMMCourse.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_17_0;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct SMMCourse {
    // message fields
    pub modified: u64,
    pub title: ::std::string::String,
    pub maker: ::std::string::String,
    pub game_style: SMMCourse_GameStyle,
    pub course_theme: SMMCourse_CourseTheme,
    pub course_theme_sub: SMMCourse_CourseTheme,
    pub time: u32,
    pub auto_scroll: SMMCourse_AutoScroll,
    pub auto_scroll_sub: SMMCourse_AutoScroll,
    pub width: u32,
    pub width_sub: u32,
    pub tiles: ::protobuf::RepeatedField<super::Tile::Tile>,
    pub tiles_sub: ::protobuf::RepeatedField<super::Tile::Tile>,
    pub sounds: ::protobuf::RepeatedField<super::Sound::Sound>,
    pub sounds_sub: ::protobuf::RepeatedField<super::Sound::Sound>,
    pub thumbnail: ::bytes::Bytes,
    pub thumbnail_preview: ::bytes::Bytes,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SMMCourse {
    fn default() -> &'a SMMCourse {
        <SMMCourse as ::protobuf::Message>::default_instance()
    }
}

impl SMMCourse {
    pub fn new() -> SMMCourse {
        ::std::default::Default::default()
    }

    // uint64 modified = 1;


    pub fn get_modified(&self) -> u64 {
        self.modified
    }
    pub fn clear_modified(&mut self) {
        self.modified = 0;
    }

    // Param is passed by value, moved
    pub fn set_modified(&mut self, v: u64) {
        self.modified = v;
    }

    // string title = 2;


    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    // string maker = 3;


    pub fn get_maker(&self) -> &str {
        &self.maker
    }
    pub fn clear_maker(&mut self) {
        self.maker.clear();
    }

    // Param is passed by value, moved
    pub fn set_maker(&mut self, v: ::std::string::String) {
        self.maker = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_maker(&mut self) -> &mut ::std::string::String {
        &mut self.maker
    }

    // Take field
    pub fn take_maker(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.maker, ::std::string::String::new())
    }

    // .smmdb.SMMCourse.GameStyle game_style = 4;


    pub fn get_game_style(&self) -> SMMCourse_GameStyle {
        self.game_style
    }
    pub fn clear_game_style(&mut self) {
        self.game_style = SMMCourse_GameStyle::M1;
    }

    // Param is passed by value, moved
    pub fn set_game_style(&mut self, v: SMMCourse_GameStyle) {
        self.game_style = v;
    }

    // .smmdb.SMMCourse.CourseTheme course_theme = 5;


    pub fn get_course_theme(&self) -> SMMCourse_CourseTheme {
        self.course_theme
    }
    pub fn clear_course_theme(&mut self) {
        self.course_theme = SMMCourse_CourseTheme::GROUND;
    }

    // Param is passed by value, moved
    pub fn set_course_theme(&mut self, v: SMMCourse_CourseTheme) {
        self.course_theme = v;
    }

    // .smmdb.SMMCourse.CourseTheme course_theme_sub = 6;


    pub fn get_course_theme_sub(&self) -> SMMCourse_CourseTheme {
        self.course_theme_sub
    }
    pub fn clear_course_theme_sub(&mut self) {
        self.course_theme_sub = SMMCourse_CourseTheme::GROUND;
    }

    // Param is passed by value, moved
    pub fn set_course_theme_sub(&mut self, v: SMMCourse_CourseTheme) {
        self.course_theme_sub = v;
    }

    // uint32 time = 7;


    pub fn get_time(&self) -> u32 {
        self.time
    }
    pub fn clear_time(&mut self) {
        self.time = 0;
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: u32) {
        self.time = v;
    }

    // .smmdb.SMMCourse.AutoScroll auto_scroll = 8;


    pub fn get_auto_scroll(&self) -> SMMCourse_AutoScroll {
        self.auto_scroll
    }
    pub fn clear_auto_scroll(&mut self) {
        self.auto_scroll = SMMCourse_AutoScroll::DISABLED;
    }

    // Param is passed by value, moved
    pub fn set_auto_scroll(&mut self, v: SMMCourse_AutoScroll) {
        self.auto_scroll = v;
    }

    // .smmdb.SMMCourse.AutoScroll auto_scroll_sub = 9;


    pub fn get_auto_scroll_sub(&self) -> SMMCourse_AutoScroll {
        self.auto_scroll_sub
    }
    pub fn clear_auto_scroll_sub(&mut self) {
        self.auto_scroll_sub = SMMCourse_AutoScroll::DISABLED;
    }

    // Param is passed by value, moved
    pub fn set_auto_scroll_sub(&mut self, v: SMMCourse_AutoScroll) {
        self.auto_scroll_sub = v;
    }

    // uint32 width = 10;


    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn clear_width(&mut self) {
        self.width = 0;
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: u32) {
        self.width = v;
    }

    // uint32 width_sub = 11;


    pub fn get_width_sub(&self) -> u32 {
        self.width_sub
    }
    pub fn clear_width_sub(&mut self) {
        self.width_sub = 0;
    }

    // Param is passed by value, moved
    pub fn set_width_sub(&mut self, v: u32) {
        self.width_sub = v;
    }

    // repeated .smmdb.Tile tiles = 12;


    pub fn get_tiles(&self) -> &[super::Tile::Tile] {
        &self.tiles
    }
    pub fn clear_tiles(&mut self) {
        self.tiles.clear();
    }

    // Param is passed by value, moved
    pub fn set_tiles(&mut self, v: ::protobuf::RepeatedField<super::Tile::Tile>) {
        self.tiles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tiles(&mut self) -> &mut ::protobuf::RepeatedField<super::Tile::Tile> {
        &mut self.tiles
    }

    // Take field
    pub fn take_tiles(&mut self) -> ::protobuf::RepeatedField<super::Tile::Tile> {
        ::std::mem::replace(&mut self.tiles, ::protobuf::RepeatedField::new())
    }

    // repeated .smmdb.Tile tiles_sub = 13;


    pub fn get_tiles_sub(&self) -> &[super::Tile::Tile] {
        &self.tiles_sub
    }
    pub fn clear_tiles_sub(&mut self) {
        self.tiles_sub.clear();
    }

    // Param is passed by value, moved
    pub fn set_tiles_sub(&mut self, v: ::protobuf::RepeatedField<super::Tile::Tile>) {
        self.tiles_sub = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tiles_sub(&mut self) -> &mut ::protobuf::RepeatedField<super::Tile::Tile> {
        &mut self.tiles_sub
    }

    // Take field
    pub fn take_tiles_sub(&mut self) -> ::protobuf::RepeatedField<super::Tile::Tile> {
        ::std::mem::replace(&mut self.tiles_sub, ::protobuf::RepeatedField::new())
    }

    // repeated .smmdb.Sound sounds = 14;


    pub fn get_sounds(&self) -> &[super::Sound::Sound] {
        &self.sounds
    }
    pub fn clear_sounds(&mut self) {
        self.sounds.clear();
    }

    // Param is passed by value, moved
    pub fn set_sounds(&mut self, v: ::protobuf::RepeatedField<super::Sound::Sound>) {
        self.sounds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sounds(&mut self) -> &mut ::protobuf::RepeatedField<super::Sound::Sound> {
        &mut self.sounds
    }

    // Take field
    pub fn take_sounds(&mut self) -> ::protobuf::RepeatedField<super::Sound::Sound> {
        ::std::mem::replace(&mut self.sounds, ::protobuf::RepeatedField::new())
    }

    // repeated .smmdb.Sound sounds_sub = 15;


    pub fn get_sounds_sub(&self) -> &[super::Sound::Sound] {
        &self.sounds_sub
    }
    pub fn clear_sounds_sub(&mut self) {
        self.sounds_sub.clear();
    }

    // Param is passed by value, moved
    pub fn set_sounds_sub(&mut self, v: ::protobuf::RepeatedField<super::Sound::Sound>) {
        self.sounds_sub = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sounds_sub(&mut self) -> &mut ::protobuf::RepeatedField<super::Sound::Sound> {
        &mut self.sounds_sub
    }

    // Take field
    pub fn take_sounds_sub(&mut self) -> ::protobuf::RepeatedField<super::Sound::Sound> {
        ::std::mem::replace(&mut self.sounds_sub, ::protobuf::RepeatedField::new())
    }

    // bytes thumbnail = 16;


    pub fn get_thumbnail(&self) -> &[u8] {
        &self.thumbnail
    }
    pub fn clear_thumbnail(&mut self) {
        self.thumbnail.clear();
    }

    // Param is passed by value, moved
    pub fn set_thumbnail(&mut self, v: ::bytes::Bytes) {
        self.thumbnail = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_thumbnail(&mut self) -> &mut ::bytes::Bytes {
        &mut self.thumbnail
    }

    // Take field
    pub fn take_thumbnail(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.thumbnail, ::bytes::Bytes::new())
    }

    // bytes thumbnail_preview = 17;


    pub fn get_thumbnail_preview(&self) -> &[u8] {
        &self.thumbnail_preview
    }
    pub fn clear_thumbnail_preview(&mut self) {
        self.thumbnail_preview.clear();
    }

    // Param is passed by value, moved
    pub fn set_thumbnail_preview(&mut self, v: ::bytes::Bytes) {
        self.thumbnail_preview = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_thumbnail_preview(&mut self) -> &mut ::bytes::Bytes {
        &mut self.thumbnail_preview
    }

    // Take field
    pub fn take_thumbnail_preview(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.thumbnail_preview, ::bytes::Bytes::new())
    }
}

impl ::protobuf::Message for SMMCourse {
    fn is_initialized(&self) -> bool {
        for v in &self.tiles {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tiles_sub {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.sounds {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.sounds_sub {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.modified = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.maker)?;
                },
                4 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.game_style, 4, &mut self.unknown_fields)?
                },
                5 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.course_theme, 5, &mut self.unknown_fields)?
                },
                6 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.course_theme_sub, 6, &mut self.unknown_fields)?
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time = tmp;
                },
                8 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.auto_scroll, 8, &mut self.unknown_fields)?
                },
                9 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.auto_scroll_sub, 9, &mut self.unknown_fields)?
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.width = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.width_sub = tmp;
                },
                12 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tiles)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tiles_sub)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sounds)?;
                },
                15 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sounds_sub)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.thumbnail)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.thumbnail_preview)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.modified != 0 {
            my_size += ::protobuf::rt::value_size(1, self.modified, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.title);
        }
        if !self.maker.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.maker);
        }
        if self.game_style != SMMCourse_GameStyle::M1 {
            my_size += ::protobuf::rt::enum_size(4, self.game_style);
        }
        if self.course_theme != SMMCourse_CourseTheme::GROUND {
            my_size += ::protobuf::rt::enum_size(5, self.course_theme);
        }
        if self.course_theme_sub != SMMCourse_CourseTheme::GROUND {
            my_size += ::protobuf::rt::enum_size(6, self.course_theme_sub);
        }
        if self.time != 0 {
            my_size += ::protobuf::rt::value_size(7, self.time, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.auto_scroll != SMMCourse_AutoScroll::DISABLED {
            my_size += ::protobuf::rt::enum_size(8, self.auto_scroll);
        }
        if self.auto_scroll_sub != SMMCourse_AutoScroll::DISABLED {
            my_size += ::protobuf::rt::enum_size(9, self.auto_scroll_sub);
        }
        if self.width != 0 {
            my_size += ::protobuf::rt::value_size(10, self.width, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.width_sub != 0 {
            my_size += ::protobuf::rt::value_size(11, self.width_sub, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tiles {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tiles_sub {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.sounds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.sounds_sub {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.thumbnail.is_empty() {
            my_size += ::protobuf::rt::bytes_size(16, &self.thumbnail);
        }
        if !self.thumbnail_preview.is_empty() {
            my_size += ::protobuf::rt::bytes_size(17, &self.thumbnail_preview);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.modified != 0 {
            os.write_uint64(1, self.modified)?;
        }
        if !self.title.is_empty() {
            os.write_string(2, &self.title)?;
        }
        if !self.maker.is_empty() {
            os.write_string(3, &self.maker)?;
        }
        if self.game_style != SMMCourse_GameStyle::M1 {
            os.write_enum(4, ::protobuf::ProtobufEnum::value(&self.game_style))?;
        }
        if self.course_theme != SMMCourse_CourseTheme::GROUND {
            os.write_enum(5, ::protobuf::ProtobufEnum::value(&self.course_theme))?;
        }
        if self.course_theme_sub != SMMCourse_CourseTheme::GROUND {
            os.write_enum(6, ::protobuf::ProtobufEnum::value(&self.course_theme_sub))?;
        }
        if self.time != 0 {
            os.write_uint32(7, self.time)?;
        }
        if self.auto_scroll != SMMCourse_AutoScroll::DISABLED {
            os.write_enum(8, ::protobuf::ProtobufEnum::value(&self.auto_scroll))?;
        }
        if self.auto_scroll_sub != SMMCourse_AutoScroll::DISABLED {
            os.write_enum(9, ::protobuf::ProtobufEnum::value(&self.auto_scroll_sub))?;
        }
        if self.width != 0 {
            os.write_uint32(10, self.width)?;
        }
        if self.width_sub != 0 {
            os.write_uint32(11, self.width_sub)?;
        }
        for v in &self.tiles {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tiles_sub {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.sounds {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.sounds_sub {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.thumbnail.is_empty() {
            os.write_bytes(16, &self.thumbnail)?;
        }
        if !self.thumbnail_preview.is_empty() {
            os.write_bytes(17, &self.thumbnail_preview)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SMMCourse {
        SMMCourse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "modified",
                |m: &SMMCourse| { &m.modified },
                |m: &mut SMMCourse| { &mut m.modified },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "title",
                |m: &SMMCourse| { &m.title },
                |m: &mut SMMCourse| { &mut m.title },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "maker",
                |m: &SMMCourse| { &m.maker },
                |m: &mut SMMCourse| { &mut m.maker },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SMMCourse_GameStyle>>(
                "game_style",
                |m: &SMMCourse| { &m.game_style },
                |m: &mut SMMCourse| { &mut m.game_style },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SMMCourse_CourseTheme>>(
                "course_theme",
                |m: &SMMCourse| { &m.course_theme },
                |m: &mut SMMCourse| { &mut m.course_theme },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SMMCourse_CourseTheme>>(
                "course_theme_sub",
                |m: &SMMCourse| { &m.course_theme_sub },
                |m: &mut SMMCourse| { &mut m.course_theme_sub },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "time",
                |m: &SMMCourse| { &m.time },
                |m: &mut SMMCourse| { &mut m.time },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SMMCourse_AutoScroll>>(
                "auto_scroll",
                |m: &SMMCourse| { &m.auto_scroll },
                |m: &mut SMMCourse| { &mut m.auto_scroll },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SMMCourse_AutoScroll>>(
                "auto_scroll_sub",
                |m: &SMMCourse| { &m.auto_scroll_sub },
                |m: &mut SMMCourse| { &mut m.auto_scroll_sub },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "width",
                |m: &SMMCourse| { &m.width },
                |m: &mut SMMCourse| { &mut m.width },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "width_sub",
                |m: &SMMCourse| { &m.width_sub },
                |m: &mut SMMCourse| { &mut m.width_sub },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Tile::Tile>>(
                "tiles",
                |m: &SMMCourse| { &m.tiles },
                |m: &mut SMMCourse| { &mut m.tiles },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Tile::Tile>>(
                "tiles_sub",
                |m: &SMMCourse| { &m.tiles_sub },
                |m: &mut SMMCourse| { &mut m.tiles_sub },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Sound::Sound>>(
                "sounds",
                |m: &SMMCourse| { &m.sounds },
                |m: &mut SMMCourse| { &mut m.sounds },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Sound::Sound>>(
                "sounds_sub",
                |m: &SMMCourse| { &m.sounds_sub },
                |m: &mut SMMCourse| { &mut m.sounds_sub },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                "thumbnail",
                |m: &SMMCourse| { &m.thumbnail },
                |m: &mut SMMCourse| { &mut m.thumbnail },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                "thumbnail_preview",
                |m: &SMMCourse| { &m.thumbnail_preview },
                |m: &mut SMMCourse| { &mut m.thumbnail_preview },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SMMCourse>(
                "SMMCourse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SMMCourse {
        static instance: ::protobuf::rt::LazyV2<SMMCourse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SMMCourse::new)
    }
}

impl ::protobuf::Clear for SMMCourse {
    fn clear(&mut self) {
        self.modified = 0;
        self.title.clear();
        self.maker.clear();
        self.game_style = SMMCourse_GameStyle::M1;
        self.course_theme = SMMCourse_CourseTheme::GROUND;
        self.course_theme_sub = SMMCourse_CourseTheme::GROUND;
        self.time = 0;
        self.auto_scroll = SMMCourse_AutoScroll::DISABLED;
        self.auto_scroll_sub = SMMCourse_AutoScroll::DISABLED;
        self.width = 0;
        self.width_sub = 0;
        self.tiles.clear();
        self.tiles_sub.clear();
        self.sounds.clear();
        self.sounds_sub.clear();
        self.thumbnail.clear();
        self.thumbnail_preview.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SMMCourse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SMMCourse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum SMMCourse_GameStyle {
    M1 = 0,
    M3 = 1,
    MW = 2,
    WU = 3,
}

impl ::protobuf::ProtobufEnum for SMMCourse_GameStyle {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SMMCourse_GameStyle> {
        match value {
            0 => ::std::option::Option::Some(SMMCourse_GameStyle::M1),
            1 => ::std::option::Option::Some(SMMCourse_GameStyle::M3),
            2 => ::std::option::Option::Some(SMMCourse_GameStyle::MW),
            3 => ::std::option::Option::Some(SMMCourse_GameStyle::WU),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SMMCourse_GameStyle] = &[
            SMMCourse_GameStyle::M1,
            SMMCourse_GameStyle::M3,
            SMMCourse_GameStyle::MW,
            SMMCourse_GameStyle::WU,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<SMMCourse_GameStyle>("SMMCourse.GameStyle", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for SMMCourse_GameStyle {
}

impl ::std::default::Default for SMMCourse_GameStyle {
    fn default() -> Self {
        SMMCourse_GameStyle::M1
    }
}

impl ::protobuf::reflect::ProtobufValue for SMMCourse_GameStyle {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum SMMCourse_CourseTheme {
    GROUND = 0,
    UNDERGROUND = 1,
    CASTLE = 2,
    AIRSHIP = 3,
    UNDERWATER = 4,
    GHOUST_HOUSE = 5,
}

impl ::protobuf::ProtobufEnum for SMMCourse_CourseTheme {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SMMCourse_CourseTheme> {
        match value {
            0 => ::std::option::Option::Some(SMMCourse_CourseTheme::GROUND),
            1 => ::std::option::Option::Some(SMMCourse_CourseTheme::UNDERGROUND),
            2 => ::std::option::Option::Some(SMMCourse_CourseTheme::CASTLE),
            3 => ::std::option::Option::Some(SMMCourse_CourseTheme::AIRSHIP),
            4 => ::std::option::Option::Some(SMMCourse_CourseTheme::UNDERWATER),
            5 => ::std::option::Option::Some(SMMCourse_CourseTheme::GHOUST_HOUSE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SMMCourse_CourseTheme] = &[
            SMMCourse_CourseTheme::GROUND,
            SMMCourse_CourseTheme::UNDERGROUND,
            SMMCourse_CourseTheme::CASTLE,
            SMMCourse_CourseTheme::AIRSHIP,
            SMMCourse_CourseTheme::UNDERWATER,
            SMMCourse_CourseTheme::GHOUST_HOUSE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<SMMCourse_CourseTheme>("SMMCourse.CourseTheme", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for SMMCourse_CourseTheme {
}

impl ::std::default::Default for SMMCourse_CourseTheme {
    fn default() -> Self {
        SMMCourse_CourseTheme::GROUND
    }
}

impl ::protobuf::reflect::ProtobufValue for SMMCourse_CourseTheme {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum SMMCourse_AutoScroll {
    DISABLED = 0,
    SLOW = 1,
    MEDIUM = 2,
    FAST = 3,
    LOCK = 4,
}

impl ::protobuf::ProtobufEnum for SMMCourse_AutoScroll {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SMMCourse_AutoScroll> {
        match value {
            0 => ::std::option::Option::Some(SMMCourse_AutoScroll::DISABLED),
            1 => ::std::option::Option::Some(SMMCourse_AutoScroll::SLOW),
            2 => ::std::option::Option::Some(SMMCourse_AutoScroll::MEDIUM),
            3 => ::std::option::Option::Some(SMMCourse_AutoScroll::FAST),
            4 => ::std::option::Option::Some(SMMCourse_AutoScroll::LOCK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SMMCourse_AutoScroll] = &[
            SMMCourse_AutoScroll::DISABLED,
            SMMCourse_AutoScroll::SLOW,
            SMMCourse_AutoScroll::MEDIUM,
            SMMCourse_AutoScroll::FAST,
            SMMCourse_AutoScroll::LOCK,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<SMMCourse_AutoScroll>("SMMCourse.AutoScroll", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for SMMCourse_AutoScroll {
}

impl ::std::default::Default for SMMCourse_AutoScroll {
    fn default() -> Self {
        SMMCourse_AutoScroll::DISABLED
    }
}

impl ::protobuf::reflect::ProtobufValue for SMMCourse_AutoScroll {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fSMMCourse.proto\x12\x05smmdb\"\xd0\x07\n\tSMMCourse\x12\x1c\n\x08m\
    odified\x18\x01\x20\x01(\x04R\x08modifiedB\0\x12\x16\n\x05title\x18\x02\
    \x20\x01(\tR\x05titleB\0\x12\x16\n\x05maker\x18\x03\x20\x01(\tR\x05maker\
    B\0\x12;\n\ngame_style\x18\x04\x20\x01(\x0e2\x1a.smmdb.SMMCourse.GameSty\
    leR\tgameStyleB\0\x12A\n\x0ccourse_theme\x18\x05\x20\x01(\x0e2\x1c.smmdb\
    .SMMCourse.CourseThemeR\x0bcourseThemeB\0\x12H\n\x10course_theme_sub\x18\
    \x06\x20\x01(\x0e2\x1c.smmdb.SMMCourse.CourseThemeR\x0ecourseThemeSubB\0\
    \x12\x14\n\x04time\x18\x07\x20\x01(\rR\x04timeB\0\x12>\n\x0bauto_scroll\
    \x18\x08\x20\x01(\x0e2\x1b.smmdb.SMMCourse.AutoScrollR\nautoScrollB\0\
    \x12E\n\x0fauto_scroll_sub\x18\t\x20\x01(\x0e2\x1b.smmdb.SMMCourse.AutoS\
    crollR\rautoScrollSubB\0\x12\x16\n\x05width\x18\n\x20\x01(\rR\x05widthB\
    \0\x12\x1d\n\twidth_sub\x18\x0b\x20\x01(\rR\x08widthSubB\0\x12#\n\x05til\
    es\x18\x0c\x20\x03(\x0b2\x0b.smmdb.TileR\x05tilesB\0\x12*\n\ttiles_sub\
    \x18\r\x20\x03(\x0b2\x0b.smmdb.TileR\x08tilesSubB\0\x12&\n\x06sounds\x18\
    \x0e\x20\x03(\x0b2\x0c.smmdb.SoundR\x06soundsB\0\x12-\n\nsounds_sub\x18\
    \x0f\x20\x03(\x0b2\x0c.smmdb.SoundR\tsoundsSubB\0\x12\x1e\n\tthumbnail\
    \x18\x10\x20\x01(\x0cR\tthumbnailB\0\x12-\n\x11thumbnail_preview\x18\x11\
    \x20\x01(\x0cR\x10thumbnailPreviewB\0\"-\n\tGameStyle\x12\x06\n\x02M1\
    \x10\0\x12\x06\n\x02M3\x10\x01\x12\x06\n\x02MW\x10\x02\x12\x06\n\x02WU\
    \x10\x03\x1a\0\"g\n\x0bCourseTheme\x12\n\n\x06GROUND\x10\0\x12\x0f\n\x0b\
    UNDERGROUND\x10\x01\x12\n\n\x06CASTLE\x10\x02\x12\x0b\n\x07AIRSHIP\x10\
    \x03\x12\x0e\n\nUNDERWATER\x10\x04\x12\x10\n\x0cGHOUST_HOUSE\x10\x05\x1a\
    \0\"F\n\nAutoScroll\x12\x0c\n\x08DISABLED\x10\0\x12\x08\n\x04SLOW\x10\
    \x01\x12\n\n\x06MEDIUM\x10\x02\x12\x08\n\x04FAST\x10\x03\x12\x08\n\x04LO\
    CK\x10\x04\x1a\0:\0B\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
