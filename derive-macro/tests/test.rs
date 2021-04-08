#[macro_use]
extern crate derive_macro;

use std::borrow::Borrow;
use regex::Regex;

#[derive(Describe)]
struct MyStruct {
    my_string: String,
    my_enum: MyEnum,
    my_number: f64,
}

#[derive(Describe)]
struct MyTupleStruct(u32, String, i8);

#[derive(Describe)]
enum MyEnum {
    VariantA,
    VariantB,
}

#[derive(Describe)]
union MyUnion {
    unsigned: u32,
    signed: i32,
}

#[derive(Describe)]
struct MyStruct2 {
    my_string: &'static str,
    my_enum: MyEnum,
    my_number: f64,
}

// pub trait EndPoint {
//     fn get_endpoint(&self) -> String;
//     // fn get_query_string(&self) -> String;
// }
//
// impl<'a> EndPoint for AbcRequest<'a> {
//     fn get_endpoint(&self) -> String {
//         let re = Regex::new(r"\{\w+}").unwrap();
//         let iter = re.find_iter(self.endpoint);
//         let mut after = self.endpoint.to_string();
//         for m in iter {
//             match m.as_str() {
//                 "{id}" => after = after.replace(m.as_str(), self.id.to_string().as_str()),
//                 _ => panic!("不能替换")
//             }
//         }
//         after
//     }
// }


// #[derive(Endpoint, Debug)]
// struct AbcRequest<'a> {
//     #[method(GET)]
//     pub method: &'a str,
//     #[endpoint("/group/{id}/access_requests/{ddd}")]
//     pub endpoint: &'a str,
//     pub id: &'a str,
//     pub ddd: &'a str,
//
//     #[query(30)]
//     pub access_level: Option<i32>,
//     #[query("caiwenhui")]
//     pub name: Option<&'a str>,
//     #[query(None)]
//     pub name2: Option<&'a str>,
// }

#[derive(Endpoint, Debug)]
struct AbcRequest<'a> {
    #[method(GET)]
    pub method: &'a str,
    #[endpoint("/group/{id}/access_requests")]
    pub endpoint: &'a str,
    pub id: &'a str,
}



// struct ARequest<'a> {
//     // #[method(GET)]
//     method: &'a str,
//     // #[endpoint(/group/{id}/access_requests)]
//     endpoint: &'a str,
//     id: i32,
// }
//
#[test]
fn test_derive_endpoint() {
    println!("{:?}", AbcRequest::new("456").get_endpoint());
    // println!("{:?}", AbcRequest::new(456, "nihao").get_query_fields());
    // let mut a = AbcRequest::new(456, "nihao");
    // println!("{:?}", a.get_query());
    // a.name = Some("name-level2");
    // println!("{:?}", a.get_query());
}

#[test]
fn test_derive_a() {
    MyStruct::describe();
    MyStruct2::describe();
    MyTupleStruct::describe();
    MyEnum::describe();
    MyUnion::describe();
}


#[test]
fn test_match() {
    let re = Regex::new(r"\{(?P<ident>\w+)}").unwrap();
    let before = "/group/{id}/access_requests/{ids}/";
    // let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    println!("{}", 123);
    // re.find_iter(before).map(|m| before = before.replace(m.as_str(), "123").clone().as_str() );
    // let a = before.clone().as_str();
    let iter = re.find_iter(before);
    let mut after = before.to_string();
    let mut i = 0;

    // iter.map(|m| after = after.replace(m.as_str(), i.to_string().as_str()).
    for m in iter {
        // i += 1;
        println!("{}", m.as_str());
        let cap = re.captures(m.as_str()).unwrap();
        let value = cap.name("ident").unwrap();
        println!("{}", value.as_str())

        // before = "1213";
        // after = after.replace(m.as_str(), i.to_string().as_str());
    }

    // let after: Vec<&str> = re.find_iter(before).map(|m| m.as_str()).collect();
    // println!("{}", 1234);
    // println!("{}", after);
    // println!("==============={}", after.len());
    // println!("===={:?}", after);
    // println!("{}", before.replace(after.first().unwrap(), "123"))

    // assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");
}

#[test]
fn test_match2() {
    struct AbcRequest<'a> {
        pub method: Option<&'a str>,
        pub endpoint: Option<&'a str>,
    }

    let a = AbcRequest { method: Some("123"), endpoint: None };

    let mut query = String::new();
    let test1 = ["method", "endpoint"];
    query.push_str("?");
    for name in &test1 {
        match name {
            &"method" => {
                if a.method.is_some() {
                    query.push_str(format!("{k}={v}",
                                           k = *name,
                                           v = a.method.unwrap()
                    ).as_str());
                    query.push_str("&");
                }
            },
            &"endpoint" => {
                if a.endpoint.is_some() {
                    query.push_str(format!("{k}={v}",
                                           k = *name,
                                           v = a.endpoint.unwrap()
                    ).as_str());
                    query.push_str("&");
                }
            },
            _ => panic!("不存在属性")
        }
    }
    query.pop();

    println!("{}", query);
}
