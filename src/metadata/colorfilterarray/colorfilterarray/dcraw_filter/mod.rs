use rawspeed_std::coord_common::{
    ColIndex, ColOffset, Coord2D, CoordOffset2D, RowIndex, RowOffset,
};
use rawspeed_std_ndslice::{
    array2dref::Array2DRef, offsetarray2dref::OffsetArray2DRef,
};

use crate::colorfilterarray::ColorVariant;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct ColorVariantArray<T> {
    data: [T; ColorVariant::card()],
}

impl<T> ColorVariantArray<T> {
    #[inline]
    #[must_use]
    const fn indice(color: ColorVariant) -> usize {
        color as usize
    }
}

impl<T> core::ops::Index<ColorVariant> for ColorVariantArray<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: ColorVariant) -> &Self::Output {
        let index = ColorVariantArray::<T>::indice(index);
        self.data.get(index).unwrap()
    }
}

impl<T> core::ops::IndexMut<ColorVariant> for ColorVariantArray<T> {
    #[inline]
    fn index_mut(&mut self, index: ColorVariant) -> &mut Self::Output {
        let index = ColorVariantArray::<T>::indice(index);
        self.data.get_mut(index).unwrap()
    }
}

impl From<OffsetArray2DRef<'_, ColorVariant>> for ColorVariantArray<bool> {
    #[inline]
    fn from(cfa: OffsetArray2DRef<'_, ColorVariant>) -> Self {
        let mut seen = ColorVariantArray::<bool>::default();
        for row in 0..*cfa.num_rows() {
            for col in 0..*cfa.row_length() {
                let i = Coord2D::new(RowIndex::new(row), ColIndex::new(col));
                let color = cfa[i];
                seen[color] = true;
            }
        }
        seen
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ColorBasisComponents {
    basis: &'static [ColorVariant],
}

impl ColorBasisComponents {
    #[inline]
    #[must_use]
    const fn new(basis: &'static [ColorVariant]) -> Self {
        Self { basis }
    }
}

impl core::ops::Deref for ColorBasisComponents {
    type Target = &'static [ColorVariant];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.basis
    }
}

impl From<ColorBasisComponents> for ColorVariantArray<bool> {
    #[inline]
    fn from(val: ColorBasisComponents) -> Self {
        let mut contains = ColorVariantArray::<bool>::default();
        for component in val.iter().copied() {
            contains[component] = true;
        }
        contains
    }
}

#[expect(clippy::missing_trait_methods)]
impl PartialEq<ColorVariantArray<bool>> for ColorBasisComponents {
    #[inline]
    fn eq(&self, other: &ColorVariantArray<bool>) -> bool {
        <ColorVariantArray<bool>>::from(*self) == *other
    }
}

