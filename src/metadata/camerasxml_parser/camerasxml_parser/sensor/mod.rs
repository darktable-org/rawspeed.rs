use super::black;
use super::iso_list;
use super::iso_max;
use super::iso_min;
use super::white;
use super::xmlparser;

#[derive(Debug, Clone, PartialEq)]
pub enum Bounds {
    Unbounded,
    LowerBounded(iso_min::IsoMin),
    UpperBounded(iso_max::IsoMax),
    Range((iso_min::IsoMin, iso_max::IsoMax)),
    Enumerated(iso_list::IsoList),
}

impl<'a, 'b> xmlparser::Parse<'a, 'b> for Bounds {
    fn parse(
        input: &'b mut xmlparser::ParseStream<'a>,
    ) -> xmlparser::Result<Self> {
        if let Ok(list) = input.parse() {
            return Ok(Bounds::Enumerated(list));
        }
        let b = match (input.parse().ok(), input.parse().ok()) {
            (Some(lb), Some(ub)) => Bounds::Range((lb, ub)),
            (Some(lb), None) => Bounds::LowerBounded(lb),
            (None, Some(ub)) => Bounds::UpperBounded(ub),
            (None, None) => Self::Unbounded,
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
