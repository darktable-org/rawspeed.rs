use super::Int;
use super::xmlparser;

mod repr {
    use super::Int;
    use super::xmlparser;
    impl_attr_matcher!(
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct Planes {
            planes: Int,
        }
    );
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Planes {
    ThreeComponent,
    FourComponent,
}

impl Planes {
    pub fn val(&self) -> usize {
        (*self).into()
    }
}

impl TryFrom<Int> for Planes {
    type Error = String;

    #[inline]
    fn try_from(value: Int) -> Result<Self, Self::Error> {
        match *value {
            3 => Ok(Planes::ThreeComponent),
            4 => Ok(Planes::FourComponent),
            _ => {
                Err(format!("Unsupported number of color planes ({})", *value))
            }
        }
    }
}

impl From<Planes> for usize {
    #[inline]
    fn from(val: Planes) -> Self {
        match val {
            Planes::ThreeComponent => 3,
            Planes::FourComponent => 4,
        }
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for Planes {
    #[inline]
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        let planes = input.parse::<repr::Planes>()?;
        (*planes).try_into()
    }
}