#[expect(clippy::missing_trait_methods)]
impl PartialEq<ColorVariantArray<bool>> for ColorBasis {
    #[inline]
    fn eq(&self, other: &ColorVariantArray<bool>) -> bool {
        <ColorBasisComponents>::from(*self) == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
enum ColorBasisError {
    UnknownCFABasis,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
enum ColorBasis {
    Bayer,
    #[expect(clippy::upper_case_acronyms)]
    RGBY,
    #[expect(clippy::upper_case_acronyms)]
    CYGM,
}

impl ColorBasis {
    #[inline]
    #[must_use]
    const fn bases() -> &'static [ColorBasis] {
        &[ColorBasis::Bayer, ColorBasis::RGBY, ColorBasis::CYGM]
    }

    #[inline]
    #[must_use]
    fn get_components(self) -> ColorBasisComponents {
        self.into()
    }
}

impl From<ColorBasis> for ColorBasisComponents {
    #[inline]
    fn from(value: ColorBasis) -> Self {
        match value {
            ColorBasis::Bayer => ColorBasisComponents::new(&[
                ColorVariant::Red,
                ColorVariant::Green,
                ColorVariant::Blue,
            ]),
            ColorBasis::RGBY => ColorBasisComponents::new(&[
                ColorVariant::Red,
                ColorVariant::Green,
                ColorVariant::Blue,
                ColorVariant::Yellow,
            ]),
            ColorBasis::CYGM => ColorBasisComponents::new(&[
                ColorVariant::FujiGreen,
                ColorVariant::Magenta,
                ColorVariant::Cyan,
                ColorVariant::Yellow,
            ]),
        }
    }
}

impl TryFrom<OffsetArray2DRef<'_, ColorVariant>> for ColorBasis {
    type Error = ColorBasisError;

    #[inline]
    fn try_from(
        cfa: OffsetArray2DRef<'_, ColorVariant>,
    ) -> Result<Self, Self::Error> {
        let used_colors: ColorVariantArray<bool> = cfa.into();
        ColorBasis::bases()
            .iter()
            .copied()
            .find(|basis: &ColorBasis| *basis == used_colors)
            .ok_or(ColorBasisError::UnknownCFABasis)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
enum DCrawColorIndexError {
    WrongBasis,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum DCrawFilterError {
    BadDims,
    XTrans,
    UnknownCFABasis,
}

impl From<ColorBasisError> for DCrawFilterError {
    #[inline]
    fn from(value: ColorBasisError) -> Self {
        match value {
            ColorBasisError::UnknownCFABasis => {
                DCrawFilterError::UnknownCFABasis
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct DCrawFilter {
    filter: u32,
}

impl DCrawFilter {
    #[inline]
    fn get_dcraw_color_index(
        color: ColorVariant,
        basis: ColorBasis,
    ) -> Result<usize, DCrawColorIndexError> {
        basis
            .get_components()
            .iter()
            .position(|component| *component == color)
            .ok_or(DCrawColorIndexError::WrongBasis)
    }

    #[inline]
    #[must_use]
    pub const fn filter(&self) -> u32 {
        self.filter
    }
}

impl TryFrom<OffsetArray2DRef<'_, ColorVariant>> for DCrawFilter {
    type Error = DCrawFilterError;

    #[inline]
    fn try_from(
        cfa: OffsetArray2DRef<'_, ColorVariant>,
    ) -> Result<Self, Self::Error> {
        if *cfa.num_rows() == 6 && *cfa.row_length() == 6 {
            return Err(DCrawFilterError::XTrans);
        }
        if !(*cfa.num_rows() > 0
            && *cfa.row_length() > 0
            && *cfa.num_rows() <= 8
            && *cfa.row_length() <= 2
            && cfa.num_rows().is_power_of_two()
            && cfa.row_length().is_power_of_two())
        {
            return Err(DCrawFilterError::BadDims);
        }
        let basis: ColorBasis = cfa.try_into()?;
        let mut ret: u32 = 0;
        for x in 0..2 {
            for y in 0..8 {
                let pos = Coord2D::new(
                    RowIndex::new(y % *cfa.num_rows()),
                    ColIndex::new(x % *cfa.row_length()),
                );
                let color = cfa[pos];
                let c: u32 = DCrawFilter::get_dcraw_color_index(color, basis)
                    .unwrap()
                    .try_into()
                    .unwrap();
                assert!((0..=3).contains(&c));
                let idx = (x >> 1) * 4 + y * 2 + (x & 1);
                ret |= c << (2 * idx);
            }
        }
        Ok(DCrawFilter { filter: ret })
    }
}

impl TryFrom<Array2DRef<'_, ColorVariant>> for DCrawFilter {
    type Error = DCrawFilterError;

    #[inline]
    fn try_from(
        cfa: Array2DRef<'_, ColorVariant>,
    ) -> Result<Self, Self::Error> {
        let zero_offset =
            CoordOffset2D::new(RowOffset::new(0), ColOffset::new(0));
        let cfa = OffsetArray2DRef::new(cfa, zero_offset);
        cfa.try_into()
    }
}

#[cfg(test)]
mod tests;
