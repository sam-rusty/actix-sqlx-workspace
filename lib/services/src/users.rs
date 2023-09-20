use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, Copy, Eq, PartialEq, Hash)]
#[sqlx(type_name = "user_type")]
pub enum UserType {
    Admin,
    Associate,
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "martial_status")]
pub enum MartialStatus {
    Married,
    Single,
    Divorced,
    Widow,
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

#[derive(Deserialize, Serialize, Clone, sqlx::Type, Debug, PartialEq)]
pub enum UserStatus {
    /// Default Level of every user
    Active,
    Resigned,
    Terminated,
    Inactive,
}

impl UserStatus {
    pub fn is_active(&self) -> bool {
        *self == Self::Active
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, PartialEq)]
pub enum State {
    // US States
    AL,
    AK,
    AZ,
    AR,
    CA,
    CO,
    CT,
    DE,
    DC,
    FL,
    GA,
    HI,
    ID,
    IL,
    IN,
    IA,
    KS,
    KY,
    LA,
    ME,
    MT,
    NE,
    NV,
    NH,
    NJ,
    NM,
    NY,
    NC,
    ND,
    OH,
    OK,
    OR,
    MD,
    MA,
    MI,
    MN,
    MS,
    MO,
    PA,
    RI,
    SC,
    SD,
    TN,
    TX,
    UT,
    VT,
    VA,
    WA,
    WV,
    WI,
    WY,

    // Canadian Provinces
    AB,
    BC,
    MB,
    NB,
    NL,
    NT,
    NS,
    NU,
    PE,
    ON,
    QC,
    SK,
    YT,
}
