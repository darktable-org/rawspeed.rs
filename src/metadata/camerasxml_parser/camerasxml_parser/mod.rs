use rawspeed_metadata_xmlparser::xmlparser;

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Int {
    val: i32,
}

impl core::ops::Deref for Int {
    type Target = i32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for Int {
    #[inline]
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let val = input.parse::<xmlparser::ElementAttributeValue<'a>>()?;
        match val.parse() {
            Ok(val) => Ok(Self { val }),
            Err(_) => Err(format!("Unable to parse `{val:?}` as an integer")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Str<'a> {
    val: &'a str,
}

impl<'a> core::ops::Deref for Str<'a> {
    type Target = &'a str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for Str<'a> {
    #[inline]
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        Ok(Self {
            val: &input.parse::<xmlparser::ElementAttributeValue<'a>>()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct BodyStr<'a> {
    val: &'a str,
}

impl<'a> core::ops::Deref for BodyStr<'a> {
    type Target = &'a str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for BodyStr<'a> {
    #[inline]
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        Ok(Self {
            val: &input.parse::<xmlparser::ElementContentVerbatim<'a>>()?,
        })
    }
}

macro_rules! impl_attr_matcher {
    (
        #[derive(
            $(
                $trait:ident
            ),+
        )]
        struct $struct_ident:ident $(<$struct_lifetime:lifetime>)? {
            $attr_ident:ident: $val_type_ident:path,
        }
    ) => {
        #[derive(
            $(
                $trait
            ),+
        )]
        #[allow(clippy::upper_case_acronyms, clippy::allow_attributes)]
        #[non_exhaustive]
        pub struct $struct_ident $(<$struct_lifetime>)? {
            pub val: $val_type_ident,
        }

        impl$(<$struct_lifetime>)? core::ops::Deref for $struct_ident $(<$struct_lifetime>)? {
            type Target = $val_type_ident;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.val
            }
        }

        impl<'a, 'b> xmlparser::Parse<'a, 'b> for $struct_ident $(<$struct_lifetime>)? {
            #[inline]
            fn parse(
                input: &'b mut xmlparser::ParseStream<'a>,
            ) -> xmlparser::Result<Self> {
                const EXPECTED_NAME: &str = stringify!($attr_ident);
                let name = *input.parse::<xmlparser::ElementAttributeName<'a>>()?;
                if name != EXPECTED_NAME {
                    return Err(format!(
                        "Error while parsing attribute, expected `{EXPECTED_NAME:?}`, but instead found: `{name:?}`"
                    ));
                }
                input.parse::<xmlparser::ElementAttributeEq<'a>>()?;
                Ok(Self {
                    val: input.parse()?,
                })
            }
        }
    }
}

macro_rules! _impl_elt_matcher {
    (
        $struct_ident:ident $(<$struct_lifetime:lifetime>)?,
        $(
            $attr_ident:ident
        ),+
        $(,)?
    ) => {
        impl<'a, 'b> xmlparser::Parse<'a, 'b> for $struct_ident $(<$struct_lifetime>)? {
            #[inline]
            fn parse(
                input: &'b mut xmlparser::ParseStream<'a>,
            ) -> xmlparser::Result<Self> {
                const EXPECTED_NAME: &str = stringify!($struct_ident);
                input.parse::<xmlparser::Lt<'a>>()?;
                let name = *input.parse::<xmlparser::ElementName<'a>>()?;
                if name != EXPECTED_NAME {
                    return Err(format!(
                        "Error while parsing element, expected `{EXPECTED_NAME:?}`, but instead found: `{name:?}`"
                    ));
                }
                let res = Self {
                    $(
                        $attr_ident: input.parse()?
                    ),+
                };
                input.parse::<xmlparser::ElementSlash<'a>>()?;
                input.parse::<xmlparser::Gt<'a>>()?;
                Ok(res)
            }
        }
    };
}

macro_rules! _impl_elt_with_body_matcher {
    (
        $struct_ident:ident $(<$struct_lifetime:lifetime>)?,
        $(
            $attr_ident:ident,
        )*
        ; $body_ident:ident
    ) => {
        impl<'a, 'b> xmlparser::Parse<'a, 'b> for $struct_ident $(<$struct_lifetime>)? {
            #[inline]
            fn parse(
                input: &'b mut xmlparser::ParseStream<'a>,
            ) -> xmlparser::Result<Self> {
                const EXPECTED_NAME: &str = stringify!($struct_ident);
                input.parse::<xmlparser::Lt<'a>>()?;
                let name = *input.parse::<xmlparser::ElementName<'a>>()?;
                if name != EXPECTED_NAME {
                    return Err(format!(
                        "Error while parsing element, expected `{EXPECTED_NAME:?}`, but instead found: `{name:?}`"
                    ));
                }
                $(
                    let $attr_ident = input.parse()?;
                )*
                input.parse::<xmlparser::Gt<'a>>()?;
                let $body_ident = input.parse()?;
                input.parse::<xmlparser::Lt<'a>>()?;
                input.parse::<xmlparser::ElementSlash<'a>>()?;
                let name = *input.parse::<xmlparser::ElementName<'a>>()?;
                if name != EXPECTED_NAME {
                    return Err(format!(
                        "Error while parsing element, expected `{EXPECTED_NAME:?}`, but instead found: `{name:?}`"
                    ));
                }
                input.parse::<xmlparser::Gt<'a>>()?;
                Ok(Self {
                    $(
                        $attr_ident: $attr_ident,
                    )*
                    $body_ident: $body_ident,
                })
            }
        }
    };
}

macro_rules! impl_elt_matcher {
    (
        #[derive(
            $(
                $trait:ident
            ),+
        )]
        struct $struct_ident:ident $(<$struct_lifetime:lifetime>)? {
            $attr0_ident:ident: $val0_type_ident:path,
            $attr1_ident:ident: $val1_type_ident:path,
        }
    ) => {
        #[derive(
            $(
                $trait
            ),+
        )]
        #[non_exhaustive]
        pub struct $struct_ident $(<$struct_lifetime>)? {
            pub $attr0_ident: $val0_type_ident,
            pub $attr1_ident: $val1_type_ident,
        }
        _impl_elt_matcher!(
            $struct_ident $(<$struct_lifetime>)?,
            $attr0_ident,
            $attr1_ident,
        );
    };
    (
        #[derive(
            $(
                $trait:ident
            ),+
        )]
        struct $struct_ident:ident $(<$struct_lifetime:lifetime>)? {
            $attr0_ident:ident: $val0_type_ident:path,
            $attr1_ident:ident: $val1_type_ident:path,
            $attr2_ident:ident: $val2_type_ident:path,
        }
    ) => {
        #[derive(
            $(
                $trait
            ),+
        )]
        #[non_exhaustive]
        pub struct $struct_ident $(<$struct_lifetime>)? {
            pub $attr0_ident: $val0_type_ident,
            pub $attr1_ident: $val1_type_ident,
            pub $attr2_ident: $val2_type_ident,
        }
        _impl_elt_matcher!(
            $struct_ident $(<$struct_lifetime>)?,
            $attr0_ident,
            $attr1_ident,
            $attr2_ident,
        );
    };
    (
        #[derive(
            $(
                $trait:ident
            ),+
        )]
        struct $struct_ident:ident $(<$struct_lifetime:lifetime>)? {
            $attr0_ident:ident: $val0_type_ident:path,
            $attr1_ident:ident: $val1_type_ident:path,
            $attr2_ident:ident: $val2_type_ident:path,
            $attr3_ident:ident: $val3_type_ident:path,
        }
    ) => {
        #[derive(
            $(
                $trait
            ),+
        )]
        #[non_exhaustive]
        pub struct $struct_ident $(<$struct_lifetime>)? {
            pub $attr0_ident: $val0_type_ident,
            pub $attr1_ident: $val1_type_ident,
            pub $attr2_ident: $val2_type_ident,
            pub $attr3_ident: $val3_type_ident,
        }
        _impl_elt_matcher!(
            $struct_ident $(<$struct_lifetime>)?,
            $attr0_ident,
            $attr1_ident,
            $attr2_ident,
            $attr3_ident,
        );
    };
}

