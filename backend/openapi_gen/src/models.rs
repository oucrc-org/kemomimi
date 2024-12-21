#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct CategoriesCategoryIdDeletePathParams {
            /// カテゴリのID
                pub category_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct CategoriesCategoryIdGetPathParams {
            /// カテゴリのID
                pub category_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct CategoriesCategoryIdPutPathParams {
            /// カテゴリのID
                pub category_id: String,
    }


      
      
      
      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct PrivateItemsPrivateItemIdDeletePathParams {
            /// 私物のID
                pub private_item_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct PrivateItemsPrivateItemIdGetPathParams {
            /// 私物のID
                pub private_item_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct PrivateItemsPrivateItemIdPutPathParams {
            /// 私物のID
                pub private_item_id: String,
    }


      
      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ProductsProductIdDeletePathParams {
            /// 製品のID
                pub product_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ProductsProductIdGetPathParams {
            /// 製品のID
                pub product_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ProductsProductIdPutPathParams {
            /// 製品のID
                pub product_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct PublicItemsGetQueryParams {
            /// ソート対象のフィールド名
            /// Note: inline enums are not fully supported by openapi-generator
                #[serde(rename = "sort")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub sort: Option<String>,
            /// フィルター対象のフィールド名
            /// Note: inline enums are not fully supported by openapi-generator
                #[serde(rename = "filter")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub filter: Option<String>,
            /// 検索対象のフィールド名
            /// Note: inline enums are not fully supported by openapi-generator
                #[serde(rename = "search")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub search: Option<String>,
    }

      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct PublicItemsPublicItemIdDeletePathParams {
            /// 備品のID
                pub public_item_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct PublicItemsPublicItemIdGetPathParams {
            /// 備品のID
                pub public_item_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct PublicItemsPublicItemIdPutPathParams {
            /// 備品のID
                pub public_item_id: String,
    }


      
      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct UsersUserIdDeletePathParams {
            /// 部員のID
                pub user_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct UsersUserIdGetPathParams {
            /// 部員のID
                pub user_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct UsersUserIdPutPathParams {
            /// 部員のID
                pub user_id: String,
    }




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Category {
/// カテゴリのユニークID
    #[serde(rename = "category_id")]
    pub category_id: String,

/// カテゴリ名
    #[serde(rename = "name")]
    pub name: String,

/// 備考欄
    #[serde(rename = "remarks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remarks: Option<String>,

}


impl Category {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(category_id: String, name: String, ) -> Category {
        Category {
            category_id,
            name,
            remarks: None,
        }
    }
}

/// Converts the Category value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("category_id".to_string()),
            Some(self.category_id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            self.remarks.as_ref().map(|remarks| {
                [
                    "remarks".to_string(),
                    remarks.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Category value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub category_id: Vec<String>,
            pub name: Vec<String>,
            pub remarks: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Category".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "category_id" => intermediate_rep.category_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "remarks" => intermediate_rep.remarks.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Category".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Category {
            category_id: intermediate_rep.category_id.into_iter().next().ok_or_else(|| "category_id missing in Category".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in Category".to_string())?,
            remarks: intermediate_rep.remarks.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Category> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Category>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Category>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Category - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Category> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Category as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Category - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PrivateItem {
/// 私物のユニークID
    #[serde(rename = "private_item_id")]
    pub private_item_id: String,

/// 製品名
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

/// 所有者ID
    #[serde(rename = "owner_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_id: Option<String>,

/// 卒業後の処理ID
    #[serde(rename = "post_grad_treat_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub post_grad_treat_id: Option<String>,

/// 型番
    #[serde(rename = "model_number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub model_number: Option<String>,

/// 現存しているか
    #[serde(rename = "is_remaining")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_remaining: Option<bool>,

/// 備考欄
    #[serde(rename = "remarks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remarks: Option<String>,

}


impl PrivateItem {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(private_item_id: String, ) -> PrivateItem {
        PrivateItem {
            private_item_id,
            name: None,
            owner_id: None,
            post_grad_treat_id: None,
            model_number: None,
            is_remaining: None,
            remarks: None,
        }
    }
}

/// Converts the PrivateItem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PrivateItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("private_item_id".to_string()),
            Some(self.private_item_id.to_string()),


            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),


            self.owner_id.as_ref().map(|owner_id| {
                [
                    "owner_id".to_string(),
                    owner_id.to_string(),
                ].join(",")
            }),


            self.post_grad_treat_id.as_ref().map(|post_grad_treat_id| {
                [
                    "post_grad_treat_id".to_string(),
                    post_grad_treat_id.to_string(),
                ].join(",")
            }),


            self.model_number.as_ref().map(|model_number| {
                [
                    "model_number".to_string(),
                    model_number.to_string(),
                ].join(",")
            }),


            self.is_remaining.as_ref().map(|is_remaining| {
                [
                    "is_remaining".to_string(),
                    is_remaining.to_string(),
                ].join(",")
            }),


            self.remarks.as_ref().map(|remarks| {
                [
                    "remarks".to_string(),
                    remarks.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PrivateItem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PrivateItem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub private_item_id: Vec<String>,
            pub name: Vec<String>,
            pub owner_id: Vec<String>,
            pub post_grad_treat_id: Vec<String>,
            pub model_number: Vec<String>,
            pub is_remaining: Vec<bool>,
            pub remarks: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PrivateItem".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "private_item_id" => intermediate_rep.private_item_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "owner_id" => intermediate_rep.owner_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "post_grad_treat_id" => intermediate_rep.post_grad_treat_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "model_number" => intermediate_rep.model_number.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_remaining" => intermediate_rep.is_remaining.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "remarks" => intermediate_rep.remarks.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PrivateItem".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PrivateItem {
            private_item_id: intermediate_rep.private_item_id.into_iter().next().ok_or_else(|| "private_item_id missing in PrivateItem".to_string())?,
            name: intermediate_rep.name.into_iter().next(),
            owner_id: intermediate_rep.owner_id.into_iter().next(),
            post_grad_treat_id: intermediate_rep.post_grad_treat_id.into_iter().next(),
            model_number: intermediate_rep.model_number.into_iter().next(),
            is_remaining: intermediate_rep.is_remaining.into_iter().next(),
            remarks: intermediate_rep.remarks.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PrivateItem> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PrivateItem>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PrivateItem>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PrivateItem - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PrivateItem> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PrivateItem as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PrivateItem - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Product {
/// 製品のユニークID
    #[serde(rename = "product_id")]
    pub product_id: String,

/// 製品名
    #[serde(rename = "name")]
    pub name: String,

/// 型番
    #[serde(rename = "model_number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub model_number: Option<String>,

/// 商品のURL
    #[serde(rename = "product_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_url: Option<String>,

    #[serde(rename = "categiries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub categiries: Option<Vec<models::Category>>,

    #[serde(rename = "main_users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub main_users: Option<Vec<models::User>>,

/// 備考欄
    #[serde(rename = "remarks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remarks: Option<String>,

}


impl Product {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(product_id: String, name: String, ) -> Product {
        Product {
            product_id,
            name,
            model_number: None,
            product_url: None,
            categiries: None,
            main_users: None,
            remarks: None,
        }
    }
}

/// Converts the Product value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("product_id".to_string()),
            Some(self.product_id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            self.model_number.as_ref().map(|model_number| {
                [
                    "model_number".to_string(),
                    model_number.to_string(),
                ].join(",")
            }),


            self.product_url.as_ref().map(|product_url| {
                [
                    "product_url".to_string(),
                    product_url.to_string(),
                ].join(",")
            }),

            // Skipping categiries in query parameter serialization

            // Skipping main_users in query parameter serialization


            self.remarks.as_ref().map(|remarks| {
                [
                    "remarks".to_string(),
                    remarks.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Product value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Product {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub product_id: Vec<String>,
            pub name: Vec<String>,
            pub model_number: Vec<String>,
            pub product_url: Vec<String>,
            pub categiries: Vec<Vec<models::Category>>,
            pub main_users: Vec<Vec<models::User>>,
            pub remarks: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Product".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "product_id" => intermediate_rep.product_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "model_number" => intermediate_rep.model_number.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "product_url" => intermediate_rep.product_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "categiries" => return std::result::Result::Err("Parsing a container in this style is not supported in Product".to_string()),
                    "main_users" => return std::result::Result::Err("Parsing a container in this style is not supported in Product".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "remarks" => intermediate_rep.remarks.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Product".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Product {
            product_id: intermediate_rep.product_id.into_iter().next().ok_or_else(|| "product_id missing in Product".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in Product".to_string())?,
            model_number: intermediate_rep.model_number.into_iter().next(),
            product_url: intermediate_rep.product_url.into_iter().next(),
            categiries: intermediate_rep.categiries.into_iter().next(),
            main_users: intermediate_rep.main_users.into_iter().next(),
            remarks: intermediate_rep.remarks.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Product> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Product>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Product>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Product - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Product> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Product as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Product - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PublicItem {
/// 備品のユニークID
    #[serde(rename = "public_item_id")]
    pub public_item_id: String,

/// 備品名
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "category")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub category: Option<models::Category>,

/// 備品の購入コスト
    #[serde(rename = "cost")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cost: Option<i32>,

/// 承認日
    #[serde(rename = "approval_date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub approval_date: Option<chrono::naive::NaiveDate>,

/// 耐用期限
    #[serde(rename = "expiration_date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expiration_date: Option<chrono::naive::NaiveDate>,

/// 現存しているか
    #[serde(rename = "is_remaining")]
    pub is_remaining: bool,

    #[serde(rename = "main_user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub main_user: Option<models::User>,

/// 備考欄
    #[serde(rename = "remarks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remarks: Option<String>,

}


impl PublicItem {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(public_item_id: String, name: String, is_remaining: bool, ) -> PublicItem {
        PublicItem {
            public_item_id,
            name,
            category: None,
            cost: None,
            approval_date: None,
            expiration_date: None,
            is_remaining,
            main_user: None,
            remarks: None,
        }
    }
}

/// Converts the PublicItem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PublicItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("public_item_id".to_string()),
            Some(self.public_item_id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),

            // Skipping category in query parameter serialization


            self.cost.as_ref().map(|cost| {
                [
                    "cost".to_string(),
                    cost.to_string(),
                ].join(",")
            }),

            // Skipping approval_date in query parameter serialization

            // Skipping expiration_date in query parameter serialization


            Some("is_remaining".to_string()),
            Some(self.is_remaining.to_string()),

            // Skipping main_user in query parameter serialization


            self.remarks.as_ref().map(|remarks| {
                [
                    "remarks".to_string(),
                    remarks.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PublicItem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PublicItem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub public_item_id: Vec<String>,
            pub name: Vec<String>,
            pub category: Vec<models::Category>,
            pub cost: Vec<i32>,
            pub approval_date: Vec<chrono::naive::NaiveDate>,
            pub expiration_date: Vec<chrono::naive::NaiveDate>,
            pub is_remaining: Vec<bool>,
            pub main_user: Vec<models::User>,
            pub remarks: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PublicItem".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "public_item_id" => intermediate_rep.public_item_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "category" => intermediate_rep.category.push(<models::Category as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cost" => intermediate_rep.cost.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "approval_date" => intermediate_rep.approval_date.push(<chrono::naive::NaiveDate as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "expiration_date" => intermediate_rep.expiration_date.push(<chrono::naive::NaiveDate as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_remaining" => intermediate_rep.is_remaining.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "main_user" => intermediate_rep.main_user.push(<models::User as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "remarks" => intermediate_rep.remarks.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PublicItem".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PublicItem {
            public_item_id: intermediate_rep.public_item_id.into_iter().next().ok_or_else(|| "public_item_id missing in PublicItem".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in PublicItem".to_string())?,
            category: intermediate_rep.category.into_iter().next(),
            cost: intermediate_rep.cost.into_iter().next(),
            approval_date: intermediate_rep.approval_date.into_iter().next(),
            expiration_date: intermediate_rep.expiration_date.into_iter().next(),
            is_remaining: intermediate_rep.is_remaining.into_iter().next().ok_or_else(|| "is_remaining missing in PublicItem".to_string())?,
            main_user: intermediate_rep.main_user.into_iter().next(),
            remarks: intermediate_rep.remarks.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PublicItem> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PublicItem>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PublicItem>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PublicItem - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PublicItem> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PublicItem as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PublicItem - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PublicItemDetails {
/// 備品のユニークID
    #[serde(rename = "public_item_id")]
    pub public_item_id: String,

/// 備品名
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "product")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub product: Option<models::Product>,

/// 備品の購入コスト
    #[serde(rename = "cost")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cost: Option<i32>,

/// 導入日
    #[serde(rename = "purchase_date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub purchase_date: Option<chrono::naive::NaiveDate>,

/// 耐用期限
    #[serde(rename = "expiration_date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expiration_date: Option<chrono::naive::NaiveDate>,

/// 現存しているか
    #[serde(rename = "is_remaining")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_remaining: Option<bool>,

/// 追加元の購入申請ID
    #[serde(rename = "purchase_request_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub purchase_request_id: Option<String>,

/// 備考欄
    #[serde(rename = "remarks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remarks: Option<String>,

}


impl PublicItemDetails {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(public_item_id: String, ) -> PublicItemDetails {
        PublicItemDetails {
            public_item_id,
            name: None,
            product: None,
            cost: None,
            purchase_date: None,
            expiration_date: None,
            is_remaining: None,
            purchase_request_id: None,
            remarks: None,
        }
    }
}

/// Converts the PublicItemDetails value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PublicItemDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("public_item_id".to_string()),
            Some(self.public_item_id.to_string()),


            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),

            // Skipping product in query parameter serialization


            self.cost.as_ref().map(|cost| {
                [
                    "cost".to_string(),
                    cost.to_string(),
                ].join(",")
            }),

            // Skipping purchase_date in query parameter serialization

            // Skipping expiration_date in query parameter serialization


            self.is_remaining.as_ref().map(|is_remaining| {
                [
                    "is_remaining".to_string(),
                    is_remaining.to_string(),
                ].join(",")
            }),


            self.purchase_request_id.as_ref().map(|purchase_request_id| {
                [
                    "purchase_request_id".to_string(),
                    purchase_request_id.to_string(),
                ].join(",")
            }),


            self.remarks.as_ref().map(|remarks| {
                [
                    "remarks".to_string(),
                    remarks.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PublicItemDetails value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PublicItemDetails {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub public_item_id: Vec<String>,
            pub name: Vec<String>,
            pub product: Vec<models::Product>,
            pub cost: Vec<i32>,
            pub purchase_date: Vec<chrono::naive::NaiveDate>,
            pub expiration_date: Vec<chrono::naive::NaiveDate>,
            pub is_remaining: Vec<bool>,
            pub purchase_request_id: Vec<String>,
            pub remarks: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PublicItemDetails".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "public_item_id" => intermediate_rep.public_item_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "product" => intermediate_rep.product.push(<models::Product as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cost" => intermediate_rep.cost.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "purchase_date" => intermediate_rep.purchase_date.push(<chrono::naive::NaiveDate as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "expiration_date" => intermediate_rep.expiration_date.push(<chrono::naive::NaiveDate as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_remaining" => intermediate_rep.is_remaining.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "purchase_request_id" => intermediate_rep.purchase_request_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "remarks" => intermediate_rep.remarks.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PublicItemDetails".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PublicItemDetails {
            public_item_id: intermediate_rep.public_item_id.into_iter().next().ok_or_else(|| "public_item_id missing in PublicItemDetails".to_string())?,
            name: intermediate_rep.name.into_iter().next(),
            product: intermediate_rep.product.into_iter().next(),
            cost: intermediate_rep.cost.into_iter().next(),
            purchase_date: intermediate_rep.purchase_date.into_iter().next(),
            expiration_date: intermediate_rep.expiration_date.into_iter().next(),
            is_remaining: intermediate_rep.is_remaining.into_iter().next(),
            purchase_request_id: intermediate_rep.purchase_request_id.into_iter().next(),
            remarks: intermediate_rep.remarks.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PublicItemDetails> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PublicItemDetails>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PublicItemDetails>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PublicItemDetails - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PublicItemDetails> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PublicItemDetails as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PublicItemDetails - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PublicItemEntry {
/// 備品名
    #[serde(rename = "name")]
    pub name: String,

/// 備品の購入コスト(NULLなら申請から)
    #[serde(rename = "cost")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cost: Option<i32>,

/// 導入日(NULLなら現時刻)
    #[serde(rename = "purchase_date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub purchase_date: Option<chrono::naive::NaiveDate>,

/// 耐用期限
    #[serde(rename = "expiration_date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expiration_date: Option<chrono::naive::NaiveDate>,

/// 現存しているか(NULLなら現存)
    #[serde(rename = "is_remaining")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_remaining: Option<bool>,

/// 追加元の購入申請ID
    #[serde(rename = "purchase_request_id")]
    pub purchase_request_id: String,

/// 備考欄
    #[serde(rename = "remarks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remarks: Option<String>,

}


impl PublicItemEntry {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(name: String, purchase_request_id: String, ) -> PublicItemEntry {
        PublicItemEntry {
            name,
            cost: None,
            purchase_date: None,
            expiration_date: None,
            is_remaining: None,
            purchase_request_id,
            remarks: None,
        }
    }
}

/// Converts the PublicItemEntry value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PublicItemEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("name".to_string()),
            Some(self.name.to_string()),


            self.cost.as_ref().map(|cost| {
                [
                    "cost".to_string(),
                    cost.to_string(),
                ].join(",")
            }),

            // Skipping purchase_date in query parameter serialization

            // Skipping expiration_date in query parameter serialization


            self.is_remaining.as_ref().map(|is_remaining| {
                [
                    "is_remaining".to_string(),
                    is_remaining.to_string(),
                ].join(",")
            }),


            Some("purchase_request_id".to_string()),
            Some(self.purchase_request_id.to_string()),


            self.remarks.as_ref().map(|remarks| {
                [
                    "remarks".to_string(),
                    remarks.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PublicItemEntry value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PublicItemEntry {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub cost: Vec<i32>,
            pub purchase_date: Vec<chrono::naive::NaiveDate>,
            pub expiration_date: Vec<chrono::naive::NaiveDate>,
            pub is_remaining: Vec<bool>,
            pub purchase_request_id: Vec<String>,
            pub remarks: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PublicItemEntry".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cost" => intermediate_rep.cost.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "purchase_date" => intermediate_rep.purchase_date.push(<chrono::naive::NaiveDate as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "expiration_date" => intermediate_rep.expiration_date.push(<chrono::naive::NaiveDate as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_remaining" => intermediate_rep.is_remaining.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "purchase_request_id" => intermediate_rep.purchase_request_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "remarks" => intermediate_rep.remarks.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PublicItemEntry".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PublicItemEntry {
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in PublicItemEntry".to_string())?,
            cost: intermediate_rep.cost.into_iter().next(),
            purchase_date: intermediate_rep.purchase_date.into_iter().next(),
            expiration_date: intermediate_rep.expiration_date.into_iter().next(),
            is_remaining: intermediate_rep.is_remaining.into_iter().next(),
            purchase_request_id: intermediate_rep.purchase_request_id.into_iter().next().ok_or_else(|| "purchase_request_id missing in PublicItemEntry".to_string())?,
            remarks: intermediate_rep.remarks.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PublicItemEntry> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PublicItemEntry>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PublicItemEntry>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PublicItemEntry - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PublicItemEntry> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PublicItemEntry as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PublicItemEntry - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct User {
/// ユーザーのユニークID
    #[serde(rename = "user_id")]
    pub user_id: String,

/// ユーザーのハンドルネーム
    #[serde(rename = "handle_name")]
    pub handle_name: String,

/// ユーザーのスクリーンネーム
    #[serde(rename = "screen_name")]
    pub screen_name: String,

/// ユーザーのSlack ID
    #[serde(rename = "slack_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slack_id: Option<String>,

/// 管理者フラグ
    #[serde(rename = "is_admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_admin: Option<bool>,

/// 在籍状況
    #[serde(rename = "is_member")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_member: Option<bool>,

/// 卒業日
    #[serde(rename = "graduation_date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub graduation_date: Option<chrono::naive::NaiveDate>,

/// 備考欄
    #[serde(rename = "remarks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remarks: Option<String>,

}


impl User {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(user_id: String, handle_name: String, screen_name: String, ) -> User {
        User {
            user_id,
            handle_name,
            screen_name,
            slack_id: None,
            is_admin: None,
            is_member: None,
            graduation_date: None,
            remarks: None,
        }
    }
}

/// Converts the User value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("user_id".to_string()),
            Some(self.user_id.to_string()),


            Some("handle_name".to_string()),
            Some(self.handle_name.to_string()),


            Some("screen_name".to_string()),
            Some(self.screen_name.to_string()),


            self.slack_id.as_ref().map(|slack_id| {
                [
                    "slack_id".to_string(),
                    slack_id.to_string(),
                ].join(",")
            }),


            self.is_admin.as_ref().map(|is_admin| {
                [
                    "is_admin".to_string(),
                    is_admin.to_string(),
                ].join(",")
            }),


            self.is_member.as_ref().map(|is_member| {
                [
                    "is_member".to_string(),
                    is_member.to_string(),
                ].join(",")
            }),

            // Skipping graduation_date in query parameter serialization


            self.remarks.as_ref().map(|remarks| {
                [
                    "remarks".to_string(),
                    remarks.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a User value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for User {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub user_id: Vec<String>,
            pub handle_name: Vec<String>,
            pub screen_name: Vec<String>,
            pub slack_id: Vec<String>,
            pub is_admin: Vec<bool>,
            pub is_member: Vec<bool>,
            pub graduation_date: Vec<chrono::naive::NaiveDate>,
            pub remarks: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing User".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "user_id" => intermediate_rep.user_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "handle_name" => intermediate_rep.handle_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "screen_name" => intermediate_rep.screen_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "slack_id" => intermediate_rep.slack_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_admin" => intermediate_rep.is_admin.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_member" => intermediate_rep.is_member.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "graduation_date" => intermediate_rep.graduation_date.push(<chrono::naive::NaiveDate as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "remarks" => intermediate_rep.remarks.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing User".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(User {
            user_id: intermediate_rep.user_id.into_iter().next().ok_or_else(|| "user_id missing in User".to_string())?,
            handle_name: intermediate_rep.handle_name.into_iter().next().ok_or_else(|| "handle_name missing in User".to_string())?,
            screen_name: intermediate_rep.screen_name.into_iter().next().ok_or_else(|| "screen_name missing in User".to_string())?,
            slack_id: intermediate_rep.slack_id.into_iter().next(),
            is_admin: intermediate_rep.is_admin.into_iter().next(),
            is_member: intermediate_rep.is_member.into_iter().next(),
            graduation_date: intermediate_rep.graduation_date.into_iter().next(),
            remarks: intermediate_rep.remarks.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<User> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<User>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<User>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for User - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<User> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <User as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into User - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}



