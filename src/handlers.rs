use actix_web::web::{Query, Json};
use actix_web::{web, HttpResponse, Responder};

use eosio_client_api::json_rpc::{AbiTrio, EOSRPC};
use serde::Deserialize;
use std::collections::HashMap;
use eosio_client_api::api_types::AbiStruct;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        "<h1>Account Explorer</h1><h2>System</h2><ul>".to_owned()
            + "<li><a href='/account/eosio'>eosio</a></li>"
            + "<li><a href='/account/eosio.token'>eosio.token</a></li>"

            + "</ul><h2>Regular</h2><ul>"
            +"<li><a href='/account/fwonhjnefmps'>fwonhjnefmps(Tic Tac Toe)</a></li>"
            + "<li><a href='/account/exoejrppvyew'>exoejrppvyew</a></li>"
            + "<li><a href='/account/wmftggomuzsi'>wmftggomuzsi</a></li>"
            + "</ul>"
        +"<h2>Resource Market</h2><ul>"
            +"<li><a href='/account/eosio/rows/eosio.reserv/userres'>eosio.reserv - userres</a></li>"
            +"<li><a href='/account/eosio/rows/eosio/rammarket'>eosio - ram market</a></li>"
        +"</ul>",
    )
}
pub async fn action_detail_post(info: web::Path<(String,String)>/*,params: Json<HashMap<String,String>>*/) -> impl Responder {
    println!("{:?}",info);
   // println!("{:?}",params);
    HttpResponse::Ok().content_type("text/html").body(
        "<h1>Account Explorer</h1><ul><li><a href='/account/eosio'>eosio</a></li>".to_owned()
            + "<li><a href='/account/fwonhjnefmps'>fwonhjnefmps(Tic Tac Toe)</a></li>"
            + "<li><a href='/account/exoejrppvyew'>exoejrppvyew</a></li>"
            + "<li><a href='/account/wmftggomuzsi'>wmftggomuzsi</a></li>"
            + "</ul>",
    )
}

pub async fn account_detail(info: web::Path<String>) -> impl Responder {
    let host = "http://tempest.local:8888";
    let acc = String::from(info.as_str());
    match EOSRPC::non_blocking(String::from(host)).await {
        Ok(eos) => match eos.get_abi(&acc).await {
            Ok(get_abi) => match get_abi.abi {
                Some(abi) => {
                    let action_str = abi
                        .actions
                        .iter()
                        .map(|act| {
                            format!(
                                "<li><a href='/account/{}/action/{}'>{}</a></li>",
                                &acc,
                                &act.name[0..],
                                &act.name[0..]
                            )
                        })
                        .collect::<Vec<_>>();
                    let table_str = abi
                        .tables
                        .iter()
                        .map(|table| {
                            format!(
                                "<li><a href='/account/{}/table/{}'>{}</a></li>",
                                &acc, &table.name, &table.name
                            )
                        })
                        .collect::<Vec<_>>();
                    let types_str = abi
                        .types
                        .iter()
                        .map(|type_def| {
                            format!(
                                "<li>{} {}</li>", type_def.abi_type, type_def.new_type_name
                            )
                        })
                        .collect::<Vec<_>>();

                    HttpResponse::Ok().content_type("text/html").body(
                        "<a href='/'>Home</a>".to_owned()
                            + &format!("<h1>Account Details: {}</h1>", acc)
                            + "<h2>Actions</h2><ul>"
                            + &action_str.join("\n")
                            + "</ul>"
                            + "<h2>Tables</h2><ul>"
                            + &table_str.join("\n")
                            + "</ul>"
                            + "<h2>Types</h2><ul>"
                            + &types_str.join("\n")
                            + "</ul>",
                    )
                }
                None => HttpResponse::NotFound().body("No ABI defined for account"),
            },
            Err(_e) => HttpResponse::NotFound().body(acc),
        },
        Err(e) => HttpResponse::InternalServerError().body(String::from(e.description())),
    }
}

