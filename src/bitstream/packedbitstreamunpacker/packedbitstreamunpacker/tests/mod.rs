macro_rules! test {
    (unpack $bytes:tt as $N:literal-bits: $items:tt) => {
        let input = $bytes;
        let bss = BitStreamSlice::<T>::new(&input).unwrap();
        let pbss = PackedBitstreamSlice::<_, $N>::new(bss).unwrap();
        let q = PackedBitstreamUnpacker::new(pbss).unwrap();
        assert_eq!(q.as_slice(), $items);
    };
}

#[cfg(test)]
mod lsb;

#[cfg(test)]
mod msb;

#[cfg(test)]
mod msb16;

#[cfg(test)]
mod msb32;
