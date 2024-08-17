extern crate serde;
extern crate serde_json;
use serde::Deserialize;
use std::hash::Hash;
use std::hash::Hasher;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Default, Eq)]
pub struct PlayerLoanInfo {
    pub loanType: String,
    pub loanValue: u64,
}
impl PartialEq for PlayerLoanInfo {
    fn eq(&self, other: &Self) -> bool {
        self.loanType == other.loanType && self.loanValue == other.loanValue
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Default, Eq)]
pub struct Player {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub timestamp: u64,
    #[serde(default)]
    pub formation: String,
    #[serde(default)]
    pub untradeable: bool,
    #[serde(default)]
    pub assetId: u64,
    #[serde(default)]
    pub rating: u8,
    #[serde(default)]
    pub itemType: String,
    #[serde(default)]
    pub resourceId: u64,
    #[serde(default)]
    pub owners: u16,
    #[serde(default)]
    pub discardValue: u64,
    #[serde(default)]
    pub itemState: String,
    #[serde(default)]
    pub cardsubtypeid: u16,
    #[serde(default)]
    pub lastSalePrice: u32,
    #[serde(default)]
    pub injuryType: String,
    #[serde(default)]
    pub injuryGames: u16,
    #[serde(default)]
    pub preferredPosition: String,
    #[serde(default)]
    pub contract: u16,
    #[serde(default)]
    pub teamid: u32,
    #[serde(default)]
    pub rareflag: u16,
    #[serde(default)]
    pub playStyle: u16,
    #[serde(default)]
    pub leagueId: u32,
    #[serde(default)]
    pub assists: u16,
    #[serde(default)]
    pub lifetimeAssists: u16,

    #[serde(default)]
    pub loans: u16,

    #[serde(default)]
    pub loansInfo: PlayerLoanInfo,

    #[serde(default)]
    pub loyaltyBonus: u64,
    #[serde(default)]
    pub pile: u16,
    #[serde(default)]
    pub nation: u32,
    #[serde(default)]
    pub marketDataMinPrice: u64,
    #[serde(default)]
    pub marketDataMaxPrice: u64,
    #[serde(default)]
    pub resourceGameYear: u16,

    #[serde(default)]
    pub guidAssetId: String,

    #[serde(default)]
    pub groups: Vec<u16>,

    #[serde(default)]
    pub attributeArray: Vec<u8>,
    #[serde(default)]
    pub statsArray: Vec<u16>,
    #[serde(default)]
    pub lifetimeStatsArray: Vec<u16>,
    #[serde(default)]
    pub skillmoves: u8,
    #[serde(default)]
    pub weakfootabilitytypecode: u8,
    #[serde(default)]
    pub attackingworkrate: u8,
    #[serde(default)]
    pub defensiveworkrate: u8,
    #[serde(default)]
    pub preferredfoot: u8,
    #[serde(default)]
    pub possiblePositions: Vec<String>,
    #[serde(default)]
    pub gender: u8,

    #[serde(default)]
    pub baseTraits: Vec<u8>,

    #[serde(default)]
    pub iconTraits: Vec<u8>,

    #[serde(default)]
    pub __fullname: String,

    #[serde(default)]
    pub __name: String,

    #[serde(default)]
    pub __nickname: String,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
