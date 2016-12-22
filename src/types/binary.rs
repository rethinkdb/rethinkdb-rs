use types::data;

pub trait IntoBinary {
    fn into_binary(self) -> data::Binary;
}
