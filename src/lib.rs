extern crate proc_macro;
use quote::quote;
use syn::{Type, TypeReference, parse_macro_input};
use syn::parse::Parse;

mod impl_docs;

use crate::impl_docs::vector_add::vector_add_impl_doc;
use crate::impl_docs::vector_dot_prod::vector_dot_prod_impl_doc;
use crate::impl_docs::vector_scalar_mul::vector_scalar_mul_impl_doc;

fn is_borrow(ty: &Type) -> bool {
    match ty {
        Type::Reference(_) => true,
        _ => false
    }
}

fn is_mutable(ty: &Type) -> bool {
    if let Type::Reference(TypeReference {
        mutability : Some(_),
        ..
    }) = ty {
        true
    } else {
        false
    }
}

struct VectorImplTypes {
    lhs_ty : Box<Type>,
    rhs_ty : Box<Type>,
}

impl Parse for VectorImplTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(VectorImplTypes {
            lhs_ty: input.parse()?,
            rhs_ty: input.parse()?
        })
    }
}

#[proc_macro]
pub fn vector_add_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data = parse_macro_input!(input as VectorImplTypes);

    let left_hand_type: proc_macro2::TokenStream;
    let right_hand_type: proc_macro2::TokenStream;

    let types_state = (
        is_borrow(&data.lhs_ty), 
        is_mutable(&data.lhs_ty), 
        is_borrow(&data.rhs_ty), 
        is_mutable(&data.rhs_ty)
    );

    match types_state {
        // &mut Vector, &mut Vector
        (true, true, true, true) => {
            left_hand_type = quote!(&'a mut Vector<T>);
            right_hand_type = quote!(&'a mut Vector<T>);
        },
        
        // &mut Vector, &Vector
        (true, true, true, false) => {
            left_hand_type = quote!(&'a mut Vector<T>);
            right_hand_type = quote!(&'a Vector<T>);
        },

        // &mut Vector, Vector
        (true, true, false, false) => {
            left_hand_type = quote!(&'a mut Vector<T>);
            right_hand_type = quote!(Vector<T>);
        },

        // &Vector, &mut Vector
        (true, false, true, true) => {
            left_hand_type = quote!(&'a Vector<T>);
            right_hand_type = quote!(&'a mut Vector<T>);
        },

        // Vector, &mut Vector
        (false, false, true, true) => {
            left_hand_type = quote!(Vector<T>);
            right_hand_type = quote!(&'a mut Vector<T>);
        },

        // -------------------------

        // &Vector, &Vector
        (true, false, true, false) => {
            left_hand_type = quote!(&Vector<T>);
            right_hand_type = quote!(&Vector<T>);
        },

        // &Vector, Vector
        (true, false, false, false) => {
            left_hand_type = quote!(&Vector<T>);
            right_hand_type = quote!(Vector<T>);
        },

        // Vector, &Vector
        (false, false, true, false) => {
            left_hand_type = quote!(Vector<T>);
            right_hand_type = quote!(&Vector<T>);
        },

        // Vector, Vector
        (false, false, false, false) => {
            left_hand_type = quote!(Vector<T>);
            right_hand_type = quote!(Vector<T>);
        },

        _ => panic!("Not supported")
    }

    let documentation = vector_add_impl_doc(types_state);

    let immut_impl = quote!{
        #documentation
        impl<T> Add<#right_hand_type> for #left_hand_type
        where
            T: Add<Output = T> + Copy
        {
            type Output = Vector<T>;

            fn add(self, rhs: #right_hand_type) -> Self::Output {
                if self.len() != rhs.len() {
                    panic!("Vectors with different sizes cannot be added together.")
                }
                
                let length = self.len();

                let mut params = Vec::with_capacity(length);
                for idx in 0..length {
                    params.push(self.list[idx] + rhs.list[idx])
                }

                Vector::from(params)
            }
        }
    };

    let left_mut_impl = quote!{
        #documentation
        impl<'a, T> Add<#right_hand_type> for #left_hand_type
        where
            T: Add<Output = T> + Copy
        {
            type Output = &'a mut Vector<T>;

            fn add(self, rhs: #right_hand_type) -> Self::Output {
                if self.len() != rhs.len() {
                    panic!("Vectors with different sizes cannot be added together.")
                }

                for idx in 0..self.len() {
                    self.list[idx] = self.list[idx] + rhs.list[idx]
                }

                self
            }
        }
    };

    let right_mut_impl = quote!{
        #documentation
        impl<'a, T> Add<#right_hand_type> for #left_hand_type
        where
            T: Add<Output = T> + Copy
        {
            type Output = &'a mut Vector<T>;

            fn add(self, rhs: #right_hand_type) -> Self::Output {
                if self.len() != rhs.len() {
                    panic!("Vectors with different sizes cannot be added together.")
                }

                for idx in 0..self.len() {
                    rhs.list[idx] = self.list[idx] + rhs.list[idx]
                }

                rhs
            }
        }
    };

    match (is_mutable(&data.lhs_ty), is_mutable(&data.rhs_ty)) {
        (true, _) => {
            return left_mut_impl.into();
        },
        (_, true) => {
            return right_mut_impl.into();
        },
        (false, false) => {
            return immut_impl.into();
        },
    }
}

#[proc_macro]
pub fn vector_dot_prod_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data = parse_macro_input!(input as VectorImplTypes);

    let left_hand_type: proc_macro2::TokenStream;
    let right_hand_type: proc_macro2::TokenStream;

    let types_state = (
        is_borrow(&data.lhs_ty), 
        is_mutable(&data.lhs_ty), 
        is_borrow(&data.rhs_ty), 
        is_mutable(&data.rhs_ty)
    );

    match types_state {
        // &mut Vector, &mut Vector
        (true, true, true, true) => {
            left_hand_type = quote!(&mut Vector<T>);
            right_hand_type = quote!(&mut Vector<T>);
        },
        
        // &mut Vector, &Vector
        (true, true, true, false) => {
            left_hand_type = quote!(&mut Vector<T>);
            right_hand_type = quote!(&Vector<T>);
        },

        // &mut Vector, Vector
        (true, true, false, false) => {
            left_hand_type = quote!(&mut Vector<T>);
            right_hand_type = quote!(Vector<T>);
        },

        // &Vector, &mut Vector
        (true, false, true, true) => {
            left_hand_type = quote!(&Vector<T>);
            right_hand_type = quote!(&mut Vector<T>);
        },

        // Vector, &mut Vector
        (false, false, true, true) => {
            left_hand_type = quote!(Vector<T>);
            right_hand_type = quote!(&mut Vector<T>);
        },

        // -------------------------

        // &Vector, &Vector
        (true, false, true, false) => {
            left_hand_type = quote!(&Vector<T>);
            right_hand_type = quote!(&Vector<T>);
        },

        // &Vector, Vector
        (true, false, false, false) => {
            left_hand_type = quote!(&Vector<T>);
            right_hand_type = quote!(Vector<T>);
        },

        // Vector, &Vector
        (false, false, true, false) => {
            left_hand_type = quote!(Vector<T>);
            right_hand_type = quote!(&Vector<T>);
        },

        // Vector, Vector
        (false, false, false, false) => {
            left_hand_type = quote!(Vector<T>);
            right_hand_type = quote!(Vector<T>);
        },

        _ => panic!("Not supported")
    }

    let documentation = vector_dot_prod_impl_doc(types_state);

    let implemtation = quote!{
        #documentation
        impl<T> Mul<#right_hand_type> for #left_hand_type
        where
            T: Copy + Mul<Output = T> + AddAssign + Default
        {
            type Output = T;

            fn mul(self, rhs: #right_hand_type) -> Self::Output {
                if self.len() != rhs.len() {
                    panic!("Cannot find dot product of two differently sized vectors.")
                }

                let mut product = T::default();
                
                for idx in 0..self.len() {
                    product += self.list[idx] * rhs.list[idx]
                }

                product
            }
        }
    };

    implemtation.into()
}

#[proc_macro]
pub fn vector_scalar_mul_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data = parse_macro_input!(input as VectorImplTypes);

    let left_hand_type: proc_macro2::TokenStream;

    let type_state = (
        is_borrow(&data.lhs_ty), 
        is_mutable(&data.lhs_ty),
    );

    match type_state {
        //&mut Vector<T> * T
        (true, true) => {
            left_hand_type = quote!(&'a mut Vector<T>)
        },

        //&Vector<T> * T
        (true, false) => {
            left_hand_type = quote!(&Vector<T>)
        },

        //Vector<T> * T
        (false, false) => {
            left_hand_type = quote!(Vector<T>)
        },

        _ => panic!("Not supported"),
    }

    let documentation = vector_scalar_mul_impl_doc(type_state);

    let implementation;

    if is_mutable(&data.lhs_ty) {
        implementation = quote!{
            #documentation
            impl<'a, T> Mul<T> for #left_hand_type
            where
                T: Copy + Mul<Output = T>
            {
                type Output = &'a mut Vector<T>;
                
                fn mul(self, rhs: T) -> Self::Output {
                    let list_ptr = self.list.as_mut_ptr();
            
                    for idx in 0..self.list.len() {
                        //SAFETY:
                        //Since the pointer to the list is
                        //the same list which is being iterated
                        //over, the index will never go out of scope.
                        //
                        //Additionally, the original values are used
                        //before the memory location is overwitten.
                        unsafe {
                            self.list[idx] = rhs * *list_ptr.add(idx)
                        }
                    }
                    self
                }
            }
        };
    } else {
        implementation = quote!{
            #documentation
            impl<T> Mul<T> for #left_hand_type
            where
                T: Copy + Mul<Output = T>
            {
                type Output = Vector<T>;
            
                fn mul(self, rhs: T) -> Self::Output {
                    let mut params = Vec::with_capacity(self.len());
                    for item in self.list() {
                        params.push(rhs * *item)
                    }
                    Vector::from(params)
                }
            }
        };
    }

    implementation.into()
}