use acvm::FieldElement;

use super::{barretenberg_structures::Assignments, BlackboxSolver, Error};
use crate::bindings::pedersen;

pub(crate) trait Pedersen {
    fn encrypt(
        &self,
        inputs: Vec<FieldElement>,
        hash_index: u32,
    ) -> Result<(FieldElement, FieldElement), Error>;
}

impl Pedersen for BlackboxSolver {
    fn encrypt(
        &self,
        inputs: Vec<FieldElement>,
        hash_index: u32,
    ) -> Result<(FieldElement, FieldElement), Error> {
        let input_buf = Assignments::from(inputs).to_bytes();
        let result =
            pedersen::plookup_commit_with_hash_index(input_buf.as_slice(), hash_index).unwrap();
        let (point_x_bytes, point_y_bytes) = result.split_at(32);
        let point_x = FieldElement::from_be_bytes_reduce(point_x_bytes);
        let point_y = FieldElement::from_be_bytes_reduce(point_y_bytes);
        Ok((point_x, point_y))
    }
}

#[test]
fn pedersen_hash_to_point() {
    let solver = BlackboxSolver::new();
    let (x, y) = solver
        .encrypt(vec![FieldElement::zero(), FieldElement::one()], 0)
        .unwrap();
    let expected_x = FieldElement::from_hex(
        "0x0c5e1ddecd49de44ed5e5798d3f6fb7c71fe3d37f5bee8664cf88a445b5ba0af",
    )
    .unwrap();
    let expected_y = FieldElement::from_hex(
        "0x230294a041e26fe80b827c2ef5cb8784642bbaa83842da2714d62b1f3c4f9752",
    )
    .unwrap();

    assert_eq!(expected_x.to_hex(), x.to_hex());
    assert_eq!(expected_y.to_hex(), y.to_hex());
}
