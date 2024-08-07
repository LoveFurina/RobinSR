use rand::Rng;

use crate::net::tools::{self, BattleType, Monster};

use super::*;

pub async fn on_start_cocoon_stage_cs_req(
    session: &mut PlayerSession,
    body: &StartCocoonStageCsReq,
) -> Result<()> {
    let player = tools::JsonData::load().await;

    let mut battle_info = SceneBattleInfo {
        stage_id: player.battle_config.stage_id,
        logic_random_seed: rand::thread_rng().gen::<u32>(),
        battle_id: 1,
        cycle_count: player.battle_config.cycle_count, // wave
        ..Default::default()
    };

    // avatars
    for i in 0..4 {
        let avatar_id = &player.lineups.get(&i).unwrap_or(&0);
        if **avatar_id == 0 {
            continue;
        }
        if let Some(avatar) = player.avatars.get(avatar_id) {
            let (battle_avatar, techs) = avatar.to_battle_avatar_proto(
                i,
                player
                    .lightcones
                    .iter()
                    .find(|v| v.equip_avatar == avatar.avatar_id),
                player
                    .relics
                    .iter()
                    .filter(|v| v.equip_avatar == avatar.avatar_id)
                    .collect::<Vec<_>>(),
            );
            for tech in techs {
                battle_info.buff_list.push(tech);
            }
            battle_info.battle_avatar_list.push(battle_avatar);
        };
    }

    // custom stats for avatars
    for stat in &player.battle_config.custom_stats {
        for avatar in &mut battle_info.battle_avatar_list {
            if avatar.relic_list.is_empty() {
                avatar.relic_list.push(BattleRelic {
                    id: 61011,
                    main_affix_id: 1,
                    level: 1,
                    ..Default::default()
                })
            }

            if let Some(sub_affix) = avatar.relic_list[0]
                .sub_affix_list
                .iter_mut()
                .find(|v| v.affix_id == stat.sub_affix_id)
            {
                sub_affix.cnt = stat.count;
            } else {
                avatar.relic_list[0].sub_affix_list.push(RelicAffix {
                    affix_id: stat.sub_affix_id,
                    cnt: stat.count,
                    step: stat.step,
                })
            }
        }
    }

    // blessings
    for blessing in &player.battle_config.blessings {
        let mut buffs = BattleBuff {
            id: blessing.id,
            level: blessing.level,
            wave_flag: 0xffffffff,
            owner_index: 0xffffffff,
            ..Default::default()
        };
        if let Some(dynamic_key) = &blessing.dynamic_key {
            buffs
                .dynamic_values
                .insert(dynamic_key.key.clone(), dynamic_key.value as f32);
        };
        battle_info.buff_list.push(buffs);
    }

    // pf score object
    if player.battle_config.battle_type == BattleType::PF {
        let mut battle_target = BattleTargetList::default();
        battle_target.knbbhojnoff.push(BattleTarget {
            id: 10001,
            progress: 0,
            ..Default::default()
        });

        battle_info.battle_target_info.insert(1, battle_target);
        for i in 2..=4 {
            battle_info
                .battle_target_info
                .insert(i, BattleTargetList::default());
        }
        battle_info.battle_target_info.insert(
            5,
            BattleTargetList {
                knbbhojnoff: vec![
                    BattleTarget {
                        id: 2001,
                        progress: 0,
                        ..Default::default()
                    },
                    BattleTarget {
                        id: 2002,
                        progress: 0,
                        ..Default::default()
                    },
                ],
            },
        );
    }

    // Apocalyptic Shadow
    if player.battle_config.battle_type == BattleType::AS {
        let battle_target = BattleTargetList {
            knbbhojnoff: vec![BattleTarget {
                id: 90005,
                progress: 0,
                ..Default::default()
            }],
        };
        battle_info.battle_target_info.insert(1, battle_target);
    }

    //  SU
    if player.battle_config.battle_type == BattleType::SU {
        battle_info.apkpbammnhm.push(Pldmljcickk {
            ghhnoefibke: player.battle_config.path_resonance_id,
            status: Some(Chcjbneicfa {
                sp: Some(SpProgress {
                    sp_cur: 10_000,
                    sp_need: 10_000,
                }),
            }),
            ..Default::default()
        })
    }

    // monsters
    battle_info.monster_wave_list = Monster::to_scene_monster_waves(&player.battle_config.monsters);

    let rsp = StartCocoonStageScRsp {
        retcode: 0,
        prop_entity_id: body.prop_entity_id,
        cocoon_id: body.cocoon_id,
        wave: body.wave,
        battle_info: Some(battle_info),
    };

    session.send(CMD_START_COCOON_STAGE_SC_RSP, rsp).await
}

pub async fn on_pve_battle_result_cs_req(
    session: &mut PlayerSession,
    body: &PveBattleResultCsReq,
) -> Result<()> {
    session
        .send(
            CMD_P_V_E_BATTLE_RESULT_SC_RSP,
            PveBattleResultScRsp {
                retcode: 0,
                end_status: body.end_status,
                battle_id: body.battle_id,
                ..Default::default()
            },
        )
        .await
}
