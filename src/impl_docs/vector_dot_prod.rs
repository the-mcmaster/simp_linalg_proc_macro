use quote::quote;

pub fn vector_dot_prod_impl_doc(types_state : (bool, bool, bool, bool)) -> proc_macro2::TokenStream {
    match types_state {
        // &mut Vector, &mut Vector
        (true, true, true, true) => quote!{
        },
        
        // &mut Vector, &Vector
        (true, true, true, false) => quote!{
        },

        // &mut Vector, Vector
        (true, true, false, false) => quote!{
        },

        // &Vector, &mut Vector
        (true, false, true, true) => quote!{
        },

        // Vector, &mut Vector
        (false, false, true, true) => quote!{
        },

        // -------------------------

        // &Vector, &Vector
        (true, false, true, false) => quote!{
            /// The [multiplication][std::ops::Mul] implementation for '&Vector * &Vector'.
            /// 
            /// This calculates the dot product of the two vectors.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::Vector;
            /// use simp_linalg::vector;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// let value = &vector1 * &vector2;
            /// 
            /// assert_eq!(value, 32)
            /// ```
            /// 
            /// # Panic!
            /// This function will panic if the vectors are not the same size.
        },

        // &Vector, Vector
        (true, false, false, false) => quote!{
            /// The [multiplication][std::ops::Mul] implementation for '&Vector * Vector'.
            /// 
            /// This calculates the dot product of the two vectors.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::Vector;
            /// use simp_linalg::vector;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// // Notice 'vector2' is moved here.
            /// let value = &vector1 * vector2;
            /// 
            /// assert_eq!(value, 32)
            /// ```
            /// 
            /// # Panic!
            /// This function will panic if the vectors are not the same size.
        },

        // Vector, &Vector
        (false, false, true, false) => quote!{
            /// The [multiplication][std::ops::Mul] implementation for 'Vector * &Vector'.
            /// 
            /// This calculates the dot product of the two vectors.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::Vector;
            /// use simp_linalg::vector;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// // Notice 'vector1' is moved here.
            /// let value = vector1 * &vector2;
            /// 
            /// assert_eq!(value, 32)
            /// ```
            /// 
            /// # Panic!
            /// This function will panic if the vectors are not the same size.
        },

        // Vector, Vector
        (false, false, false, false) => quote!{
            /// The [multiplication][std::ops::Mul] implementation for 'Vector * Vector'.
            /// 
            /// This calculates the dot product of the two vectors.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::Vector;
            /// use simp_linalg::vector;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// // Notice 'vector1' and 'vector2' are moved here.
            /// let value = vector1 * &vector2;
            /// 
            /// assert_eq!(value, 32)
            /// ```
            /// 
            /// # Panic!
            /// This function will panic if the vectors are not the same size.
        },

        _ => panic!("Not supported")
    }
}