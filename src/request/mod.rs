use std::io::{BufRead, BufReader, Read, Write};

use crate::Error;
use crate::read_util::{ByteOrder, Encoding, Readable, ReadableRead, Writable, WritableWrite};
use crate::Result;

pub mod create_window;
pub mod change_window_attributes;
pub mod get_window_attributes;
pub mod destroy_window;
pub mod destroy_subwindows;
pub mod change_save_set;
pub mod reparent_window;
pub mod map_window;
pub mod map_subwindows;
pub mod unmap_window;
pub mod unmap_subwindows;
pub mod configure_window;
pub mod circulate_window;
pub mod get_geometry;
pub mod query_tree;
pub mod intern_atom;
pub mod get_atom_name;
pub mod change_property;
pub mod delete_property;
pub mod get_property;
pub mod list_properties;
pub mod set_selection_owner;
pub mod get_selection_owner;
pub mod convert_selection;
pub mod send_event;
pub mod grab_pointer;
pub mod ungrab_pointer;
pub mod grab_button;
pub mod ungrab_button;
pub mod change_active_pointer_grab;
pub mod grab_keyboard;
pub mod ungrab_keyboard;
pub mod grab_key;
pub mod ungrab_key;
pub mod allow_events;
pub mod grab_server;
pub mod ungrab_server;
pub mod query_pointer;
pub mod get_motion_events;
pub mod translate_coordinates;
pub mod warp_pointer;
pub mod set_input_focus;
pub mod get_input_focus;
pub mod query_keymap;
pub mod open_font;
pub mod close_font;
pub mod query_font;
pub mod query_text_extents;
pub mod list_fonts;
pub mod list_fonts_with_info;
pub mod set_font_path;
pub mod get_font_path;
pub mod create_pixmap;
pub mod free_pixmap;
pub mod create_gc;
pub mod change_gc;
pub mod copy_gc;
pub mod set_dashes;
pub mod set_clip_rectangles;
pub mod free_gc;
pub mod clear_area;
pub mod copy_area;
pub mod copy_plane;
pub mod poly_point;
pub mod poly_line;
pub mod poly_segment;
pub mod poly_rectangle;
pub mod poly_arc;
pub mod fill_poly;
pub mod poly_fill_rectangle;
pub mod poly_fill_arc;
pub mod put_image;
pub mod get_image;
pub mod poly_text8;
pub mod poly_text16;
pub mod image_text8;
pub mod image_text16;
pub mod create_colormap;
pub mod free_colormap;
pub mod copy_colormap_and_free;
pub mod install_colormap;
pub mod uninstall_colormap;
pub mod list_installed_colormaps;
pub mod alloc_color;
pub mod alloc_named_color;
pub mod alloc_color_cells;
pub mod alloc_color_planes;
pub mod free_colors;
pub mod store_colors;
pub mod store_named_color;
pub mod query_colors;
pub mod lookup_color;
pub mod create_cursor;
pub mod create_glyph_cursor;
pub mod free_cursor;
pub mod recolor_cursor;
pub mod query_best_size;
pub mod query_extension;
pub mod list_extensions;
pub mod change_keyboard_mapping;
pub mod get_keyboard_mapping;
pub mod change_keyboard_control;
pub mod get_keyboard_control;
pub mod bell;
pub mod change_pointer_control;
pub mod get_pointer_control;
pub mod set_screen_saver;
pub mod get_screen_saver;
pub mod change_hosts;
pub mod list_hosts;
pub mod set_access_control;
pub mod set_close_down_mode;
pub mod kill_client;
pub mod rotate_properties;
pub mod force_screen_saver;
pub mod set_pointer_mapping;
pub mod get_pointer_mapping;
pub mod set_modifier_mapping;
pub mod get_modifier_mapping;
pub mod no_operation;


