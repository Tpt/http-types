use super::ParamKind;
use crate::Mime;

/// Content-Type that matches anything.
///
/// # Mime Type
///
/// ```txt
/// */*
/// ```
pub const ANY: Mime = Mime {
    essence: String::new(),
    basetype: String::new(),
    subtype: String::new(),
    params: None,
    static_essence: Some("*/*"),
    static_basetype: Some("*"),
    static_subtype: Some("*"),
};

/// Content-Type for JavaScript.
///
/// # Mime Type
///
/// ```txt
/// application/javascript; charset=utf-8
/// ```
pub const JAVASCRIPT: Mime = Mime {
    static_essence: Some("application/javascript"),
    essence: String::new(),
    basetype: String::new(),
    subtype: String::new(),
    params: Some(ParamKind::Utf8),
    static_basetype: Some("application"),
    static_subtype: Some("javascript"),
};

/// Content-Type for JSON.
///
/// # Mime Type
///
/// ```txt
/// application/json
/// ```
pub const JSON: Mime = Mime {
    static_essence: Some("application/json"),
    essence: String::new(),
    basetype: String::new(),
    subtype: String::new(),
    params: None,
    static_basetype: Some("application"),
    static_subtype: Some("json"),
};

/// Content-Type for CSS.
///
/// # Mime Type
///
/// ```txt
/// text/css; charset=utf-8
/// ```
pub const CSS: Mime = Mime {
    static_essence: Some("text/css"),
    essence: String::new(),
    basetype: String::new(),
    subtype: String::new(),
    params: Some(ParamKind::Utf8),
    static_basetype: Some("text"),
    static_subtype: Some("css"),
};

/// Content-Type for HTML.
///
/// # Mime Type
///
/// ```txt
/// text/html; charset=utf-8
/// ```
pub const HTML: Mime = Mime {
    static_essence: Some("text/html"),
    essence: String::new(),
    basetype: String::new(),
    subtype: String::new(),
    params: Some(ParamKind::Utf8),
    static_basetype: Some("text"),
    static_subtype: Some("html"),
};

/// Content-Type for Server Sent Events
///
/// # Mime Type
///
/// ```txt
/// text/event-stream
/// ```
pub const SSE: Mime = Mime {
    static_essence: Some("text/event-stream"),
    essence: String::new(),
    basetype: String::new(),
    subtype: String::new(),
    static_basetype: Some("text"),
    static_subtype: Some("event-stream"),
    params: None,
};

/// Content-Type for plain text.
///
/// # Mime Type
///
/// ```txt
/// text/plain; charset=utf-8
/// ```
pub const PLAIN: Mime = Mime {
    static_essence: Some("text/plain"),
    essence: String::new(),
    basetype: String::new(),
    subtype: String::new(),
    params: Some(ParamKind::Utf8),
    static_basetype: Some("text"),
    static_subtype: Some("plain"),
};

/// Content-Type for byte streams.
///
/// # Mime Type
///
/// ```txt
/// application/octet-stream
/// ```
pub const BYTE_STREAM: Mime = Mime {
    static_essence: Some("application/octet-stream"),
    essence: String::new(),
    basetype: String::new(),
    subtype: String::new(),
    static_basetype: Some("application"),
    static_subtype: Some("octet-stream"),
    params: None,
};

/// Content-Type for form.
///
/// # Mime Type
///
/// ```txt
/// application/x-www-form-urlencoded
/// ```
pub const FORM: Mime = Mime {
    static_essence: Some("application/x-www-form-urlencoded"),
    essence: String::new(),
    basetype: String::new(),
    subtype: String::new(),
    static_basetype: Some("application"),
    static_subtype: Some("x-www-form-urlencoded"),
    params: None,
};

/// Content-Type for a multipart form.
///
/// # Mime Type
///
/// ```txt
/// multipart/form-data
/// ```
pub const MULTIPART_FORM: Mime = Mime {
    static_essence: Some("multipart/form-data"),
    essence: String::new(),
    basetype: String::new(),
    subtype: String::new(),
    static_basetype: Some("multipart"),
    static_subtype: Some("form-data"),
    params: None,
};
