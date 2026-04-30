use crate::pb::polymarket::v1 as pb;
use substreams_abis::prediction::polymarket::v1::umactfadapter::v3::events;
use substreams_ethereum::pb::eth::v2::Log;
use substreams_ethereum::Event;

pub fn parse_log(log: &Log) -> Result<Option<pb::log::Log>, substreams::errors::Error> {
    // V3 events (same signatures as V2, so these will match both V2 and V3 contract events)

    // AncillaryDataUpdated event
    if let Some(event) = events::AncillaryDataUpdated::match_and_decode(log) {
        let event = pb::log::Log::UmaCtfAdapterAncillaryDataUpdated(
            pb::UmaCtfAdapterAncillaryDataUpdated {
                question_id: event.question_id.to_vec(),
                owner: event.owner.to_vec(),
                update: event.update,
            },
        );
        return Ok(Some(event));
    }

    // NewAdmin event
    if let Some(event) = events::NewAdmin::match_and_decode(log) {
        let event = pb::log::Log::UmaCtfAdapterNewAdmin(pb::UmaCtfAdapterNewAdmin {
            admin: event.admin.to_vec(),
            new_admin_address: event.new_admin_address.to_vec(),
        });
        return Ok(Some(event));
    }

    // QuestionEmergencyResolved event
    if let Some(event) = events::QuestionEmergencyResolved::match_and_decode(log) {
        let event = pb::log::Log::UmaCtfAdapterQuestionEmergencyResolved(
            pb::UmaCtfAdapterQuestionEmergencyResolved {
                question_id: event.question_id.to_vec(),
                payouts: event.payouts.iter().map(|p| p.to_string()).collect(),
            },
        );
        return Ok(Some(event));
    }

    // QuestionFlagged event
    if let Some(event) = events::QuestionFlagged::match_and_decode(log) {
        let event = pb::log::Log::UmaCtfAdapterQuestionFlagged(pb::UmaCtfAdapterQuestionFlagged {
            question_id: event.question_id.to_vec(),
        });
        return Ok(Some(event));
    }

    // QuestionInitialized event
    if let Some(event) = events::QuestionInitialized::match_and_decode(log) {
        let event =
            pb::log::Log::UmaCtfAdapterQuestionInitialized(pb::UmaCtfAdapterQuestionInitialized {
                question_id: event.question_id.to_vec(),
                request_timestamp: event.request_timestamp.to_string(),
                creator: event.creator.to_vec(),
                ancillary_data: event.ancillary_data,
                reward_token: event.reward_token.to_vec(),
                reward: event.reward.to_string(),
                proposal_bond: event.proposal_bond.to_string(),
            });
        return Ok(Some(event));
    }

    // QuestionPaused event
    if let Some(event) = events::QuestionPaused::match_and_decode(log) {
        let event = pb::log::Log::UmaCtfAdapterQuestionPaused(pb::UmaCtfAdapterQuestionPaused {
            question_id: event.question_id.to_vec(),
        });
        return Ok(Some(event));
    }

    // QuestionReset event
    if let Some(event) = events::QuestionReset::match_and_decode(log) {
        let event = pb::log::Log::UmaCtfAdapterQuestionReset(pb::UmaCtfAdapterQuestionReset {
            question_id: event.question_id.to_vec(),
        });
        return Ok(Some(event));
    }

    // QuestionResolved event
    if let Some(event) = events::QuestionResolved::match_and_decode(log) {
        let event =
            pb::log::Log::UmaCtfAdapterQuestionResolved(pb::UmaCtfAdapterQuestionResolved {
                question_id: event.question_id.to_vec(),
                settled_price: event.settled_price.to_string(),
                payouts: event.payouts.iter().map(|p| p.to_string()).collect(),
            });
        return Ok(Some(event));
    }

    // QuestionUnpaused event
    if let Some(event) = events::QuestionUnpaused::match_and_decode(log) {
        let event =
            pb::log::Log::UmaCtfAdapterQuestionUnpaused(pb::UmaCtfAdapterQuestionUnpaused {
                question_id: event.question_id.to_vec(),
            });
        return Ok(Some(event));
    }

    // RemovedAdmin event
    if let Some(event) = events::RemovedAdmin::match_and_decode(log) {
        let event = pb::log::Log::UmaCtfAdapterRemovedAdmin(pb::UmaCtfAdapterRemovedAdmin {
            admin: event.admin.to_vec(),
            removed_admin: event.removed_admin.to_vec(),
        });
        return Ok(Some(event));
    }

    // QuestionUnflagged event (V3 only)
    if let Some(event) = events::QuestionUnflagged::match_and_decode(log) {
        let event =
            pb::log::Log::UmaCtfAdapterQuestionUnflagged(pb::UmaCtfAdapterQuestionUnflagged {
                question_id: event.question_id.to_vec(),
            });
        return Ok(Some(event));
    }

    Ok(None)
}
