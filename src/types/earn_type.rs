use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum EarnType {
    AllProgram = 0,
	UsdcProgram = 1,
	ElysProgram = 2,
	EdenProgram = 3,
	EdenBProgram = 4,
}