use faer::{mat, Mat};

use crate::scalar::Scalar;
use crate::*;

impl From<Mat<f64>> for Robj {
    /// Convert a faer Mat<f64> into Robj.
    fn from(mat: Mat<f64>) -> Self {
        mat.col_chunks(1)
            .flat_map(|c| c.row_chunks(1).map(|r| r.read(0, 0)))
            .collect_robj()
    }
}

impl<'a> FromRobj<'a> for Mat<f64> {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if robj.is_matrix() {
            if let Some(dim) = robj.dim() {
                let dim: Vec<_> = dim.iter().map(|d| d.inner() as usize).collect();

                if dim.len() != 2 {
                    return Err("could not convert to matrix");
                }

                if let Some(slice) = robj.as_real_slice() {
                    let fmat = mat::from_column_major_slice::<f64>(&slice, dim[0], dim[1]);
                    Ok(fmat.to_owned())
                } else if let Some(slice) = robj.as_integer_slice() {
                    let fmat =
                        Mat::<f64>::from_fn(dim[0], dim[1], |i, j| slice[i + j * dim[0]] as f64);
                    Ok(fmat)
                } else {
                    Err("could not convert to matrix")
                }
            } else {
                Err("could not convert to matrix")
            }
        } else {
            Err("R object is not a matrix")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use faer::{mat, Mat};

    #[test]
    fn test_robj_to_faer_mat() {
        test! {
            let values = [
                [1.0, 5.0, 9.0],
                [2.0, 6.0, 10.0],
                [3.0, 7.0, 11.0],
                [4.0, 8.0, 12.0f64]
            ];
            let a = Mat::<f64>::from_fn(4, 3, |i, j| values[i][j] as f64);

            let rmatrix = RMatrix::new_matrix(4, 3, |i, j| values[i][j]);
            let b = Mat::<f64>::from_robj(&Robj::from(rmatrix));
            assert_eq!(a, b.expect("matrix to be converted"));
        }
    }

    #[test]
    fn test_faer_mat_to_robj() {
        test! {
            let vec: Vec<f64> = (1..13).map(f64::from).collect();
            let a = mat![
                [1.0, 5.0, 9.0],
                [2.0, 6.0, 10.0],
                [3.0, 7.0, 11.0],
                [4.0, 8.0, 12.0f64],
            ];
            let robj: Robj = a.clone().into();
            assert_eq!(robj.as_real_slice().expect("slice"), &vec);
        }
    }
}