pub enum Request {
    CreateWindow(create_window::CreateWindowRequest),
    ChangeWindowAttributes(change_window_attributes::ChangeWindowAttributesRequest),
    GetWindowAttributes(get_window_attributes::GetWindowAttributesRequest),
    DestroyWindow(destroy_window::DestroyWindowRequest),
    DestroySubwindows(destroy_subwindows::DestroySubwindowsRequest),
    ChangeSaveSet(change_save_set::ChangeSaveSetRequest),
    ReparentWindow(reparent_window::ReparentWindowRequest),
    MapWindow(map_window::MapWindowRequest),
    MapSubwindows(map_subwindows::MapSubwindowsRequest),
    UnmapWindow(unmap_window::UnmapWindowRequest),
    UnmapSubwindows(unmap_subwindows::UnmapSubwindowsRequest),
    ConfigureWindow(configure_window::ConfigureWindowRequest),
    CirculateWindow(circulate_window::CirculateWindowRequest),
    GetGeometry(get_geometry::GetGeometryRequest),
    QueryTree(query_tree::QueryTreeRequest),
    InternAtom(intern_atom::InternAtomRequest),
    GetAtomName(get_atom_name::GetAtomNameRequest),
    ChangeProperty(change_property::ChangePropertyRequest),
    DeleteProperty(delete_property::DeletePropertyRequest),
    GetProperty(get_property::GetPropertyRequest),
    ListProperties(list_properties::ListPropertiesRequest),
    SetSelectionOwner(set_selection_owner::SetSelectionOwnerRequest),
    GetSelectionOwner(get_selection_owner::GetSelectionOwnerRequest),
    ConvertSelection(convert_selection::ConvertSelectionRequest),
    SendEvent(send_event::SendEventRequest),
    GrabPointer(grab_pointer::GrabPointerRequest),
    UngrabPointer(ungrab_pointer::UngrabPointerRequest),
    GrabButton(grab_button::GrabButtonRequest),
    UngrabButton(ungrab_button::UngrabButtonRequest),
    ChangeActivePointerGrab(change_active_pointer_grab::ChangeActivePointerGrabRequest),
    GrabKeyboard(grab_keyboard::GrabKeyboardRequest),
    UngrabKeyboard(ungrab_keyboard::UngrabKeyboardRequest),
    GrabKey(grab_key::GrabKeyRequest),
    UngrabKey(ungrab_key::UngrabKeyRequest),
    AllowEvents(allow_events::AllowEventsRequest),
    GrabServer(grab_server::GrabServerRequest),
    UngrabServer(ungrab_server::UngrabServerRequest),
    QueryPointer(query_pointer::QueryPointerRequest),
    GetMotionEvents(get_motion_events::GetMotionEventsRequest),
    TranslateCoordinates(translate_coordinates::TranslateCoordinatesRequest),
    WarpPointer(warp_pointer::WarpPointerRequest),
    SetInputFocus(set_input_focus::SetInputFocusRequest),
    GetInputFocus(get_input_focus::GetInputFocusRequest),
    QueryKeymap(query_keymap::QueryKeymapRequest),
    OpenFont(open_font::OpenFontRequest),
    CloseFont(close_font::CloseFontRequest),
    QueryFont(query_font::QueryFontRequest),
    QueryTextExtents(query_text_extents::QueryTextExtentsRequest),
    ListFonts(list_fonts::ListFontsRequest),
    ListFontsWithInfo(list_fonts_with_info::ListFontsWithInfoRequest),
    SetFontPath(set_font_path::SetFontPathRequest),
    GetFontPath(get_font_path::GetFontPathRequest),
    CreatePixmap(create_pixmap::CreatePixmapRequest),
    FreePixmap(free_pixmap::FreePixmapRequest),
    CreateGC(create_gc::CreateGCRequest),
    ChangeGC(change_gc::ChangeGCRequest),
    CopyGC(copy_gc::CopyGCRequest),
    SetDashes(set_dashes::SetDashesRequest),
    SetClipRectangles(set_clip_rectangles::SetClipRectanglesRequest),
    FreeGC(free_gc::FreeGCRequest),
    ClearArea(clear_area::ClearAreaRequest),
    CopyArea(copy_area::CopyAreaRequest),
    CopyPlane(copy_plane::CopyPlaneRequest),
    PolyPoint(poly_point::PolyPointRequest),
    PolyLine(poly_line::PolyLineRequest),
    PolySegment(poly_segment::PolySegmentRequest),
    PolyRectangle(poly_rectangle::PolyRectangleRequest),
    PolyArc(poly_arc::PolyArcRequest),
    FillPoly(fill_poly::FillPolyRequest),
    PolyFillRectangle(poly_fill_rectangle::PolyFillRectangleRequest),
    PolyFillArc(poly_fill_arc::PolyFillArcRequest),
    PutImage(put_image::PutImageRequest),
    GetImage(get_image::GetImageRequest),
    PolyText8(poly_text8::PolyText8Request),
    PolyText16(poly_text16::PolyText16Request),
    ImageText8(image_text8::ImageText8Request),
    ImageText16(image_text16::ImageText16Request),
    CreateColormap(create_colormap::CreateColormapRequest),
    FreeColormap(free_colormap::FreeColormapRequest),
    CopyColormapAndFree(copy_colormap_and_free::CopyColormapAndFreeRequest),
    InstallColormap(install_colormap::InstallColormapRequest),
    UninstallColormap(uninstall_colormap::UninstallColormapRequest),
    ListInstalledColormaps(list_installed_colormaps::ListInstalledColormapsRequest),
    AllocColor(alloc_color::AllocColorRequest),
    AllocNamedColor(alloc_named_color::AllocNamedColorRequest),
    AllocColorCells(alloc_color_cells::AllocColorCellsRequest),
    AllocColorPlanes(alloc_color_planes::AllocColorPlanesRequest),
    FreeColors(free_colors::FreeColorsRequest),
    StoreColors(store_colors::StoreColorsRequest),
    StoreNamedColor(store_named_color::StoreNamedColorRequest),
    QueryColors(query_colors::QueryColorsRequest),
    LookupColor(lookup_color::LookupColorRequest),
    CreateCursor(create_cursor::CreateCursorRequest),
    CreateGlyphCursor(create_glyph_cursor::CreateGlyphCursorRequest),
    FreeCursor(free_cursor::FreeCursorRequest),
    RecolorCursor(recolor_cursor::RecolorCursorRequest),
    QueryBestSize(query_best_size::QueryBestSizeRequest),
    QueryExtension(query_extension::QueryExtensionRequest),
    ListExtensions(list_extensions::ListExtensionsRequest),
    ChangeKeyboardMapping(change_keyboard_mapping::ChangeKeyboardMappingRequest),
    GetKeyboardMapping(get_keyboard_mapping::GetKeyboardMappingRequest),
    ChangeKeyboardControl(change_keyboard_control::ChangeKeyboardControlRequest),
    GetKeyboardControl(get_keyboard_control::GetKeyboardControlRequest),
    Bell(bell::BellRequest),
    ChangePointerControl(change_pointer_control::ChangePointerControlRequest),
    GetPointerControl(get_pointer_control::GetPointerControlRequest),
    SetScreenSaver(set_screen_saver::SetScreenSaverRequest),
    GetScreenSaver(get_screen_saver::GetScreenSaverRequest),
    ChangeHosts(change_hosts::ChangeHostsRequest),
    ListHosts(list_hosts::ListHostsRequest),
    SetAccessControl(set_access_control::SetAccessControlRequest),
    SetCloseDownMode(set_close_down_mode::SetCloseDownModeRequest),
    KillClient(kill_client::KillClientRequest),
    RotateProperties(rotate_properties::RotatePropertiesRequest),
    ForceScreenSaver(force_screen_saver::ForceScreenSaverRequest),
    SetPointerMapping(set_pointer_mapping::SetPointerMappingRequest),
    GetPointerMapping(get_pointer_mapping::GetPointerMappingRequest),
    SetModifierMapping(set_modifier_mapping::SetModifierMappingRequest),
    GetModifierMapping(get_modifier_mapping::GetModifierMappingRequest),
    NoOperation(no_operation::NoOperationRequest),
}

