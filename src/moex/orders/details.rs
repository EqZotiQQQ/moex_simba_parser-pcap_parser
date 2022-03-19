pub mod details {
    use std::fmt::{Display, Formatter};
    use crate::errors::CustomErrors;

    #[derive(Debug, Clone, Copy)]
    pub enum MDUpdateAction {
        New,
        Change,
        Delete,
    }

    impl MDUpdateAction {
        pub fn new(action: u8) -> Result<MDUpdateAction, CustomErrors> {
            Ok(match action {
                0 => MDUpdateAction::New,
                1 => MDUpdateAction::Change,
                2 => MDUpdateAction::Delete,
                _ => return Err(CustomErrors::BadMDUpdateActionType)
            })
        }
    }

    impl Display for MDUpdateAction {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                MDUpdateAction::New => write!(f, "New"),
                MDUpdateAction::Change => write!(f, "Change"),
                MDUpdateAction::Delete => write!(f, "Delete"),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum MDEntryType {
        Bid,
        Ask,
        EmtpyBook
    }

    impl MDEntryType {
        pub fn new(action: u8) -> Result<MDEntryType, CustomErrors> {
            Ok(match action {
                48 => MDEntryType::Ask,
                49 => MDEntryType::Bid,
                74 => MDEntryType::EmtpyBook,
                _ => return Err(CustomErrors::BadMDEntryType)
            })
        }
    }

    impl Display for MDEntryType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                MDEntryType::Bid => write!(f, "Bid"),
                MDEntryType::Ask => write!(f, "Ask"),
                MDEntryType::EmtpyBook => write!(f, "EmtpyBook"),
            }
        }
    }
}
