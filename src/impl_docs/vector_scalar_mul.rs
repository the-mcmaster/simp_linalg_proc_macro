use quote::quote;

pub fn vector_scalar_mul_impl_doc(type_state : (bool, bool)) -> proc_macro2::TokenStream {
    match type_state {

        //&mut Vector<T> * T
        (true, true) => quote!{
            /// The [multiplication][std::ops::Mul] implementation for '&mut Vector * T'.
            /// 
            /// In contrast to common mathematical notation,
            /// the scalar must be on the right of the vector.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::prelude::*;
            /// 
            /// let mut vector = vector![1, 2, 3];
            /// 
            /// &mut vector * 3;
            /// 
            /// assert_eq!(vector, vector![3, 6, 9])
            /// ```
        },

        //&Vector<T> * T
        (true, false) => quote!{
            /// The [multiplication][std::ops::Mul] implementation for '&Vector * T'.
            /// 
            /// In contrast to common mathematical notation,
            /// the scalar must be on the right of the vector.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::Vector;
            /// use simp_linalg::vector;
            /// 
            /// let vector = vector![1, 2, 3];
            /// 
            /// assert_eq!(&vector * 3, vector![3, 6, 9])
            /// ```
        },

        //Vector<T> * T
        (false, false) => quote!{
            /// The [multiplication][std::ops::Mul] implementation for 'Vector * T'.
            /// 
            /// In contrast to common mathematical notation,
            /// the scalar must be on the right of the vector.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::prelude::*;
            /// 
            /// let vector = vector![1, 2, 3];
            /// 
            /// // Notice that the vector is moved here.
            /// assert_eq!(vector * 3, vector![3, 6, 9])
            /// ```
        },
        _ => panic!("Not supported")
    }
}