impl Readable for Request {
    fn read(stream: &mut std::io::BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        match stream.read_value::<u8>(order)? {
            1 => Ok(Request::CreateWindow(stream.read_value(order)?)),
            2 => Ok(Request::ChangeWindowAttributes(stream.read_value(order)?)),
            3 => Ok(Request::GetWindowAttributes(stream.read_value(order)?)),
            4 => Ok(Request::DestroyWindow(stream.read_value(order)?)),
            5 => Ok(Request::DestroySubwindows(stream.read_value(order)?)),
            6 => Ok(Request::ChangeSaveSet(stream.read_value(order)?)),
            7 => Ok(Request::ReparentWindow(stream.read_value(order)?)),
            8 => Ok(Request::MapWindow(stream.read_value(order)?)),
            9 => Ok(Request::MapSubwindows(stream.read_value(order)?)),
            10 => Ok(Request::UnmapWindow(stream.read_value(order)?)),
            11 => Ok(Request::UnmapSubwindows(stream.read_value(order)?)),
            12 => Ok(Request::ConfigureWindow(stream.read_value(order)?)),
            13 => Ok(Request::CirculateWindow(stream.read_value(order)?)),
            14 => Ok(Request::GetGeometry(stream.read_value(order)?)),
            15 => Ok(Request::QueryTree(stream.read_value(order)?)),
            16 => Ok(Request::InternAtom(stream.read_value(order)?)),
            17 => Ok(Request::GetAtomName(stream.read_value(order)?)),
            18 => Ok(Request::ChangeProperty(stream.read_value(order)?)),
            19 => Ok(Request::DeleteProperty(stream.read_value(order)?)),
            20 => Ok(Request::GetProperty(stream.read_value(order)?)),
            21 => Ok(Request::ListProperties(stream.read_value(order)?)),
            22 => Ok(Request::SetSelectionOwner(stream.read_value(order)?)),
            23 => Ok(Request::GetSelectionOwner(stream.read_value(order)?)),
            24 => Ok(Request::ConvertSelection(stream.read_value(order)?)),
            25 => Ok(Request::SendEvent(stream.read_value(order)?)),
            26 => Ok(Request::GrabPointer(stream.read_value(order)?)),
            27 => Ok(Request::UngrabPointer(stream.read_value(order)?)),
            28 => Ok(Request::GrabButton(stream.read_value(order)?)),
            29 => Ok(Request::UngrabButton(stream.read_value(order)?)),
            30 => Ok(Request::ChangeActivePointerGrab(stream.read_value(order)?)),
            31 => Ok(Request::GrabKeyboard(stream.read_value(order)?)),
            32 => Ok(Request::UngrabKeyboard(stream.read_value(order)?)),
            33 => Ok(Request::GrabKey(stream.read_value(order)?)),
            34 => Ok(Request::UngrabKey(stream.read_value(order)?)),
            35 => Ok(Request::AllowEvents(stream.read_value(order)?)),
            36 => Ok(Request::GrabServer(stream.read_value(order)?)),
            37 => Ok(Request::UngrabServer(stream.read_value(order)?)),
            38 => Ok(Request::QueryPointer(stream.read_value(order)?)),
            39 => Ok(Request::GetMotionEvents(stream.read_value(order)?)),
            40 => Ok(Request::TranslateCoordinates(stream.read_value(order)?)),
            41 => Ok(Request::WarpPointer(stream.read_value(order)?)),
            42 => Ok(Request::SetInputFocus(stream.read_value(order)?)),
            43 => Ok(Request::GetInputFocus(stream.read_value(order)?)),
            44 => Ok(Request::QueryKeymap(stream.read_value(order)?)),
            45 => Ok(Request::OpenFont(stream.read_value(order)?)),
            46 => Ok(Request::CloseFont(stream.read_value(order)?)),
            47 => Ok(Request::QueryFont(stream.read_value(order)?)),
            48 => Ok(Request::QueryTextExtents(stream.read_value(order)?)),
            49 => Ok(Request::ListFonts(stream.read_value(order)?)),
            50 => Ok(Request::ListFontsWithInfo(stream.read_value(order)?)),
            51 => Ok(Request::SetFontPath(stream.read_value(order)?)),
            52 => Ok(Request::GetFontPath(stream.read_value(order)?)),
            53 => Ok(Request::CreatePixmap(stream.read_value(order)?)),
            54 => Ok(Request::FreePixmap(stream.read_value(order)?)),
            55 => Ok(Request::CreateGC(stream.read_value(order)?)),
            56 => Ok(Request::ChangeGC(stream.read_value(order)?)),
            57 => Ok(Request::CopyGC(stream.read_value(order)?)),
            58 => Ok(Request::SetDashes(stream.read_value(order)?)),
            59 => Ok(Request::SetClipRectangles(stream.read_value(order)?)),
            60 => Ok(Request::FreeGC(stream.read_value(order)?)),
            61 => Ok(Request::ClearArea(stream.read_value(order)?)),
            62 => Ok(Request::CopyArea(stream.read_value(order)?)),
            63 => Ok(Request::CopyPlane(stream.read_value(order)?)),
            64 => Ok(Request::PolyPoint(stream.read_value(order)?)),
            65 => Ok(Request::PolyLine(stream.read_value(order)?)),
            66 => Ok(Request::PolySegment(stream.read_value(order)?)),
            67 => Ok(Request::PolyRectangle(stream.read_value(order)?)),
            68 => Ok(Request::PolyArc(stream.read_value(order)?)),
            69 => Ok(Request::FillPoly(stream.read_value(order)?)),
            70 => Ok(Request::PolyFillRectangle(stream.read_value(order)?)),
            71 => Ok(Request::PolyFillArc(stream.read_value(order)?)),
            72 => Ok(Request::PutImage(stream.read_value(order)?)),
            73 => Ok(Request::GetImage(stream.read_value(order)?)),
            74 => Ok(Request::PolyText8(stream.read_value(order)?)),
            75 => Ok(Request::PolyText16(stream.read_value(order)?)),
            76 => Ok(Request::ImageText8(stream.read_value(order)?)),
            77 => Ok(Request::ImageText16(stream.read_value(order)?)),
            78 => Ok(Request::CreateColormap(stream.read_value(order)?)),
            79 => Ok(Request::FreeColormap(stream.read_value(order)?)),
            80 => Ok(Request::CopyColormapAndFree(stream.read_value(order)?)),
            81 => Ok(Request::InstallColormap(stream.read_value(order)?)),
            82 => Ok(Request::UninstallColormap(stream.read_value(order)?)),
            83 => Ok(Request::ListInstalledColormaps(stream.read_value(order)?)),
            84 => Ok(Request::AllocColor(stream.read_value(order)?)),
            85 => Ok(Request::AllocNamedColor(stream.read_value(order)?)),
            86 => Ok(Request::AllocColorCells(stream.read_value(order)?)),
            87 => Ok(Request::AllocColorPlanes(stream.read_value(order)?)),
            88 => Ok(Request::FreeColors(stream.read_value(order)?)),
            89 => Ok(Request::StoreColors(stream.read_value(order)?)),
            90 => Ok(Request::StoreNamedColor(stream.read_value(order)?)),
            91 => Ok(Request::QueryColors(stream.read_value(order)?)),
            92 => Ok(Request::LookupColor(stream.read_value(order)?)),
            93 => Ok(Request::CreateCursor(stream.read_value(order)?)),
            94 => Ok(Request::CreateGlyphCursor(stream.read_value(order)?)),
            95 => Ok(Request::FreeCursor(stream.read_value(order)?)),
            96 => Ok(Request::RecolorCursor(stream.read_value(order)?)),
            97 => Ok(Request::QueryBestSize(stream.read_value(order)?)),
            98 => Ok(Request::QueryExtension(stream.read_value(order)?)),
            99 => Ok(Request::ListExtensions(stream.read_value(order)?)),
            100 => Ok(Request::ChangeKeyboardMapping(stream.read_value(order)?)),
            101 => Ok(Request::GetKeyboardMapping(stream.read_value(order)?)),
            102 => Ok(Request::ChangeKeyboardControl(stream.read_value(order)?)),
            103 => Ok(Request::GetKeyboardControl(stream.read_value(order)?)),
            104 => Ok(Request::Bell(stream.read_value(order)?)),
            105 => Ok(Request::ChangePointerControl(stream.read_value(order)?)),
            106 => Ok(Request::GetPointerControl(stream.read_value(order)?)),
            107 => Ok(Request::SetScreenSaver(stream.read_value(order)?)),
            108 => Ok(Request::GetScreenSaver(stream.read_value(order)?)),
            109 => Ok(Request::ChangeHosts(stream.read_value(order)?)),
            110 => Ok(Request::ListHosts(stream.read_value(order)?)),
            111 => Ok(Request::SetAccessControl(stream.read_value(order)?)),
            112 => Ok(Request::SetCloseDownMode(stream.read_value(order)?)),
            113 => Ok(Request::KillClient(stream.read_value(order)?)),
            114 => Ok(Request::RotateProperties(stream.read_value(order)?)),
            115 => Ok(Request::ForceScreenSaver(stream.read_value(order)?)),
            116 => Ok(Request::SetPointerMapping(stream.read_value(order)?)),
            117 => Ok(Request::GetPointerMapping(stream.read_value(order)?)),
            118 => Ok(Request::SetModifierMapping(stream.read_value(order)?)),
            119 => Ok(Request::GetModifierMapping(stream.read_value(order)?)),
            127 => Ok(Request::NoOperation(stream.read_value(order)?)),
            _ => Err(Error::InvalidValue("opcode")),
        }
    }
}

