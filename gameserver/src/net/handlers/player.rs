use super::*;
use crate::util;
use prost::Message;

pub async fn on_player_heart_beat_cs_req(
    session: &mut PlayerSession,
    body: &PlayerHeartBeatCsReq,
) -> Result<()> {
    session
        .send(
            CMD_PLAYER_HEART_BEAT_SC_RSP,
            PlayerHeartBeatScRsp {
                retcode: 0,
                client_time_ms: body.client_time_ms,
                server_time_ms: util::cur_timestamp_ms(),
                download_data: Some(
                    ClientDownloadData::decode(prost::bytes::Bytes::from(rbase64::decode("CDMQuQoa1gFDUy5Vbml0eUVuZ2luZS5HYW1lT2JqZWN0LkZpbmQoIlVJUm9vdC9BYm92ZURpYWxvZy9CZXRhSGludERpYWxvZyhDbG9uZSkiKTpHZXRDb21wb25lbnRJbkNoaWxkcmVuKHR5cGVvZihDUy5SUEcuQ2xpZW50LkxvY2FsaXplZFRleHQpKS50ZXh0ID0gIkxpbmdzaGFTUiBpcyBhIGZyZWUgYW5kIG9wZW4gc291cmNlIHNvZnR3YXJlLiBkaXNjb3JkLmdnL3JldmVyc2Vkcm9vbXMi")?)).unwrap(),
                ),
            },
        )
        .await
}
