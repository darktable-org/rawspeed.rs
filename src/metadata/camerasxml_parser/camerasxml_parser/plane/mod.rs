use super::xmlparser;

mod repr {
    use super::super::Int;
    use super::xmlparser;
    impl_attr_matcher!(
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct Plane {
            plane: Int,
        }
    );
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    plane: u8,
}

impl Plane {
    #[must_use]
    #[inline]
    pub const fn new(plane: u8) -> Self {
        Self { plane }
    }
}

impl core::ops::Deref for Plane {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.plane
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for Plane {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let plane = **input.parse::<repr::Plane>()?;
        match plane {
            0..=3 => Ok(Self::new(plane.try_into().unwrap())),
            _ => Err(format!("Invalid plane index: {plane}")),
        }
    }
}
