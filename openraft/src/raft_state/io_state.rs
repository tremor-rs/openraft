use crate::display_ext::DisplayOption;
use crate::LeaderId;
use crate::LogId;
use crate::NodeId;

#[derive(Debug, Clone, Copy)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub(crate) struct LogIOId<NID: NodeId> {
    pub(crate) leader_id: LeaderId<NID>,
    pub(crate) log_id: Option<LogId<NID>>,
}

/// IOState tracks the state of actually happened io including log flushed, applying log to state
/// machine or snapshot building.
///
/// These states are updated only when the io complete and thus may fall behind to the state stored
/// in [`RaftState`](`openraft::RaftState`),.
#[derive(Debug, Clone, Copy)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub(crate) struct IOState<NID: NodeId> {
    /// Whether it is building a snapshot
    building_snapshot: bool,

    /// The last log id that has been flushed to storage.
    pub(crate) flushed: LogIOId<NID>,
    /// The last log id that has been applied to state machine.
    pub(crate) applied: Option<LogId<NID>>,
}

impl<NID: NodeId> IOState<NID> {
    pub(crate) fn new(flushed: LogIOId<NID>, applied: Option<LogId<NID>>) -> Self {
        Self {
            building_snapshot: false,
            flushed,
            applied,
        }
    }
    pub(crate) fn update_applied(&mut self, log_id: Option<LogId<NID>>) {
        tracing::debug!(applied = display(DisplayOption(&log_id)), "{}", func_name!());

        // TODO: should we update flushed if applied is newer?
        debug_assert!(
            log_id > self.applied,
            "applied log id should be monotonically increasing: current: {:?}, update: {:?}",
            self.applied,
            log_id
        );

        self.applied = log_id;
    }

    pub(crate) fn applied(&self) -> Option<&LogId<NID>> {
        self.applied.as_ref()
    }

    pub(crate) fn set_building_snapshot(&mut self, building: bool) {
        self.building_snapshot = building;
    }

    pub(crate) fn building_snapshot(&self) -> bool {
        self.building_snapshot
    }
}