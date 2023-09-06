use std::{any::type_name, ops::Deref, sync::Arc};

use crate::extensions::Extensions;

// use actix_http::Extensions;
// use actix_utils::future::{err, ok, Ready};
// use futures_core::future::LocalBoxFuture;
// use serde::{de, Serialize};


/// Data factory.
pub(crate) trait DataFactory {
    /// Return true if modifications were made to extensions map.
    fn create(&self, extensions: &mut Extensions) -> bool;
}

// pub(crate) type FnDataFactory =
//     Box<dyn Fn() -> LocalBoxFuture<'static, Result<Box<dyn DataFactory>, ()>>>;


    
#[doc(alias = "state")]
#[derive(Debug)]
pub struct Data<T: ?Sized>(Arc<T>);

impl<T> Data<T> {
    /// Create new `Data` instance.
    pub fn new(state: T) -> Data<T> {
        Data(Arc::new(state))
    }
}

impl<T: ?Sized> Data<T> {
    /// Returns reference to inner `T`.
    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    /// Unwraps to the internal `Arc<T>`
    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Deref for Data<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.0
    }
}

impl<T: ?Sized> Clone for Data<T> {
    fn clone(&self) -> Data<T> {
        Data(Arc::clone(&self.0))
    }
}

impl<T: ?Sized> From<Arc<T>> for Data<T> {
    fn from(arc: Arc<T>) -> Self {
        Data(arc)
    }
}

impl<T: Default> Default for Data<T> {
    fn default() -> Self {
        Data::new(T::default())
    }
}

// impl<T> Serialize for Data<T>
// where
//     T: Serialize,
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.0.serialize(serializer)
//     }
// }
// impl<'de, T> de::Deserialize<'de> for Data<T>
// where
//     T: de::Deserialize<'de>,
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: de::Deserializer<'de>,
//     {
//         Ok(Data::new(T::deserialize(deserializer)?))
//     }
// }

// impl<T: ?Sized + 'static> FromRequest for Data<T> {
//     type Error = Error;
//     type Future = Ready<Result<Self, Error>>;

//     #[inline]
//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         if let Some(st) = req.app_data::<Data<T>>() {
//             ok(st.clone())
//         } else {
//             log::debug!(
//                 "Failed to extract `Data<{}>` for `{}` handler. For the Data extractor to work \
//                 correctly, wrap the data with `Data::new()` and pass it to `App::app_data()`. \
//                 Ensure that types align in both the set and retrieve calls.",
//                 type_name::<T>(),
//                 req.match_name().unwrap_or_else(|| req.path())
//             );

//             err(error::ErrorInternalServerError(
//                 "Requested application data is not configured correctly. \
//                 View/enable debug logs for more details.",
//             ))
//         }
//     }
// }

impl<T: ?Sized + 'static> DataFactory for Data<T> {
    fn create(&self, extensions: &mut Extensions) -> bool {
        extensions.insert(Data(self.0.clone()));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        App
    };


    #[test]
     fn test_data_from_arc() {
        let data_new = Data::new(String::from("test-123"));
        let data_from_arc = Data::from(Arc::new(String::from("test-123")));
        assert_eq!(data_new.0, data_from_arc.0);
    }

    #[test]
      fn test_data_from_dyn_arc() {
        trait TestTrait {
            fn get_num(&self) -> i32;
        }
        struct A {}
        impl TestTrait for A {
            fn get_num(&self) -> i32 {
                42
            }
        }
        // This works when Sized is required
        let dyn_arc_box: Arc<Box<dyn TestTrait>> = Arc::new(Box::new(A {}));
        let data_arc_box = Data::from(dyn_arc_box);
        // This works when Data Sized Bound is removed
        let dyn_arc: Arc<dyn TestTrait> = Arc::new(A {});
        let data_arc = Data::from(dyn_arc);
        assert_eq!(data_arc_box.get_num(), data_arc.get_num())
    }

    #[test]
      fn test_dyn_data_into_arc() {
        trait TestTrait {
            fn get_num(&self) -> i32;
        }
        struct A {}
        impl TestTrait for A {
            fn get_num(&self) -> i32 {
                42
            }
        }
        let dyn_arc: Arc<dyn TestTrait> = Arc::new(A {});
        let data_arc = Data::from(dyn_arc);
        let arc_from_data = data_arc.clone().into_inner();
        assert_eq!(data_arc.get_num(), arc_from_data.get_num())
    }

    #[test]
     fn test_get_ref_from_dyn_data() {
        trait TestTrait {
            fn get_num(&self) -> i32;
        }
        struct A {}
        impl TestTrait for A {
            fn get_num(&self) -> i32 {
                42
            }
        }
        let dyn_arc: Arc<dyn TestTrait> = Arc::new(A {});
        let data_arc = Data::from(dyn_arc);
        let ref_data = data_arc.get_ref();
        assert_eq!(data_arc.get_num(), ref_data.get_num())
    }
}