macro_rules! impl_elt_with_body_matcher {
    (
        #[derive(
            $(
                $trait:ident
            ),+
        )]
        struct $struct_ident:ident $(<$struct_lifetime:lifetime>)? {
            $body_ident:ident: $body_type_ident:path,
        }
    ) => {
        #[derive(
            $(
                $trait
            ),+
        )]
        #[allow(clippy::upper_case_acronyms, clippy::allow_attributes)]
        #[non_exhaustive]
        pub struct $struct_ident $(<$struct_lifetime>)? {
            pub $body_ident: $body_type_ident,
        }
        _impl_elt_with_body_matcher!(
            $struct_ident $(<$struct_lifetime>)?,
            ; $body_ident
        );
    };
    (
        #[derive(
            $(
                $trait:ident
            ),+
        )]
        struct $struct_ident:ident $(<$struct_lifetime:lifetime>)? {
            $attr0_ident:ident: $val0_type_ident:path,
            $body_ident:ident: $body_type_ident:path,
        }
    ) => {
        #[derive(
            $(
                $trait
            ),+
        )]
        #[allow(clippy::upper_case_acronyms, clippy::allow_attributes)]
        #[non_exhaustive]
        pub struct $struct_ident $(<$struct_lifetime>)? {
            pub $attr0_ident: $val0_type_ident,
            pub $body_ident: $body_type_ident,
        }
        _impl_elt_with_body_matcher!(
            $struct_ident $(<$struct_lifetime>)?,
            $attr0_ident,
            ; $body_ident
        );
    };
    (
        #[derive(
            $(
                $trait:ident
            ),+
        )]
        struct $struct_ident:ident $(<$struct_lifetime:lifetime>)? {
            $attr0_ident:ident: $val0_type_ident:path,
            $attr1_ident:ident: $val1_type_ident:path,
            $body_ident:ident: $body_type_ident:path,
        }
    ) => {
        #[derive(
            $(
                $trait
            ),+
        )]
        #[allow(clippy::upper_case_acronyms, clippy::allow_attributes)]
        #[non_exhaustive]
        pub struct $struct_ident $(<$struct_lifetime>)? {
            pub $attr0_ident: $val0_type_ident,
            pub $attr1_ident: $val1_type_ident,
            pub $body_ident: $body_type_ident,
        }
        _impl_elt_with_body_matcher!(
            $struct_ident $(<$struct_lifetime>)?,
            $attr0_ident,
            $attr1_ident,
            ; $body_ident
        );
    };
}

#[inline]
pub fn parse_str(str: &str) -> Result<Cameras<'_>, String> {
    let preambule_end = str.find("<Cameras").unwrap_or(0);
    let (_, str) = str.split_at(preambule_end);
    xmlparser::ParseStream::new(str).parse::<Cameras<'_>>()
}

#[cfg(test)]
mod tests;

mod alias;
mod aliases;
mod black;
pub mod blackareas;
mod camera;
pub mod cameras;
mod cfa;
mod cfa2;
mod color;
mod colormatrices;
mod colormatrix;
mod colormatrixrow;
mod colorrow;
pub mod crop;
mod decoder_version;
mod height;
mod hint;
mod hints;
mod horizontal;
mod id;
mod id_attr;
mod iso_list;
mod iso_max;
mod iso_min;
mod make;
mod mode;
mod model;
mod name;
mod plane;
mod planes;
mod sensor;
mod supported;
mod value;
mod vertical;
mod white;
mod width;
mod x;
mod y;

pub use camera::Camera;
pub use cameras::Cameras;
pub use hint::Hint;
pub use hints::Hints;
pub use supported::Supported;