pub async fn table_detail(params: web::Path<(String, String)>) -> impl Responder {
    let host = "http://tempest.local:8888";
    let acc = String::from(params.0.as_str());
    let table_name = String::from(params.1.as_str());
    match EOSRPC::non_blocking(String::from(host)).await {
        Ok(eos) => {
            match eos
                .get_table_by_scope(&acc, &table_name, "", "", 30, false)
                .await
            {
                Ok(gtbs) => {
                    let rows = gtbs.rows.iter().map(|row| {
                        let sc = if row.scope == "" {
                            "0"
                        } else {
                            &row.scope
                        };
                        format!("<tr><td>{}</td><td><a href='/account/{}/rows/{}/{}'>{}</a></td><td>{}</td><td>{}</td><td>{}</td></tr>",
                                sc, row.code, sc, row.table, row.table, row.payer, row.code, row.count)
                    }).collect::<Vec<_>>();
                    HttpResponse::Ok().content_type("text/html").body(
                        "<a href='/'>Home</a><h1>Table Details</h1>".to_owned()
                            + &format!("Account:<a href='/account/{}'>{}</a>", acc, acc)
                            + "<table>"
                            + "<tr><th>Scope</th><th>Table</th><th>Payer</th><th>Code</th><th>Count</th></tr>"
                            + &rows.join("") +
                            "</table>"
                    )
                }
                Err(_e) => HttpResponse::NotFound().body(acc),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(String::from(e.description())),
    }
}

fn dump_struct(structs: &HashMap<String, AbiStruct>, name: &str, as_form: bool) -> String {
    dump_struct_int(&structs, name, "", as_form, 0)
}

fn dump_struct_int(structs: &HashMap<String, AbiStruct>, name: &str, prefix: &str, as_form: bool, level: usize) -> String {
    if level > 5 {
        return "".to_owned();
    } else {
        match structs.get(name) {
            Some(st) => {
                let mut resp = String::new();
                resp.push_str(name);
                resp.push_str("<ul>");
                if !st.base.is_empty() {
                    resp.push_str(&dump_struct_int(structs, &st.base, &format!("{}{}.base.", prefix, name), as_form, level + 1));
                }
                for f in &st.fields {
                    if f.name.is_empty() {
                        resp.push_str(&dump_struct_int(structs, &f.abi_type, &format!("{}{}.", prefix, name), as_form, level + 1));
                    } else {
                        resp.push_str(&dump_struct_int(structs, &f.abi_type, &format!("{}{}.{}.", prefix, name, &f.name), as_form, level + 1));
                    }
                }

                resp.push_str("</ul>");
                resp
            }
            None => {
                // base types
                match name {
                    "name" => {
                        format!("<li>{}{} <input name='{}{}' type='text' maxlength='12' size='14' required/></li>", prefix, name, prefix, name)
                    }
                    "string" => {
                        format!("<li>{}{} <input name='{}{}' type='text' ' required/></li>", prefix, name, prefix, name)
                    }  "public_key" => {
                        format!("<li>{}{} <input name='{}{}' type='text' ' required/></li>", prefix, name, prefix, name)
                    }
                    "int64" => {
                        format!("<li>{}{} <input name='{}{}' type='number' required/></li>", prefix, name, prefix, name)
                    }
                    "int64?" => {
                        format!("<li>{}{} <input name='{}int64' type='number'/></li>", prefix, name, prefix)
                    }
                    "int32" => {
                        format!("<li>{}{} <input name='{}{}' type='number' required/></li>", prefix, name, prefix, name)
                    }
                    "int32?" => {
                        format!("<li>{}{} <input name='{}int64' type='number'/></li>", prefix, name, prefix)
                    }
                    "int16" => {
                        format!("<li>{}{} <input name='{}{}' type='number' required/></li>", prefix, name, prefix, name)
                    }
                    "int16?" => {
                        format!("<li>{}{} <input name='{}int64' type='number'/></li>", prefix, name, prefix)
                    }
                    "uint64" => {
                        format!("<li>{}{} <input name='{}{}' type='number' required min='0' /></li>", prefix, name, prefix, name)
                    }
                    "uint64?" => {
                        format!("<li>{}{} <input name='{}int64' type='number' min='0' /></li>", prefix, name, prefix)
                    }
                    "uint32" => {
                        format!("<li>{}{} <input name='{}{}' type='number' required min='0' /></li>", prefix, name, prefix, name)
                    }
                    "uint32?" => {
                        format!("<li>{}{} <input name='{}int64' type='number' min='0' /></li>", prefix, name, prefix)
                    }
                    "uint16" => {
                        format!("<li>{}{} <input name='{}{}' type='number' required min='0' /></li>", prefix, name, prefix, name)
                    }
                    "uint16?" => {
                        format!("<li>{}{} <input name='{}int64' type='number' min='0' /></li>", prefix, name, prefix)
                    }
                    "varuint32" => {
                        format!("<li>{}{} <input name='{}{}' type='number' required min='0' /></li>", prefix, name, prefix, name)
                    }
                    "varuint32?" => {
                        format!("<li>{}{} <input name='{}int64' type='number' min='0' /></li>", prefix, name, prefix)
                    }
                    "asset" => {
                        format!("<li>{}{}.symbol <input name='{}{}.symbol' type='text' maxlength='12' size='14'required /> amount <input name='{}{}.amount' required type='number'/></li>",
                                prefix, name, prefix, name,
                                prefix, name)
                    }
                    "symbol" => {
                        format!("<li>{}{} <input name='{}{}' type='text' maxlength='12' size='14'required /></li>",
                                prefix, name, prefix, name)
                    }
                    "checksum256" => {
                        format!("<li>{}{} <input name='{}{}' type='text' minlength='256' maxlength='256' size='256' required /></li>",
                                prefix, name, prefix, name)
                    }
                    "producer_schedule?" => {
                        format!("<li>{}{} <input name='{}{}' type='text'  /> ..TBD..</li>",
                                prefix, name, prefix, name)
                    }
                    _ => {
                        format!("<li><b>UNKNOWN</b>? {} __{}__</li>", prefix, name)
                    }
                }
            }
        }
    }
}

pub async fn action_detail(params: web::Path<(String, String)>) -> impl Responder {
    let host ="http://tempest.local:8888";
    let acc = String::from(params.0.as_str());
    let action_name = String::from(params.1.as_str());
    match EOSRPC::non_blocking(String::from(host)).await {
        Ok(eos) => {
            match eos
                .get_abi(&acc)
                .await
            {
                Ok(gabi) => {
                    match gabi.abi {
                        Some(abi) => {
                            match abi.actions.iter().find(|p| p.name == action_name) {
                                Some(act) => {
                                    let mut resp: String = "<a href='/'>Home</a><br/>".to_owned();
                                    resp.push_str(&format!("Account <a href='/account/{}'>{}</a>", acc, acc));
                                    resp.push_str(&format!("<h1>Action {}</h1>", action_name));
                                    if !act.ricardian_contract.is_empty() {
                                        resp.push_str(&format!("<h2>Contract</h2><pre>{}</pre>", &act.ricardian_contract));
                                    }
                                    resp.push_str(&format!("<h2>Parameters for:{}</h2>", act.abi_type));
                                    let mut abi_map: HashMap<String, AbiStruct> = HashMap::with_capacity(abi.structs.len());
                                    for f in abi.structs {
                                        abi_map.insert(f.name.clone(), f);
                                    };

                                    if abi_map.contains_key(&act.abi_type) {
                                        resp.push_str("<form method='POST'>");
                                        resp.push_str(&dump_struct(&abi_map, &act.abi_type, true));
                                        resp.push_str("<input type='submit' />");
                                        resp.push_str("</form>");
                                    } else {
                                        resp.push_str("<p>Not Found in ABI</p>")
                                    }

                                    HttpResponse::Ok().content_type("text/html").body(resp)
                                }
                                None => HttpResponse::NotFound().body(format!("Action {} Not Found on Account {}", action_name, acc)),
                            }
                        }
                        None => HttpResponse::NotFound().body(acc),
                    }
                }
                Err(_e) => HttpResponse::NotFound().body(acc),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(String::from(e.description())),
    }
}

#[derive(Deserialize)]
pub struct TableRows {
    next: Option<String>,
    rows: Option<usize>,
}

pub async fn table_rows(
    params: web::Path<(String, String, String)>,
    qry: Query<TableRows>,
) -> impl Responder {
    let host ="http://tempest.local:8888";
    let acc = String::from(params.0.as_str());
    let scope_name = String::from(params.1.as_str());
    let table_name = String::from(params.2.as_str());
    let row_count = qry.rows.unwrap_or(10);
    let next: &str = match &qry.next {
        Some(x) => &x,
        None => "",
    };

    match EOSRPC::non_blocking(String::from(host)).await {
        Ok(eos) => {
            match eos
                .get_table_rows(
                    &acc,
                    &scope_name,
                    &table_name,
                    "",
                    next,
                    "",
                    row_count,
                    "",
                    "",
                    "",
                    false,
                    true,
                )
                .await
            {
                Ok(gtr) => {
                    let get_abi = eos.get_abi(&acc).await.unwrap();
                    let typename = match get_abi.abi {
                        Some(abi_text) => {
                            let res = abi_text
                                .tables
                                .iter()
                                .find(|p| p.name == table_name)
                                .map(|p| &p.abi_type[0..])
                                .unwrap_or(&table_name);
                            res.to_owned()
                        }
                        None => {
                            let res = &table_name[0..];
                            res.to_owned()
                        }
                    };
                    match AbiTrio::create("eosio", &acc, &eos).await {
                        Ok(abi) => {
                            let rows = gtr
                                .rows
                                .iter()
                                .map(|row| {
                                    let p = match &row.payer {
                                        Some(x) => format!(
                                            "<a href='/account/{}'>{}</a>",
                                            &x[0..],
                                            &x[0..]
                                        ),
                                        None => "-".to_owned(),
                                    };
                                    let json = abi
                                        .acct_abi
                                        .hex_to_json(&acc, &typename, row.data.as_bytes())
                                        .unwrap_or("ERR".to_owned());
                                    format!("<tr><td>{}</td><td>{}</td></tr>", &p, json)
                                })
                                .collect::<Vec<_>>();
                            let next = if gtr.more {
                                format!(
                                    "<a href='/account/{}/rows/{}/{}?next={}&rows={}'>Next:{}</a>",
                                    acc,
                                    scope_name,
                                    table_name,
                                    gtr.next_key,
                                    row_count,
                                    gtr.next_key
                                )
                            } else {
                                "".to_owned()
                            };
                            HttpResponse::Ok().content_type("text/html").body(
                                "<a href='/'>Home</a><h1>Table Details</h1>".to_owned()
                                    + &format!("<p><a href='/account/{}'>{}</a>-{}-<a href='/account/{}/table/{}'>{}</a></p>", acc, acc, scope_name, acc, table_name, table_name)
                                    + "<table>"
                                    + "<tr><th>Payer</th><th>Data</th></tr>"
                                    + &rows.join("") +
                                    "</table>" + &next
                            )
                        }
                        Err(e) => {
                            println!("{:?}",e);
                            HttpResponse::InternalServerError().body(String::from(e.description()))
                        }
                    }
                }
                Err(_e) => HttpResponse::NotFound().body(acc),
            }
        }
        Err(e) => {
            println!("{:?}",e);
            HttpResponse::InternalServerError().body(String::from(e.description()))
        },
    }
}
