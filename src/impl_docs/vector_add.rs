use quote::quote;

pub fn vector_add_impl_doc(types_state : (bool, bool, bool, bool)) -> proc_macro2::TokenStream {
    match types_state {
        // &mut Vector, &mut Vector
        (true, true, true, true) => quote!{
            /// The [addition][std::ops::Add] implementation for '&mut Vector + &mut Vector'.
            /// 
            /// # Warning
            /// While the right hand side is mutable,
            /// nothing will be mutated on the right hand side.
            /// All changes will happen to the left hand side.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::prelude::*;
            /// 
            /// let mut vector1 = vector![1, 2, 3];
            /// let mut vector2 = vector![4, 5, 6];
            /// 
            /// &mut vector1 + &mut vector2;
            /// 
            /// assert_eq!(vector1, vector![5, 7, 9])
            /// ```
            /// 
            /// # Panic!
            /// 
            /// This function will panic if the vectors are not the same size.
        },
        
        // &mut Vector, &Vector
        (true, true, true, false) => quote!{
            /// The [addition][std::ops::Add] implementation for '&mut Vector + &Vector'.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::prelude::*;
            /// 
            /// let mut vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// &mut vector1 + &vector2;
            /// 
            /// assert_eq!(vector1, vector![5, 7, 9])
            /// ```
            /// 
            /// # Panic!
            /// 
            /// This function will panic if the vectors are not the same size.
        },

        // &mut Vector, Vector
        (true, true, false, false) => quote!{
            /// The [addition][std::ops::Add] implementation for '&mut Vector + Vector'.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::prelude::*;
            /// 
            /// let mut vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// // Notice that 'vector2' is moved here
            /// &mut vector1 + vector2;
            /// 
            /// assert_eq!(vector1, vector![5, 7, 9]);
            /// ```
            /// This is useful for addition of vectors that are scaled.
            /// ```
            /// use simp_linalg::vector_impl::prelude::*;
            /// 
            /// let mut vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// // The result of '&vector2 * 2' is an owned Vector,
            /// // which is then added to '&mut vector1'.
            /// &mut vector1 + &vector2 * 2;
            /// 
            /// assert_eq!(vector1, vector![9, 12, 15])
            /// ```
            /// 
            /// # Panic!
            /// 
            /// This function will panic if the vectors are not the same size.
        },

        // &Vector, &mut Vector
        (true, false, true, true) => quote!{
            /// The [addition][std::ops::Add] implementation for '&Vector + &mut Vector'.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::prelude::*;
            /// 
            /// let mut vector1 = vector![1, 2, 3];
            /// let mut vector2 = vector![4, 5, 6];
            /// 
            /// &vector1 + &mut vector2;
            /// 
            /// assert_eq!(vector2, vector![5, 7, 9])
            /// ```
            /// 
            /// # Panic!
            /// 
            /// This function will panic if the vectors are not the same size.
        },

        // Vector, &mut Vector
        (false, false, true, true) => quote!{
            /// The [addition][std::ops::Add] implementation for 'Vector + &mut Vector'.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::prelude::*;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let mut vector2 = vector![4, 5, 6];
            /// 
            /// // Notice that 'vector1' is moved here
            /// vector1 + &mut vector2;
            /// 
            /// assert_eq!(vector2, vector![5, 7, 9]);
            /// ```
            /// This is useful for addition of vectors that are scaled.
            /// ```
            /// use simp_linalg::vector_impl::prelude::*;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let mut vector2 = vector![4, 5, 6];
            /// 
            /// // The result of '&vector1 * 2' is an owned Vector,
            /// // which is then added to '&mut vector2'.
            /// &vector1 * 2 + &mut vector2;
            /// 
            /// assert_eq!(vector2, vector![6, 9, 12])
            /// ```
            /// 
            /// # Panic!
            /// 
            /// This function will panic if the vectors are not the same size.
        },

        // -------------------------

        // &Vector, &Vector
        (true, false, true, false) => quote!{
            /// The [addition][std::ops::Add] implementation for '&Vector + &Vector'.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::Vector;
            /// use simp_linalg::vector;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// let vector3 = &vector1 + &vector2;
            /// 
            /// assert_eq!(vector3, vector![5, 7, 9])
            /// ```
            /// 
            /// # Panic!
            /// 
            /// This function will panic if the vectors are not the same size.
        },

        // &Vector, Vector
        (true, false, false, false) => quote!{
            /// The [addition][std::ops::Add] implementation for '&Vector + Vector'.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::Vector;
            /// use simp_linalg::vector;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// // Notice that 'vector2' is moved here
            /// let vector3 = &vector1 + vector2;
            /// 
            /// assert_eq!(vector3, vector![5, 7, 9]);
            /// ```
            /// This is useful for addition of vectors that are scaled.
            /// ```
            /// use simp_linalg::vector_impl::Vector;
            /// use simp_linalg::vector;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// // The result of '&vector2 * 2' is an owned Vector,
            /// // which is then added to '&vector1'.
            /// let vector3 = &vector1 + &vector2 * 2;
            /// 
            /// assert_eq!(vector3, vector![9, 12, 15])
            /// ```
            /// 
            /// # Panic!
            /// 
            /// This function will panic if the vectors are not the same size.
        },

        // Vector, &Vector
        (false, false, true, false) => quote!{
            /// The [addition][std::ops::Add] implementation for 'Vector + &Vector'.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::Vector;
            /// use simp_linalg::vector;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// // Notice that 'vector1' is moved here
            /// let vector3 = vector1 + &vector2;
            /// 
            /// assert_eq!(vector3, vector![5, 7, 9]);
            /// ```
            /// This is useful for addition of vectors that are scaled.
            /// ```
            /// use simp_linalg::vector_impl::Vector;
            /// use simp_linalg::vector;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// // The result of '&vector1 * 2' is an owned Vector,
            /// // which is then added to '&vector2'.
            /// let vector3 = &vector1 * 2 + &vector2;
            /// 
            /// assert_eq!(vector3, vector![6, 9, 12])
            /// ```
            /// 
            /// # Panic!
            /// 
            /// This function will panic if the vectors are not the same size.
        },

        // Vector, Vector
        (false, false, false, false) => quote!{
            /// The [addition][std::ops::Add] implementation for 'Vector + Vector'.
            /// 
            /// # Example
            /// ```
            /// use simp_linalg::vector_impl::Vector;
            /// use simp_linalg::vector;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// // Notice that both vectors are moved here
            /// let vector3 = vector1 + vector2;
            /// 
            /// assert_eq!(vector3, vector![5, 7, 9]);
            /// ```
            /// This is useful for addition of vectors that are scaled.
            /// ```
            /// use simp_linalg::vector_impl::Vector;
            /// use simp_linalg::vector;
            /// 
            /// let vector1 = vector![1, 2, 3];
            /// let vector2 = vector![4, 5, 6];
            /// 
            /// // The result of '&vector1 * 2' is an owned Vector,
            /// // which is then added to another owned vector '&vector2 * 3'.
            /// let vector3 = &vector1 * 2 + &vector2 * 3;
            /// 
            /// assert_eq!(vector3, vector![14, 19, 24])
            /// ```
            /// 
            /// # Panic!
            /// 
            /// This function will panic if the vectors are not the same size.
        },

        _ => panic!("Not supported")
    }
}