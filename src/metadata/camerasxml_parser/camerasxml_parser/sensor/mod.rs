use super::{black, iso_list, iso_max, iso_min, white, xmlparser};

#[derive(Debug, Clone, PartialEq)]
pub enum Bounds {
    Unbounded,
    LowerBounded(iso_min::IsoMin),
    UpperBounded(iso_max::IsoMax),
    Range((iso_min::IsoMin, iso_max::IsoMax)),
    Enumerated(iso_list::IsoList),
}

impl Bounds {
    #[must_use]
    #[inline]
    pub fn contains(&self, iso: i32) -> bool {
        match self {
            Bounds::Unbounded => true,
            Bounds::LowerBounded(iso_min) => iso >= ***iso_min,
            Bounds::UpperBounded(iso_max) => iso <= ***iso_max,
            Bounds::Range((iso_min, iso_max)) => {
                iso >= ***iso_min && iso <= ***iso_max
            }
            Bounds::Enumerated(iso_list) => iso_list.values.contains(&iso),
        }
    }
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for Bounds {
    #[inline]
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        if let Ok(list) = input.parse() {
            return Ok(Bounds::Enumerated(list));
        }
        let lb = input.parse().ok().unwrap_or(iso_min::IsoMin {
            val: crate::camerasxml_parser::Int { val: 0 },
        });
        let ub = input.parse().ok().unwrap_or(iso_max::IsoMax {
            val: crate::camerasxml_parser::Int { val: 0 },
        });
        let b = match ((**lb != 0), (**ub != 0)) {
            (true, true) => Bounds::Range((lb, ub)),
            (true, false) => Bounds::LowerBounded(lb),
            (false, true) => Bounds::UpperBounded(ub),
            (false, false) => Bounds::Unbounded,
        };
        Ok(b)
    }
}

impl_elt_matcher!(
    #[derive(Debug, Clone, PartialEq)]
    struct Sensor {
        black: black::Black,
        white: white::White,
        bounds: Bounds,
    }
);

#[cfg(test)]
mod tests;