impl Writable for Request {
    fn write(stream: &mut std::io::BufWriter<impl Write>, data: Self, order: &ByteOrder) -> Result<()> {
        match data {
            Request::CreateWindow(data) => stream.write_value(data, order),
            Request::ChangeWindowAttributes(data) => stream.write_value(data, order),
            Request::GetWindowAttributes(data) => stream.write_value(data, order),
            Request::DestroyWindow(data) => stream.write_value(data, order),
            Request::DestroySubwindows(data) => stream.write_value(data, order),
            Request::ChangeSaveSet(data) => stream.write_value(data, order),
            Request::ReparentWindow(data) => stream.write_value(data, order),
            Request::MapWindow(data) => stream.write_value(data, order),
            Request::MapSubwindows(data) => stream.write_value(data, order),
            Request::UnmapWindow(data) => stream.write_value(data, order),
            Request::UnmapSubwindows(data) => stream.write_value(data, order),
            Request::ConfigureWindow(data) => stream.write_value(data, order),
            Request::CirculateWindow(data) => stream.write_value(data, order),
            Request::GetGeometry(data) => stream.write_value(data, order),
            Request::QueryTree(data) => stream.write_value(data, order),
            Request::InternAtom(data) => stream.write_value(data, order),
            Request::GetAtomName(data) => stream.write_value(data, order),
            Request::ChangeProperty(data) => stream.write_value(data, order),
            Request::DeleteProperty(data) => stream.write_value(data, order),
            Request::GetProperty(data) => stream.write_value(data, order),
            Request::RotateProperties(data) => stream.write_value(data, order),
            Request::ListProperties(data) => stream.write_value(data, order),
            Request::SetSelectionOwner(data) => stream.write_value(data, order),
            Request::GetSelectionOwner(data) => stream.write_value(data, order),
            Request::ConvertSelection(data) => stream.write_value(data, order),
            Request::SendEvent(data) => stream.write_value(data, order),
            Request::GrabPointer(data) => stream.write_value(data, order),
            Request::UngrabPointer(data) => stream.write_value(data, order),
            Request::GrabButton(data) => stream.write_value(data, order),
            Request::UngrabButton(data) => stream.write_value(data, order),
            Request::ChangeActivePointerGrab(data) => stream.write_value(data, order),
            Request::GrabKeyboard(data) => stream.write_value(data, order),
            Request::UngrabKeyboard(data) => stream.write_value(data, order),
            Request::GrabKey(data) => stream.write_value(data, order),
            Request::UngrabKey(data) => stream.write_value(data, order),
            Request::AllowEvents(data) => stream.write_value(data, order),
            Request::GrabServer(data) => stream.write_value(data, order),
            Request::UngrabServer(data) => stream.write_value(data, order),
            Request::QueryPointer(data) => stream.write_value(data, order),
            Request::GetMotionEvents(data) => stream.write_value(data, order),
            Request::TranslateCoordinates(data) => stream.write_value(data, order),
            Request::WarpPointer(data) => stream.write_value(data, order),
            Request::SetInputFocus(data) => stream.write_value(data, order),
            Request::GetInputFocus(data) => stream.write_value(data, order),
            Request::QueryKeymap(data) => stream.write_value(data, order),
            Request::OpenFont(data) => stream.write_value(data, order),
            Request::CloseFont(data) => stream.write_value(data, order),
            Request::QueryFont(data) => stream.write_value(data, order),
            Request::QueryTextExtents(data) => stream.write_value(data, order),
            Request::ListFonts(data) => stream.write_value(data, order),
            Request::ListFontsWithInfo(data) => stream.write_value(data, order),
            Request::SetFontPath(data) => stream.write_value(data, order),
            Request::GetFontPath(data) => stream.write_value(data, order),
            Request::CreatePixmap(data) => stream.write_value(data, order),
            Request::FreePixmap(data) => stream.write_value(data, order),
            Request::CreateGC(data) => stream.write_value(data, order),
            Request::ChangeGC(data) => stream.write_value(data, order),
            Request::CopyGC(data) => stream.write_value(data, order),
            Request::SetDashes(data) => stream.write_value(data, order),
            Request::SetClipRectangles(data) => stream.write_value(data, order),
            Request::FreeGC(data) => stream.write_value(data, order),
            Request::ClearArea(data) => stream.write_value(data, order),
            Request::CopyArea(data) => stream.write_value(data, order),
            Request::CopyPlane(data) => stream.write_value(data, order),
            Request::PolyPoint(data) => stream.write_value(data, order),
            Request::PolyLine(data) => stream.write_value(data, order),
            Request::PolySegment(data) => stream.write_value(data, order),
            Request::PolyRectangle(data) => stream.write_value(data, order),
            Request::PolyArc(data) => stream.write_value(data, order),
            Request::FillPoly(data) => stream.write_value(data, order),
            Request::PolyFillRectangle(data) => stream.write_value(data, order),
            Request::PolyFillArc(data) => stream.write_value(data, order),
            Request::PutImage(data) => stream.write_value(data, order),
            Request::GetImage(data) => stream.write_value(data, order),
            Request::PolyText8(data) => stream.write_value(data, order),
            Request::PolyText16(data) => stream.write_value(data, order),
            Request::ImageText8(data) => stream.write_value(data, order),
            Request::ImageText16(data) => stream.write_value(data, order),
            Request::CreateColormap(data) => stream.write_value(data, order),
            Request::FreeColormap(data) => stream.write_value(data, order),
            Request::CopyColormapAndFree(data) => stream.write_value(data, order),
            Request::InstallColormap(data) => stream.write_value(data, order),
            Request::UninstallColormap(data) => stream.write_value(data, order),
            Request::ListInstalledColormaps(data) => stream.write_value(data, order),
            Request::AllocColor(data) => stream.write_value(data, order),
            Request::AllocNamedColor(data) => stream.write_value(data, order),
            Request::AllocColorCells(data) => stream.write_value(data, order),
            Request::AllocColorPlanes(data) => stream.write_value(data, order),
            Request::FreeColors(data) => stream.write_value(data, order),
            Request::StoreColors(data) => stream.write_value(data, order),
            Request::StoreNamedColor(data) => stream.write_value(data, order),
            Request::QueryColors(data) => stream.write_value(data, order),
            Request::LookupColor(data) => stream.write_value(data, order),
            Request::CreateCursor(data) => stream.write_value(data, order),
            Request::CreateGlyphCursor(data) => stream.write_value(data, order),
            Request::FreeCursor(data) => stream.write_value(data, order),
            Request::RecolorCursor(data) => stream.write_value(data, order),
            Request::QueryBestSize(data) => stream.write_value(data, order),
            Request::QueryExtension(data) => stream.write_value(data, order),
            Request::ListExtensions(data) => stream.write_value(data, order),
            Request::SetModifierMapping(data) => stream.write_value(data, order),
            Request::GetModifierMapping(data) => stream.write_value(data, order),
            Request::ChangeKeyboardMapping(data) => stream.write_value(data, order),
            Request::GetKeyboardMapping(data) => stream.write_value(data, order),
            Request::ChangeKeyboardControl(data) => stream.write_value(data, order),
            Request::GetKeyboardControl(data) => stream.write_value(data, order),
            Request::Bell(data) => stream.write_value(data, order),
            Request::SetPointerMapping(data) => stream.write_value(data, order),
            Request::GetPointerMapping(data) => stream.write_value(data, order),
            Request::ChangePointerControl(data) => stream.write_value(data, order),
            Request::GetPointerControl(data) => stream.write_value(data, order),
            Request::SetScreenSaver(data) => stream.write_value(data, order),
            Request::GetScreenSaver(data) => stream.write_value(data, order),
            Request::ForceScreenSaver(data) => stream.write_value(data, order),
            Request::ChangeHosts(data) => stream.write_value(data, order),
            Request::ListHosts(data) => stream.write_value(data, order),
            Request::SetAccessControl(data) => stream.write_value(data, order),
            Request::SetCloseDownMode(data) => stream.write_value(data, order),
            Request::KillClient(data) => stream.write_value(data, order),
            Request::NoOperation(data) => stream.write_value(data, order),
        }
    }
}

struct Response {
    pub sequence_number: u16,
    pub data: Vec<u8>,
}

impl Readable for Response {
    fn read(stream: &mut BufReader<impl Read>, order: &ByteOrder) -> Result<Self> {
        while stream.fill_buf().map_err(|e| Error::IoError(e))?.len() < 32 {}
        let sequence_number = u16::decode(order, &stream.buffer()[2..4]);
        let additional_length = u16::decode(order, &stream.buffer()[4..6]) as usize;
        while stream.fill_buf().map_err(|e| Error::IoError(e))?.len() < 32 + (additional_length << 2) {}
        let data = Vec::from(&stream.buffer()[..32 + (additional_length << 2)]);
        stream.consume(32 + (additional_length << 2));
        Ok(Response { sequence_number, data })
    }
}