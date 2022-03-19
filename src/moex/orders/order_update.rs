use std::fmt::{Display, Formatter};
use crate::errors::CustomErrors;
use crate::Parser;

#[derive(Debug, Clone, Copy)]
pub struct EntryType(pub u64);

impl EntryType {
    const DAY: u8 = 0x1; //- Котировочная (Day)
    const IOC: u8 = 0x2; //- Встречная (IOC)
    const OTC: u8 = 0x4; //- Внесистемная заявка
    const END_OF_TRANSACTION: u16 = 0x1000;// - Признак последней записи в транзакции матчинга
    const FILL_OR_KILL: u32 = 0x80000;// - Заявка Fill-or-Kill
    const ORDER_MOVE_RESULT: u32 = 0x100000;//- Запись является результатом перемещения заявки
    const CANCEL_RESULT: u32 = 0x200000;//- Запись является результатом удаления заявки
    const MASS_CANCEL_RESULT: u32 = 0x400000;//- Запись является результатом группового удаления заявок
    const NEGOTIATED_ORDER: u32 = 0x4000000; // - Признак адресной заявки
    const MULTI_LEG_ORDER: u32 = 0x8000000; // - Признак заявки по связке
    const SIGN_OF_ORDER_DELETION_DUE_TO_A_CROSS_TRADE: u32 = 0x20000000; // - Признак удаления остатка заявки по причине кросс-сделки
    const CANCEL_OF_DISCONNECT_RESULT: u64= 0x100000000; // - Запись является результатом отмены заявок сервисом "Cancel on Disconnect"
    const SYNTHETIC_ORDER: u64= 0x200000000000; // - Признак синтетической заявки
    const RFS_ORDER: u64= 0x400000000000; // - Заявка из системы RFS
}

impl Display for EntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f);
        if (self.0 & EntryType::DAY as u64) == EntryType::DAY as u64 {
            writeln!(f, "* {:#02X} Day", EntryType::DAY);
        }
        if (self.0 & EntryType::IOC as u64) == EntryType::IOC as u64 {
            writeln!(f, "* {:#02X} IOC", EntryType::IOC);
        }
        if (self.0 & EntryType::OTC as u64) == EntryType::OTC as u64 {
            writeln!(f, "* {:#02X} OTC", EntryType::OTC);
        }
        if (self.0 & EntryType::END_OF_TRANSACTION as u64) == EntryType::END_OF_TRANSACTION as u64 {
            writeln!(f, "* {:#02X} End of transaction bit", EntryType::END_OF_TRANSACTION);
        }
        if (self.0 & EntryType::FILL_OR_KILL as u64) == EntryType::FILL_OR_KILL as u64 {
            writeln!(f, "* {:#02X} Fill-or-Kill", EntryType::FILL_OR_KILL);
        }
        if (self.0 & EntryType::ORDER_MOVE_RESULT as u64) == EntryType::ORDER_MOVE_RESULT as u64 {
            writeln!(f, "* {:#02X} The entry is the result of the order move", EntryType::ORDER_MOVE_RESULT);
        }
        if (self.0 & EntryType::CANCEL_RESULT as u64) == EntryType::CANCEL_RESULT as u64 {
            writeln!(f, "* {:#02X} The entry is the result of the order cancel", EntryType::CANCEL_RESULT);
        }
        if (self.0 & EntryType::MASS_CANCEL_RESULT as u64) == EntryType::MASS_CANCEL_RESULT as u64 {
            writeln!(f, "* {:#02X} The entry is the result of the orders mass cancel", EntryType::MASS_CANCEL_RESULT);
        }
        if (self.0 & EntryType::NEGOTIATED_ORDER as u64) == EntryType::NEGOTIATED_ORDER as u64 {
            writeln!(f, "* {:#02X} Negotiated order", EntryType::NEGOTIATED_ORDER);
        }
        if (self.0 & EntryType::MULTI_LEG_ORDER as u64) == EntryType::MULTI_LEG_ORDER as u64 {
            writeln!(f, "* {:#02X} Multi-leg order", EntryType::MULTI_LEG_ORDER);
        }
        if (self.0 & EntryType::SIGN_OF_ORDER_DELETION_DUE_TO_A_CROSS_TRADE as u64) == EntryType::SIGN_OF_ORDER_DELETION_DUE_TO_A_CROSS_TRADE as u64 {
            writeln!(f, "* {:#02X} Sign of order deletion due to a cross-trade", EntryType::SIGN_OF_ORDER_DELETION_DUE_TO_A_CROSS_TRADE);
        }
        if (self.0 & EntryType::CANCEL_OF_DISCONNECT_RESULT) == EntryType::CANCEL_OF_DISCONNECT_RESULT {
            writeln!(f, "* {:#02X} The entry is the result of the orders cancel by \"Cancel on Disconnect\" service", EntryType::CANCEL_OF_DISCONNECT_RESULT);
        }
        if (self.0 & EntryType::SYNTHETIC_ORDER) == EntryType::SYNTHETIC_ORDER {
            writeln!(f, "* {:#02X} Synthetic order", EntryType::SYNTHETIC_ORDER);
        }
        if (self.0 & EntryType::RFS_ORDER) == EntryType::RFS_ORDER {
            writeln!(f, "* {:#02X} RFS order", EntryType::RFS_ORDER);
        }
        write!(f, "")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OrderUpdate  {
    md_entry_id: i64,
    md_entry_px: i64,
    md_entry_size: i64,
    md_flags: EntryType,
    security_id: i32,
    rpt_seq: u32,
    md_update_action: MDUpdateAction,
    md_entry_type: MDEntryType,
}

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

#[allow(unused_must_use)]
impl Display for OrderUpdate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Order update ==");
        writeln!(f, "Order ID: {}", self.md_entry_id);
        writeln!(f, "Order price: {}", self.md_entry_px);
        writeln!(f, "Order volume: {}", self.md_entry_size);
        write!(f, "Order type (bit mask): {}", self.md_flags);
        writeln!(f, "Instrument numeric code: {}", self.security_id);
        writeln!(f, "Incremental refresh sequence number: {}", self.rpt_seq);
        writeln!(f, "Incremental refresh type: {}", self.md_update_action);
        writeln!(f, "Record type: {}", self.md_entry_type);
        write!(f, "== Order update end ==")
    }
}

impl OrderUpdate  {
    pub const SIZE: u8 = 42;
    pub fn parse(parser: &mut Parser) -> (OrderUpdate, u64)  {
        (OrderUpdate  {
            md_entry_id: parser.next::<i64>(),
            md_entry_px: parser.next::<i64>(),
            md_entry_size: parser.next::<i64>(),
            md_flags: EntryType(parser.next::<u64>()),
            security_id: parser.next::<i32>(),
            rpt_seq: parser.next::<u32>(),
            md_update_action: MDUpdateAction::new(parser.next::<u8>()).unwrap(),
            md_entry_type: MDEntryType::new(parser.next::<u8>()).unwrap(),
        }, OrderUpdate::SIZE as u64)
    }
}
