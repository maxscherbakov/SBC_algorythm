pub(crate) trait Chunk {
    fn decode(&self);

    fn get_data(&self) -> Vec<u8>;

    fn size(&self) -> usize {
        self.get_data().len()
    }
}
