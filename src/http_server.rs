use std::net::SocketAddr;
use std::sync::Arc;

use dioxus_liveview::LiveViewPool;

use my_nosql_contracts::DefaultsNoSqlEntity;
use rust_extensions::slice_of_u8_utils::SliceOfU8Ext;
use salvo::prelude::*;

use crate::app::MyNoSqlReaders;
use crate::APP_CTX;

#[handler]
pub fn index(res: &mut Response) {
    let addr: SocketAddr = ([127, 0, 0, 1], 9001).into();
    res.with_header("Content-Type", "text/html; charset=uft-8", true)
        .unwrap();

    res.write_body(super::static_resources::get_html(addr).into_bytes())
        .unwrap();
}

#[handler]
pub async fn connect(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), StatusError> {
    let view = depot.obtain::<Arc<LiveViewPool>>().unwrap().clone();

    WebSocketUpgrade::new()
        .upgrade(req, res, |ws| async move {
            _ = view
                .launch(dioxus_liveview::salvo_socket(ws), crate::app)
                .await;
        })
        .await
}

#[handler]
pub async fn get_avatar(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let readers = APP_CTX.get_my_no_sql_readers().await;

    let params = req.params();

    let id = params.get("**path");

    if id.is_none() {
        res.set_status_code(StatusCode::NOT_FOUND);
        res.write_body("Avatar not found".as_bytes().to_vec())
            .unwrap();
        return;
    }

    let id = id.unwrap().trim();

    if id == "" {
        res.set_status_code(StatusCode::NOT_FOUND);
        res.write_body("Avatar not found".as_bytes().to_vec())
            .unwrap();
        return;
    }

    let avatar = readers.instrument_avatars.get_entity(id, "SVG").await;

    match avatar {
        Some(avatar) => match base64::decode(&avatar.avatar) {
            Ok(content) => {
                res.add_header("Content-Type", "image/svg+xml", true)
                    .unwrap();
                res.write_body(content).unwrap();
                return;
            }
            Err(err) => {
                res.set_status_code(StatusCode::NOT_FOUND);
                res.write_body(format!("Base64 decode error: {}", err).into_bytes())
                    .unwrap();
                return;
            }
        },
        None => match get_default_svg(&readers).await {
            Some(content) => {
                let src = &content[..256];

                if src.find_sequence_pos("<svg".as_bytes(), 0).is_none() {
                    res.add_header("Content-Type", "image/png", true).unwrap();
                } else {
                    res.add_header("Content-Type", "image/svg+xml", true)
                        .unwrap();
                }

                res.write_body(content).unwrap();
                return;
            }
            None => {
                res.set_status_code(StatusCode::NOT_FOUND);
                res.write_body("Avatar not found".as_bytes().to_vec())
                    .unwrap();
                return;
            }
        },
    }
}

async fn get_default_svg(readers: &MyNoSqlReaders) -> Option<Vec<u8>> {
    let item = readers
        .defaults
        .get_entity(
            DefaultsNoSqlEntity::generate_partition_key(),
            "TradingInstrumentAvatarSvg",
        )
        .await?;

    let value = item.value.as_ref()?;
    base64::decode(value).ok()
}
